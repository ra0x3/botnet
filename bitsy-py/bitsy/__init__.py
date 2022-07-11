import os
import enum
import logging
import sys

from uvicorn import Config, Server
from loguru import logger
from dotenv import load_dotenv

from ._utils import env_var

dotenv_path = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    f"env/.env.{env_var('ENV')}",
)
load_dotenv(dotenv_path)


from ._web import app as _app
from ._graphql import graphql_app
from ._config import BitsyConfig
from ._t import *

config = BitsyConfig.from_default_manifest()


class _Environment(enum.Enum):
    Development = "dev"
    Production = "prod"


_LOG_LEVEL = logging.getLevelName(config.log_level)


# NOTE: Referencing https://pawamoy.github.io/posts/unify-logging-for-a-gunicorn-uvicorn-app/


class _InterceptHandler(logging.Handler):
    def emit(self, record):
        # Get corresponding Loguru level if it exists
        try:
            level = logger.level(record.levelname).name
        except ValueError:
            level = record.levelno

        # Find caller from where originated the logged message
        frame, depth = logging.currentframe(), 2
        while frame.f_code.co_filename == logging.__file__:
            frame = frame.f_back
            depth += 1

        logger.opt(depth=depth, exception=record.exc_info).log(
            level, record.getMessage()
        )


def _setup_logging():
    # intercept everything at the root logger
    logging.root.handlers = [_InterceptHandler()]
    logging.root.setLevel(_LOG_LEVEL)

    # remove every other logger's handlers
    # and propagate to root logger
    for name in logging.root.manager.loggerDict.keys():
        logging.getLogger(name).handlers = []
        logging.getLogger(name).propagate = True

    # configure loguru
    logger.configure(handlers=[{"sink": sys.stdout, "serialize": 0}])


_app.include_router(graphql_app, prefix="/graphql")


server = Server(
    Config(
        "bitsy:_app",
        host=config.api_host,
        port=config.api_port,
        log_level=_LOG_LEVEL,
        workers=config.workers,
        reload=config.env == _Environment.Development.value,
        use_colors=True,
        timeout_keep_alive=5,
        debug=config.env == _Environment.Development.value,
        env_file=dotenv_path,
    ),
)

_setup_logging()
