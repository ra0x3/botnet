import os

from ._web import app
from ._graphql import graphql_app
from ._db import Database
from ._config import BitsyConfig
from ._crypto import KeyStore
from ._t import *

from dotenv import load_dotenv

load_dotenv(
    os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))), ".env"
    )
)

config_path = os.environ["CONFIG"]
config = BitsyConfig.from_manifest(config_path)

keystore = KeyStore(config)

app.include_router(graphql_app, prefix="/graphql")


_ = Database(config)
