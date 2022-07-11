import abc
import enum
import time
import logging

from ._t import *
from ._utils import *
from ._config import BitsyConfig

_SQL_NULL = "null"


_logger = logging.getLogger("bitsy.models")


class _table_name(enum.Enum):
    access_tokens = "access_tokens"
    third_parties = "third_parties"
    permissions = "permissions"
    accounts = "accounts"
    documents = "documents"
    settings = "settings"


class SettingKey(enum.Enum):
    Other = "other"
    BitsyVaultDeletegation = "bitsy.vault.delegation"


class PermissionKey(enum.Enum):
    Other = "other"
    Read = "read"
    Write = "write"
    Delete = "delete"


class ColumnType(enum.Enum):
    Varchar = "varchar(255)"
    Decimal = "decimal"
    Integer = "integer"
    Bytea = "bytea"
    Null = "Null"
    ForeignKey = "ForeignKey"


class _DeleteAction(enum.Enum):
    CASCADE = "CASCADE"
    NO_ACTION = "NO ACTION"
    SET_NULL = "SET NULL"


class ForeignKeyReference:
    def __init__(self, table_name: str, column_name: str):
        self.table_name = table_name
        self.column_name = column_name

    def create_fragment(self) -> str:
        return f"{self.table_name}({self.column_name})"


class ForeignKey:
    def __init__(
        self,
        local_column_name: str,
        reference: ForeignKeyReference,
        on_delete: _DeleteAction = _DeleteAction.NO_ACTION,
    ):
        self.local_column_name = local_column_name
        self.reference = reference
        self.on_delete = on_delete

    def create_fragment(self) -> str:
        return f"  FOREIGN KEY({self.local_column_name}) REFERENCES {self.reference.create_fragment()} ON DELETE {self.on_delete.value}"


class Index:
    def __init__(
        self, name: str, unique: bool, table_name: str, column_name: "Column"
    ):
        self.name = name
        self.unique = unique
        self.table_name = table_name
        self.column_name = column_name

    def create_fragment(self) -> str:
        stmnt = "CREATE "
        if self.unique:
            stmnt += "UNIQUE "

        stmnt += f"INDEX IF NOT EXISTS {self.name} ON {self.table_name}({self.column_name})"
        return stmnt


class Column:
    def __init__(
        self,
        name: str,
        type: ColumnType,
        auto_increment: bool = False,
        primary_key: bool = False,
        unique: Optional[bool] = False,
        foreign_key: Optional[ForeignKey] = None,
        default: Optional[str] = None,
    ):
        self.name = name
        self.type = type
        self.auto_increment = auto_increment
        self.primary_key = primary_key
        self.unique = unique
        self.foreign_key = foreign_key
        self.default = default

    def create_fragment(self) -> str:
        frag = f"  {self.name} {self.type.value}"

        if self.foreign_key:
            assert not (
                self.unique
                and not self.primary_key
                and not self.auto_increment
                and not self.default
            ), "Cannot have a FK and other constraints"

        if self.unique:
            frag += " UNIQUE"

        if self.primary_key:
            frag += " PRIMARY KEY"

        if self.auto_increment:
            frag += " AUTOINCREMENT"

        if self.default:
            frag += f" DEFAULT {self.default}"

        frag += ",\n"

        if self.foreign_key:
            frag = frag + self.foreign_key.create_fragment() + ",\n"

        return frag


class Table:
    def __init__(
        self,
        name: str,
        columns: List[Column],
        conn: sqlite3.Connection,
    ):
        self.name = name
        self.columns = columns
        self.conn = conn

    def _create_stmnt(self) -> str:
        types = "".join([column.create_fragment() for column in self.columns])
        stmnt = f"CREATE TABLE IF NOT EXISTS {self.name} (\n{types});"
        stmnt = self.reorganize_foreign_keys(stmnt)

        # Remove final trailing comma
        idx = stmnt.rfind(",")
        stmnt = stmnt[:idx] + stmnt[idx + 1 :]
        return stmnt

    def create(self):
        _logger.debug("creating table(%s)", self.name)
        cursor = self.conn.cursor()
        stmnt = self._create_stmnt()
        cursor.execute(stmnt)

    def add_index(
        self, index_name: str, table_name: str, column_name: str, unique: bool
    ) -> "Table":
        index = Index(index_name, unique, table_name, column_name)
        cursor = self.conn.cursor()
        cursor.execute(index.create_fragment())
        self.conn.commit()
        return self

    def reorganize_foreign_keys(self, stmnt: str) -> str:
        lines = stmnt.split("\n")
        column_frags = []
        foreign_keys_frags = []
        terminator = ""

        for line in lines:
            if line.strip().startswith("FOREIGN KEY"):
                foreign_keys_frags.append(line)
            elif line == ");":
                terminator = line
            else:
                column_frags.append(line)

        # Last item in column_frags is ');'
        if foreign_keys_frags:
            column_frags.append("\n".join(foreign_keys_frags))

        column_frags.append(terminator)

        return "\n".join(column_frags)


