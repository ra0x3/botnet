import os
import sys
import enum
import abc
import sqlite3

from ._t import *
from ._models import *
from ._config import BitsyConfig
from ._utils import *


class Database:
    def __init__(self, config: BitsyConfig):
        self.config = config
        self._conn: Optional[sqlite3.Connection] = None
        self._tables: List[Table] = []
        self._tables_map: Dict[str, Table] = {}
        self._init()

    def _init(self):
        self._conn = create_test_db(self.config.db_path)
        if self.config.bootstrap_db:
            self._bootstrap()

    def commit(self):
        self._conn.commit()

    def query(self, stmnt: str) -> Any:
        results = self._conn.execute(stmnt)
        return results

    def _bootstrap(self):
        tables = [
            Model.AccessToken.value,
            Model.ThirdParty.value,
            Model.Permission.value,
            Model.Account.value,
            Model.Document.value,
        ]

        self._tables = tables
        self._tables_map = dict([(table.name, table) for table in self._tables])

        for table in self._tables:
            table.create()
