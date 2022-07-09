import os
from dotenv import load_dotenv

load_dotenv(
    os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))), ".env.dev"
    )
)

from ._web import app
from ._graphql import graphql_app
from ._db import Database
from ._config import BitsyConfig
from ._t import *
from ._utils import env_var

config_path = env_var("CONFIG")
config = BitsyConfig.from_manifest(config_path)

app.include_router(graphql_app, prefix="/graphql")


_ = Database(config)
