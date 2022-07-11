import sqlite3

from ._t import *
from ._models import *
from ._config import BitsyConfig
from ._utils import *
from ._errors import *

_logger = logging.getLogger("bitsy.db")


class _Database:
    def __init__(self):
        self.config: BitsyConfig = BitsyConfig.from_default_manifest()
        self._conn = self.config.conn
        self._models: List[Model] = []
        self._models_map: Dict[str, Model] = {}
        self._init()

    def _init(self):
        self._bootstrap()

    def commit(self):
        try:
            self._conn.commit()
        except Exception as err:
            raise DatabaseError("error committing transaction: %s", str(err))

    def _bootstrap(self):
        self._models = [
            AccessToken,
            ThirdParty,
            Account,
            Document,
            Permission,
            Setting,
        ]

        with self._conn.cursor() as cursor:
            self._models_map = dict(
                [(model.table.name, model) for model in self._models]
            )

            for table in self._models:
                table.create()

        # self.commit()

    def close_conn(self):
        self._conn.close()


database = _Database()
