import sqlite3
import pathlib
import os
import uuid
from blake3 import blake3

from ._t import *


def create_test_db(path: str) -> sqlite3.Connection:
    conn = sqlite3.connect(path, check_same_thread=False)
    os.chmod(path, 0o777)
    return conn


def remove_file(paths: List[str]):
    for p in paths:
        path = pathlib.Path(p)
        if path.exists():
            try:
                path.unlink()
            except PermissionError as err:
                print(str(err))


def blake3_sha256(input: str) -> str:
    digest: str = blake3(input.encode()).hexdigest()
    return digest


def uuid4() -> str:
    return str(uuid.uuid4())


def quote(s: str) -> str:
    return f"'{s}'"


def decode_utf8(s: bytes) -> str:
    return s.decode()


def encode_utf8(s: str) -> bytes:
    return s.encode()


def remove_empty_keys(d: Dict[str, Any]) -> Dict[str, Any]:
    return {key: value for key, value in d.items() if value}


def env_var(key: str) -> Any:
    return os.environ[key]
