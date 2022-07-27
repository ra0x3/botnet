import operator
import logging

from ._models import *
from ._t import *
from ._config import config
from ._utils import *
from ._errors import *

logger = logging.getLogger("bitsy.db")


class _Database:
    def __init__(self):
        self._conn = config.connection
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

    def query(self, query: str) -> Any:
        cursor = self._conn.cursor()
        cursor.execute(query)
        return cursor.fetchone()

    def query_many(self, query: str) -> List[Any]:
        cursor = self._conn.cursor()
        cursor.execute(query)
        return [operator.itemgetter(0)(item) for item in cursor.fetchall()]

    def rollback(self):
        self._conn.rollback()

    def _bootstrap(self):
        from ._models import Model

        self._models = [
            ThirdParty,
            AccessToken,
            Account,
            Document,
            Permission,
            Setting,
            Webhook,
            ThirdPartyAccount,
        ]

        with self._conn.cursor() as _:
            self._models_map = dict(
                [(model.table.name, model) for model in self._models]
            )

            for table in self._models:
                table.create()

    def close_conn(self):
        self._conn.close()


database = _Database()
