import re
import enum
import sqlite3
import pathlib
import os
import uuid
from blake3 import blake3

from ._t import *


def create_test_db(path: str) -> sqlite3.Connection:
    conn = sqlite3.connect(path)
    os.chmod(path, 0o777)
    return conn


def remove_test_db(path: str):
    path = pathlib.Path(path)
    if path.exists():
        path.unlink()


def blake(input: str) -> str:
    return blake3(input.encode()).digest()


def uuid4() -> str:
    return str(uuid.uuid4())


class ValueType(enum.Enum):
    Int = "int"
    Float = "float"
    String = "str"
    Bool = "bool"


def is_of_type(value: Any, ty: ValueType) -> bool:
    if ty == ValueType.Int:
        return isinstance(value, int)

    if ty == ValueType.Float:
        return isinstance(value, float)

    if ty == ValueType.String:
        return isinstance(value, str)

    if ty == ValueType.Bool:
        return isinstance(value, bool)

    raise NotImplementedError


def quote(s: str) -> str:
    return f"'{s}'"
