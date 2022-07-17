import yaml
import enum
import datetime as dt
import os
import json
import jwt

from ._t import *
from ._utils import *


class KeyStoreProvider(enum.Enum):
    InMemory = "in-memory"
    Vault = "vault"


class _LogLevel(enum.Enum):
    INFO = "INFO"
    DEBUG = "DEBUG"
    WARN = "WARN"
    ERROR = "ERROR"


def derive_jwt(params: Dict[str, str]):
    expiry = dt.datetime.now() + dt.timedelta(
        days=int(BitsyConfig.jwt_expiry_days)
    )
    params["exp"] = int(expiry.strftime("%s"))
    return jwt.encode(
        params, BitsyConfig.jwt_secret, algorithm=BitsyConfig.jwt_algo
    )


class BitsyConfig:

    jwt_secret: str = env_var("JWT_SECRET")

    jwt_algo: str = "HS256"
    jwt_expiry_days: str = "90"

    api_host: str = "127.0.0.1"

    api_port: int = 8000

    vault_address: str = "http://127.0.0.1:8200"

    pg_database: str = "bitsy"

    pg_user: str = "postgres"

    pg_password: str = ""

    pg_host: str = "127.0.0.1"

    pg_port: str = "5432"

    log_level: _LogLevel = _LogLevel.DEBUG

    workers: int = 3

    log_file: str = "bitsy.log"

    connection = create_postgres_conn(
        pg_database, pg_user, pg_password, pg_host, pg_port
    )

    jwt_secret: str = "foo"

    keystore_provider: KeyStoreProvider = KeyStoreProvider.Vault.value

    def __init__(self, **kwargs):
        self.__dict__.update(kwargs)

    @staticmethod
    def from_manifest(path: str) -> "BitsyConfig":
        return BitsyConfig._load_config(path)

    @staticmethod
    def from_default_manifest() -> "BitsyConfig":
        env = env_with_default()
        path = os.path.join(
            os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
            "config",
            f"bitsy.{env}.yaml",
        )
        return BitsyConfig._load_config(path)

    @staticmethod
    def _load_config(path: str) -> "BitsyConfig":
        data = None
        with open(path, "r") as file:
            data = yaml.safe_load(file)

        return BitsyConfig(**data)

    def __str__(self) -> str:
        return json.dumps(self.__dict__)
