import sqlite3

from ._t import *
from ._models import *
from ._config import BitsyConfig
from ._utils import *


class Database:
    def __init__(self, config: BitsyConfig):
        self.config = config
        self._conn: Optional[sqlite3.Connection] = None
        self._models: List[Model] = []
        self._models_map: Dict[str, Model] = {}
        self._init()

    def _init(self):
        self._conn = create_test_db(self.config.db_path)
        if self.config.bootstrap_db:
            self._bootstrap()

    def _bootstrap(self):
        tables = [
            AccessToken,
            ThirdParty,
            Permission,
            Account,
            Document,
        ]

        self._models = tables
        self._models_map = dict(
            [(model.table.name, model) for model in self._models]
        )

        for table in self._models:
            table.create()
