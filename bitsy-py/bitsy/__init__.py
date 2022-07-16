import enum
import logging
import sys

from uvicorn import Config, Server
from loguru import logger

from ._utils import load_dot_env


class Environment(enum.Enum):
    Development = "dev"
    Production = "prod"


load_dot_env()

from ._web import app as _app
from ._graphql import graphql_app
from ._config import BitsyConfig
from ._t import *

config = BitsyConfig.from_default_manifest()


_LOG_LEVEL = logging.getLevelName(config.log_level)


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

    # remove every other logger's handlers and propagate to root logger
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
        reload=config.env == Environment.Development.value,
        use_colors=True,
        timeout_keep_alive=5,
        log_config={
            "version": 1,
            "disable_existing_loggers": False,
            "formatters": {
                "default": {
                    "()": "uvicorn.logging.DefaultFormatter",
                    "fmt": "%(levelprefix)s %(message)s",
                    "use_colors": None,
                },
                "access": {
                    "()": "uvicorn.logging.AccessFormatter",
                    "fmt": '%(levelprefix)s %(client_addr)s - "%(request_line)s" %(status_code)s',  # noqa: E501
                },
            },
            "handlers": {
                "default": {
                    "formatter": "default",
                    "class": "logging.StreamHandler",
                    "stream": "ext://sys.stderr",
                },
                "access": {
                    "formatter": "access",
                    "class": "logging.StreamHandler",
                    "stream": "ext://sys.stdout",
                },
            },
            "loggers": {
                "uvicorn": {"handlers": ["default"], "level": "INFO"},
                "uvicorn.error": {"level": "INFO"},
                "uvicorn.access": {
                    "handlers": ["access"],
                    "level": "INFO",
                    "propagate": False,
                },
            },
        },
        debug=config.env == Environment.Development.value,
        # env_file=dotenv_path,
    ),
)

_setup_logging()
