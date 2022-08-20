import yaml
import enum
import argparse
import datetime as dt
import os
import json
import jwt

from ._t import *
from ._utils import *


class KeyStoreProvider(enum.Enum):
    InMemory = "in-memory"
    Vault = "vault"


class LogLevel(enum.Enum):
    INFO = "INFO"
    DEBUG = "DEBUG"
    WARN = "WARN"
    ERROR = "ERROR"


def derive_jwt(params: Dict[str, str]):
    expiry = dt.datetime.now() + dt.timedelta(days=int(BitsyConfig.jwt_expiry_days))
    params["exp"] = int(expiry.strftime("%s"))
    return jwt.encode(params, BitsyConfig.jwt_secret, algorithm=BitsyConfig.jwt_algo)


class Defaults:
    api_host: str = "127.0.0.1"
    api_port: int = 8000
    env: str = env_with_default()
    jwt_secret: str = (
        "77388623176251186746484687673521814216112263525741856083147157640482304445455"
    )
    log_file: str = "bitsy.log"
    log_level: LogLevel = LogLevel.DEBUG
    pg_database: str = env_var_with_default("PG_DATABASE", "bitsy")
    pg_host: str = env_var_with_default("PG_HOST", "127.0.0.1")
    pg_password: str = env_var_with_default("PG_PASSWORD", "")
    pg_port: str = env_var_with_default("PG_PORT", "5432")
    pg_user: str = env_var_with_default("PG_USER", "postgres")
    vault_address: str = "http://127.0.0.1:8200"
    workers: int = 3
    keystore_provider: KeyStoreProvider = KeyStoreProvider.InMemory.value


class BitsyConfig:
    api_host: str = Defaults.api_host
    api_port: int = Defaults.api_port
    env: str = Defaults.env
    jwt_algo: str = "HS256"
    jwt_expiry_days: str = "90"
    jwt_secret: str = env_var_with_default("JWT_SECRET", Defaults.jwt_secret)
    keystore_provider: KeyStoreProvider = KeyStoreProvider.InMemory.value
    log_file: str = Defaults.log_file
    log_level: LogLevel = LogLevel.DEBUG
    pg_database: str = Defaults.pg_database
    pg_host: str = Defaults.pg_host
    pg_password: str = Defaults.pg_password
    pg_port: str = Defaults.pg_port
    pg_user: str = Defaults.pg_user
    vault_address: str = Defaults.vault_address
    workers: int = Defaults.workers
    connection = create_postgres_conn(pg_database, pg_user, pg_password, pg_host, pg_port)

    def __init__(self, **kwargs):
        self.__dict__.update(kwargs)

    @staticmethod
    def from_manifest(path: str) -> "BitsyConfig":
        return BitsyConfig._load_config(path)

    @staticmethod
    def from_default_manifest_with_opts(opts: Dict[str, Any]) -> "BitsyConfig":
        config = BitsyConfig.from_default_manifest()
        config.__dict__.update(opts)
        config.reset_posgres_connection()
        return config

    @staticmethod
    def from_default_manifest() -> "BitsyConfig":
        path = os.path.join(
            os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
            "config",
            f"{BitsyConfig.env}.yaml",
        )
        return BitsyConfig._load_config(path)

    def reset_posgres_connection(self):
        self.connection = create_postgres_conn(
            self.pg_database,
            self.pg_user,
            self.pg_password,
            self.pg_host,
            self.pg_port,
        )

    def to_json(self) -> str:
        object = dict([(k, v) for k, v in self.__dict__.items() if k != "connection"])
        return json.dumps(object)

    @staticmethod
    def _load_config(path: str) -> "BitsyConfig":
        data = None
        with open(path, "r") as file:
            data = yaml.safe_load(file)

        return BitsyConfig(**data)

    def __str__(self) -> str:
        return json.dumps(self.__dict__)


def parse_args() -> Dict[str, Any]:
    if is_pytest_session():
        return {}

    parser = argparse.ArgumentParser(description="bitsy-py <3")
    parser.add_argument(
        "-e",
        "--env",
        type=str,
        default="dev",
        nargs="?",
        const=1,
        help="Environment",
    )
    parser.add_argument(
        "--api-host",
        type=str,
        default=Defaults.api_host,
        nargs="?",
        const=1,
        help="Web API host",
    )
    parser.add_argument(
        "--api-port",
        type=int,
        default=Defaults.api_port,
        nargs="?",
        const=1,
        help="Web API port",
    )
    parser.add_argument(
        "--pg-database",
        type=str,
        default=Defaults.pg_database,
        nargs="?",
        const=1,
        help="Postgres database",
    )
    parser.add_argument(
        "--pg-user",
        type=str,
        default=Defaults.pg_user,
        nargs="?",
        const=1,
        help="Postgres user",
    )
    parser.add_argument(
        "--pg-password",
        type=str,
        default=Defaults.pg_password,
        nargs="?",
        const=1,
        help="Postgres password",
    )
    parser.add_argument(
        "--pg-host",
        type=str,
        default=Defaults.pg_host,
        nargs="?",
        const=1,
        help="Postgres host",
    )
    parser.add_argument(
        "--pg-port",
        type=str,
        default=Defaults.pg_port,
        nargs="?",
        const=1,
        help="Postgres port",
    )
    parser.add_argument(
        "--workers",
        type=int,
        default=Defaults.workers,
        nargs="?",
        const=1,
        help="Number of workers",
    )
    parser.add_argument(
        "--vault-address",
        type=str,
        default=Defaults.vault_address,
        nargs="?",
        const=1,
        help="Vault address",
    )

    args = vars(parser.parse_args())

    def replace_empty_strings(d: Dict[str, Any]) -> Dict[str, Any]:
        for k, v in d.items():
            if v == "" or v == 1:  # FIXME
                d[k] = Defaults.__dict__[k]
        return d

    args = replace_empty_strings(args)

    return args


config = BitsyConfig.from_default_manifest_with_opts(parse_args())
