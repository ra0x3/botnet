import yaml
import enum
import json
import sqlite3

from ._t import *
from ._utils import create_test_db


class KeyStoreProvider(enum.Enum):
    InMemory = "in-memory"
    Vault = "vault"


class LogLevel(enum.Enum):
    INFO = "INFO"
    DEBUG = "DEBUG"
    WARN = "WARN"
    ERROR = "ERROR"


class BitsyConfig:

    db_path: str = "bitsy.db"

    api_host: str = "127.0.0.1"

    api_port: int = 8000

    vault_address: str = "http://127.0.0.1:8200"

    bootstrap_db: bool = True

    log_level: LogLevel = LogLevel.DEBUG

    workers: int = 3

    log_file: str = "bitsy.log"

    conn: sqlite3.Connection = create_test_db(db_path)

    keystore_provider: KeyStoreProvider = KeyStoreProvider.Vault.value

    def __init__(self, **kwargs):
        self.__dict__.update(kwargs)

    @staticmethod
    def from_manifest(path: str) -> "BitsyConfig":
        data = None
        with open(path, "r") as file:
            data = yaml.safe_load(file)

        return BitsyConfig(**data)

    def __str__(self) -> str:
        return json.dumps(self.__dict__)
