import sqlite3

from bitsy._utils import create_test_db
from bitsy._t import *
from bitsy._models import Model


def create_test_db_and_bootstrap_tables(
    path: str, models: List[Model]
) -> sqlite3.Connection:
    conn = create_test_db(path)
    for model in models:
        model.table.conn = conn
        model.create()
    return conn


# NOTE: This is a fake/test account - don't add anything more that $1 to it
class RealMetamaskAcct:
    password = "supersecretpassword12345*"
    address = "0x366f3b631cf475A13402b6A04cF226f86D7B7921"
    mnemnonic = "release cargo satoshi penalty security orphan silk input soul region prevent exist"