class DocumentBlob:
    def __init__(self, data: str):
        self.data = data

    # FIXME: wtf is this shit?
    def decode(self) -> str:
        if isinstance(self.data, bytes):
            return decode(self.data, Encoding.UTF8)
        return self.data


class Wallet:
    pass


class ModelEntry:
    @abc.abstractmethod
    def to_row(self) -> Tuple[Any]:
        raise NotImplementedError

    @abc.abstractstaticmethod
    def from_row(row: Tuple[Any]):
        raise NotImplementedError


class BaseModel(ModelEntry):
    table: Table

    def save(self):
        columns_frag = ", ".join([column.name for column in self.table.columns])
        values_frag = ", ".join(self.to_row())
        stmnt = f"INSERT INTO {self.table.name} ({columns_frag}) VALUES ({values_frag});"

        cursor = self.table.conn.cursor()
        cursor.execute(stmnt)

    @classmethod
    def all(cls) -> List["BaseModel"]:
        stmnt = f"SELECT * FROM {cls.table.name};"
        cursor = cls.table.conn.cursor()
        cursor.execute(stmnt)
        results = cursor.fetchall()
        return [cls.from_row(result) for result in results]

    @classmethod
    def update(cls, update: Dict[str, Any], where: Dict[str, Any]) -> Any:
        parts = []
        for key, item in update.items():
            if isinstance(item, str):
                item = quote(item)
            parts.append(f"{key} = {item}")
        update_frag = ", ".join(parts)

        stmnt = f"UPDATE {cls.table.name} SET {update_frag}"

        if where:
            stmnt += " WHERE "
            parts = []
            for key, item in where.items():
                if isinstance(item, str):
                    item = quote(item)
                parts.append(f"{key} = {item}")
            stmnt += " AND ".join(parts)

        stmnt += " RETURNING *;"

        cursor = cls.table.conn.cursor()
        cursor.execute(stmnt)
        result = cursor.fetchone()

        return cls.from_row(result)

    @classmethod
    def get(cls, where: Dict[str, Any]) -> Any:
        stmnt = f"SELECT * FROM {cls.table.name} WHERE "
        parts = []
        for key, item in where.items():
            if isinstance(item, str):
                item = quote(item)
            parts.append(f"{key} = {item}")

        stmnt = stmnt + " AND ".join(parts) + ";"

        cursor = cls.table.conn.cursor()
        cursor.execute(stmnt)

        result = cursor.fetchone()
        return cls.from_row(result)

    @classmethod
    def get_many(cls, where: Dict[str, Any]) -> List[Any]:
        stmnt = f"SELECT * FROM {cls.table.name} WHERE "
        parts = []
        for key, item in where.items():
            if isinstance(item, str):
                item = quote(item)
            parts.append(f"{key} = {item}")

        stmnt = stmnt + " AND ".join(parts) + ";"

        cursor = cls.table.conn.cursor()
        cursor.execute(stmnt)

        results = cursor.fetchall()
        return [cls.from_row(result) for result in results]


class AccessToken(BaseModel):
    table = Table(
        _table_name.access_tokens.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column("expiry", ColumnType.Integer, default=-1),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(self, uuid: str = _SQL_NULL, expiry: int = -1):
        self.uuid = uuid
        self.expiry = expiry

    def from_row(row: Tuple[Any]) -> "AccessToken":
        (key, expiry) = row
        return AccessToken(key, expiry)

    def to_row(self) -> Tuple[Any]:
        return (quote(self.uuid), quote(self.expiry))

    @staticmethod
    def create():
        AccessToken.table.create()

    def is_null(self) -> bool:
        return self.uuid == _SQL_NULL or self.uuid is None

    def is_expired(self) -> bool:
        return self.expiry > int(time.time())


class ThirdParty(BaseModel):
    table = Table(
        _table_name.third_parties.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column(
                "access_token",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "access_token",
                    reference=ForeignKeyReference("access_tokens", "uuid"),
                ),
            ),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(self, uuid: str, access_token: Optional[AccessToken] = None):
        self.uuid = uuid
        self.access_token = access_token

    def to_row(self) -> Tuple[Any]:
        if self.access_token:
            return (quote(self.uuid), quote(self.access_token.uuid))
        return (quote(self.uuid), _SQL_NULL)

    def from_row(row: Tuple[Any]) -> "ThirdParty":
        row = tuple([item for item in row if item])
        if len(row) == 2:
            (uuid, access_token) = row
            return ThirdParty(uuid, AccessToken(access_token))
        (uuid,) = row
        return ThirdParty(uuid)

    def table_name(self) -> str:
        return _table_name.third_parties.value

    def __str__(self) -> str:
        return self.uuid

    @staticmethod
    def create():
        ThirdParty.table.create()


class Permission(BaseModel):
    name = _table_name.permissions.value
    table = Table(
        _table_name.permissions.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column("key", ColumnType.Varchar),
            Column(
                "document_cid",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "document_cid",
                    reference=ForeignKeyReference("documents", "cid"),
                ),
            ),
            Column("value", ColumnType.Integer, default=0),
            Column(
                "account_pubkey",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_pubkey",
                    reference=ForeignKeyReference("accounts", "pubkey"),
                ),
            ),
            Column(
                "third_party_id",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "third_party_id",
                    reference=ForeignKeyReference("third_parties", "uuid"),
                ),
            ),
            Column("ttl", ColumnType.Integer, default=-1),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(
        self,
        uuid: str,
        key: PermissionKey,
        document: "Document",
        value: int,
        account: "Account",
        third_party: ThirdParty,
        ttl: int,
    ):
        self.uuid = uuid
        self.key = key
        self.document = document
        self.value = value
        self.account = account
        self.third_party = third_party
        self.ttl = ttl

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.uuid),
            quote(self.key.value),
            quote(self.document.cid),
            str(self.value),
            quote(self.account.pubkey),
            quote(self.third_party.uuid),
            str(self.ttl),
        )

    def from_row(row: Tuple[Any]) -> "Permission":
        (
            uuid,
            key,
            document_cid,
            value,
            pubkey,
            third_party_id,
            ttl,
        ) = row
        document = Document.get({"cid": document_cid})
        account = Account.get({"pubkey": pubkey})
        party = ThirdParty.get({"uuid": third_party_id})
        return Permission(
            uuid,
            PermissionKey(key),
            document,
            value,
            account,
            party,
            ttl,
        )

    @staticmethod
    def create():
        Permission.table.create()


