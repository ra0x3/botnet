import yaml
import sqlite3

from ._t import *
from ._utils import create_test_db


class BitsyConfig:

    db_path: str = "bitsy.db"

    bootstrap_db: bool = True

    conn: sqlite3.Connection = create_test_db(db_path)

    keystore_provider: str = "in-memory"

    def __init__(self, **kwargs):
        self.__dict__.update(kwargs)

    @staticmethod
    def from_manifest(path: str) -> "BitsyConfig":
        data = None
        with open(path, "r") as file:
            data = yaml.safe_load(file)

        return BitsyConfig(**data)
