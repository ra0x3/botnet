import os
import enum
import logging
import sys

from gunicorn.app.base import BaseApplication
from gunicorn.glogging import Logger
from uvicorn import Config, Server
from loguru import logger
from dotenv import load_dotenv

from ._utils import env_var

dotenv_path = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    env_var("DOTENV"),
)
load_dotenv(dotenv_path)


from ._web import app as _app
from ._graphql import graphql_app
from ._db import Database
from ._config import BitsyConfig
from ._t import *

config_path = env_var("CONFIG")
config = BitsyConfig.from_manifest(config_path)


class Environment(enum.Enum):
    Development = "dev"
    Production = "prod"


LOG_LEVEL = logging.getLevelName(config.log_level)


# NOTE: Referencing https://pawamoy.github.io/posts/unify-logging-for-a-gunicorn-uvicorn-app/


class InterceptHandler(logging.Handler):
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


def setup_logging():
    # intercept everything at the root logger
    logging.root.handlers = [InterceptHandler()]
    logging.root.setLevel(LOG_LEVEL)

    # remove every other logger's handlers
    # and propagate to root logger
    for name in logging.root.manager.loggerDict.keys():
        logging.getLogger(name).handlers = []
        logging.getLogger(name).propagate = True

    # configure loguru
    logger.configure(handlers=[{"sink": sys.stdout, "serialize": 0}])


_ = Database(config)

_app.include_router(graphql_app, prefix="/graphql")


server = Server(
    Config(
        "bitsy:_app",
        host=config.api_host,
        port=config.api_port,
        log_level=LOG_LEVEL,
        workers=config.workers,
        reload=config.env == Environment.Development.value,
        use_colors=True,
        timeout_keep_alive=5,
        debug=config.env == Environment.Development.value,
        env_file=dotenv_path,
    ),
)

setup_logging()