class Account(BaseModel):
    table = Table(
        _table_name.accounts.value,
        columns=[Column("pubkey", ColumnType.Varchar, unique=True)],
        conn=BitsyConfig.conn,
    )

    def __init__(self, pubkey: str):
        self.pubkey = pubkey

    def to_row(self) -> Tuple[Any]:
        return (quote(self.pubkey),)

    def from_row(row: Tuple[Any]) -> "Account":
        (pubkey,) = row
        return Account(pubkey)

    @staticmethod
    def create():
        Account.table.create()

    def create_settings(self):
        settings = [Setting(self.pubkey, SettingKey.BitsyVaultDeletegation, 1)]

        for setting in settings:
            if setting.key == SettingKey.BitsyVaultDeletegation:
                access_token = AccessToken(uuid4())
                access_token.save()
                setting.access_token = access_token
            setting.save()


class Document(BaseModel):
    table = Table(
        _table_name.documents.value,
        columns=[
            Column("cid", ColumnType.Varchar, unique=True),
            Column("blob", ColumnType.Bytea),
            Column(
                "account_pubkey",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_pubkey",
                    reference=ForeignKeyReference("accounts", "pubkey"),
                ),
            ),
            Column("key_image", ColumnType.Varchar),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(
        self,
        cid: str,
        blob: DocumentBlob,
        account: Account,
        key_img: str = _SQL_NULL,
    ):
        self.cid = cid
        self.blob = blob
        self.account = account
        self.key_img = key_img

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.cid),
            quote(self.blob.decode()),
            quote(self.account.pubkey),
            quote(self.key_img),
        )

    def from_row(row: Tuple[Any]) -> "Document":
        (cid, blob, pubkey, key_img) = row
        return Document(
            cid,
            DocumentBlob(decode(blob, Encoding.UTF8)),
            Account(pubkey),
            key_img,
        )

    @staticmethod
    def create():
        Document.table.create()

    def set_text(self, text: str):
        self.blob = DocumentBlob(text)


class Setting(BaseModel):
    table = Table(
        _table_name.settings.value,
        columns=[
            Column(
                "account_pubkey",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_pubkey",
                    reference=ForeignKeyReference("accounts", "pubkey"),
                ),
            ),
            Column("key", ColumnType.Varchar),
            Column("value", ColumnType.Integer),
            Column(
                "access_token",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "access_token",
                    reference=ForeignKeyReference("access_tokens", "uuid"),
                ),
            ),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(
        self,
        account_pubkey: str,
        key: SettingKey,
        value: int,
        access_token: Optional[AccessToken] = None,
    ):
        self.account_pubkey = account_pubkey
        self.key = key
        self.value = value
        self.access_token = access_token

    def to_row(self) -> Tuple[Any]:
        token = (
            quote(self.access_token.uuid)
            if self.access_token is not None
            else quote(None)
        )
        return (
            quote(self.account_pubkey),
            quote(self.key.value),
            quote(self.value),
            token,
        )

    def from_row(row: Tuple[Any]) -> "Setting":
        (account_pubkey, key, value, access_token) = row
        return Setting(
            account_pubkey, SettingKey(key), value, AccessToken(access_token)
        )

    @staticmethod
    def create():
        Setting.table.create()


class Model:
    AccessToken = AccessToken
    ThirdParty = ThirdParty
    Permission = Permission
    Account = Account
    Document = Document
    Setting = Setting
