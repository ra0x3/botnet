import yaml
import enum
import os
import json

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


class BitsyConfig:

    db_path: str = "bitsy.db"

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

    conn = create_postgres_conn(
        pg_database, pg_user, pg_password, pg_host, pg_port
    )

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
