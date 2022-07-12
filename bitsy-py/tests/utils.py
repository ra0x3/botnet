import sqlite3

from bitsy._utils import create_test_db
from bitsy._t import *
from bitsy._models import Model


def double_quote(s: str) -> str:
    return f'"{s}"'


# NOTE: This is a fake/test account - don't add anything more that $1 to it
class RealMetamaskAcct:
    password = "supersecretpassword12345*"
    address = "0x366f3b631cf475A13402b6A04cF226f86D7B7921"
    mnemnonic = "release cargo satoshi penalty security orphan silk input soul region prevent exist"
