import sqlite3
import pathlib
import os
import time
import enum
import uuid
import codecs
import binascii
import psycopg2
from dotenv import load_dotenv
from black import Encoding
from blake3 import blake3
from ._const import SQL_NULL

from ._t import *


def is_nullish(value: Any) -> bool:
    return value == SQL_NULL or value == "" or value is None


class Environment(enum.Enum):
    Development = "dev"
    Production = "prod"


def env_with_default() -> Environment:
    return (
        Environment.Development.value
        if "ENV" not in os.environ
        else env_var("ENV")
    )


def load_dot_env():
    pwd = os.environ["PWD"]
    env_name = env_with_default()
    env_path = os.path.join(pwd, "env", f".env.{env_name}")
    load_dotenv(env_path)


class defaults:
    access_token_ttl: int = 60 * 60 * 24


def create_test_db(path: str) -> sqlite3.Connection:
    conn = sqlite3.connect(path, check_same_thread=False, timeout=5)
    # curr = conn.execute("PRAGMA locking_mode = EXCLUSIVE")
    os.chmod(path, 0o777)
    return conn


def create_postgres_conn(
    database: str, user: str, password: str, host: str, port: str
):
    return psycopg2.connect(
        database=database, user=user, password=password, host=host, port=port
    )


def remove_file(paths: List[str]):
    for p in paths:
        path = pathlib.Path(p)
        if path.exists():
            try:
                path.unlink()
            except PermissionError as err:
                print(str(err))


# FIXME: Obviously this is a no-no
def blake3_sha256(input: Union[str, bytes]) -> str:
    digest: str
    if isinstance(input, str):
        digest: str = blake3(input.encode()).hexdigest()
    else:
        digest = blake3(input).hexdigest()
    return digest


def uuid4() -> str:
    return str(uuid.uuid4())


def quote(s: Optional[str]) -> Optional[str]:
    return f"'{s}'" if s is not None else "''"


class Encoding(enum.Enum):
    UTF8 = "utf-8"
    HEX = "hex"
    ASCII = "ascii"


# FIXME: This is a no-no
def decode(s: bytes, encoding: Encoding) -> str:
    if isinstance(s, str):
        return s
    return codecs.decode(s, encoding.value)


# FIXME: This is a no-no
def encode(s: Union[str, bytes], encoding: Encoding) -> bytes:
    if isinstance(s, bytes):
        return s
    return codecs.encode(s, encoding.value)


def hexlify(b: bytes) -> str:
    return decode(binascii.hexlify(b), Encoding.UTF8)


def unhexlify(s: str) -> bytes:
    return binascii.unhexlify(encode(s, Encoding.UTF8))


def remove_empty_keys(d: Dict[str, Any]) -> Dict[str, Any]:
    return {key: value for key, value in d.items() if value}


def env_var(key: str) -> Any:
    return os.environ[key]


def now() -> int:
    return int(time.time())


codec_list = [
    "ascii",
    "big5",
    "big5hkscs",
    "cp037",
    "cp1006",
    "cp1026",
    "cp1125",
    "cp1140",
    "cp1250",
    "cp1251",
    "cp1252",
    "cp1253",
    "cp1254",
    "cp1255",
    "cp1256",
    "cp1257",
    "cp1258",
    "cp273",
    "cp424",
    "cp437",
    "cp500",
    "cp720",
    "cp737",
    "cp775",
    "cp850",
    "cp852",
    "cp855",
    "cp856",
    "cp857",
    "cp858",
    "cp860",
    "cp861",
    "cp862",
    "cp863",
    "cp864",
    "cp865",
    "cp866",
    "cp869",
    "cp874",
    "cp875",
    "cp932",
    "cp949",
    "cp950",
    "euc-jis-2004",
    "euc-jisx0213",
    "euc-jp",
    "euc-kr",
    "gb18030",
    "gb2312",
    "gbk",
    "hz",
    "iso2022-jp-1",
    "iso2022-jp-2",
    "iso2022-jp-2004",
    "iso2022-jp-3",
    "iso2022-jp-ext",
    "iso2022-jp",
    "iso2022-kr",
    "iso8859-10",
    "iso8859-11",
    "iso8859-13",
    "iso8859-14",
    "iso8859-15",
    "iso8859-16",
    "iso8859-2",
    "iso8859-3",
    "iso8859-4",
    "iso8859-5",
    "iso8859-6",
    "iso8859-7",
    "iso8859-8",
    "iso8859-9",
    "johab",
    "koi8-r",
    "koi8-t",
    "koi8-u",
    "kz1048",
    "latin-1",
    "mac-cyrillic",
    "mac-greek",
    "mac-iceland",
    "mac-latin2",
    "mac-roman",
    "mac-turkish",
    "ptcp154",
    "shift-jis-2004",
    "shift-jis",
    "shift-jisx0213",
    "utf-16-be",
    "utf-16-le",
    "utf-16",
    "utf-32-be",
    "utf-32-le",
    "utf-32",
    "utf-7",
    "utf-8-sig",
    "utf-8",
]
