import sqlite3

from bitsy._utils import create_test_db
from bitsy._t import *
from bitsy._models import Model


def create_test_db_and_bootstrap_tables(path: str, models: List[Model]) -> sqlite3.Connection:
    conn = create_test_db(path)
    for model in models:
        model.value.table.conn = conn
        model.value.create()
    return conn
