import abc
import enum
import time
import web3
import json
import sys

from ._t import *
from ._utils import *
from ._config import BitsyConfig


class table_name(enum.Enum):
    access_tokens = "access_tokens"
    third_parties = "third_parties"
    permissions = "permissions"
    accounts = "accounts"
    documents = "documents"


class PermKey(enum.Enum):
    Other = "other.misc_other"


class ColumnType(enum.Enum):
    Text = "Text"
    Real = "Real"
    Integer = "Integer"
    Blob = "Blob"
    Null = "Null"
    ForeignKey = "ForeignKey"


class DeleteAction(enum.Enum):
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
        on_delete: DeleteAction = DeleteAction.NO_ACTION,
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
        self._last: bool = False

    def create_fragment(self) -> str:
        frag = f"  {self.name} {self.type.name}"

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

        if not self.last or self.foreign_key:
            frag += ",\n"

        if self.foreign_key:
            frag += self.foreign_key.create_fragment()
            if not self.last:
                frag += ",\n"

        return frag

    @property
    def last(self) -> bool:
        return self._last

    def set_last(self):
        self._last = True


class Table:
    def __init__(
        self,
        name: str,
        columns: List[Column],
        conn: sqlite3.Connection,
    ):
        self.name = name
        self.columns = columns
        self.columns[-1].set_last()
        self.conn = conn

    def _create_stmnt(self) -> str:
        types = "".join([column.create_fragment() for column in self.columns])
        stmnt = f"CREATE TABLE IF NOT EXISTS {self.name} (\n{types}\n);"
        stmnt = self.reorganize_foreign_keys(stmnt)

        return stmnt

    def create(self):
        cursor = self.conn.cursor()
        stmnt = self._create_stmnt()
        cursor.execute(stmnt)

    def add_index(
        self, index_name: str, table_name: str, column_name: str, unique: bool
    ) -> "Table":
        index = Index(index_name, unique, table_name, column_name)
        cursor = self.conn.cursor()
        cursor.execute(index.create_fragment())

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


#########


class DocumentBlob:
    def __init__(self, data: bytes):
        self.data = data


class Wallet:
    def __init__(self):
        pass

    def _load_private_key(self, path: str, password: str):
        with open(path) as keyfile:
            encrypted_key = keyfile.read()
            private_key = web3.eth.account.decrypt(encrypted_key, password)


################


class BaseModel:
    table: Table

    @abc.abstractstaticmethod
    def from_row(row: Tuple[Any]):
        raise NotImplementedError

    def to_row(self) -> Tuple[Any]:
        values = []
        for value in self.__dict__.values():
            if is_of_type(value, ValueType.String):
                value = quote(value)
            values.append(value)
        return tuple(values)

    def save(self):
        columns_frag = ", ".join([column.name for column in self.table.columns])
        values_frag = ", ".join(self.to_row())
        stmnt = f"INSERT INTO {self.table.name} ({columns_frag}) VALUES ({values_frag});"
        cursor = self.table.conn.cursor()
        cursor.execute(stmnt)
        self.table.conn.commit()

    def get(self, clause: Dict[str, Any]):
        stmnt = f"SELECT * FROM {self.name} WHER"
        for key, item in clause.items():
            pass


class AccessToken(BaseModel):
    table = Table(
        table_name.access_tokens.value,
        columns=[
            Column("uuid", ColumnType.Text, unique=True),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(self, uuid: str):
        self.uuid = uuid

    def from_row(row: Tuple[Any]) -> "AccessToken":
        (key,) = row
        return AccessToken(key)

    def to_row(self) -> Tuple[Any]:
        values = []
        for value in self.__dict__.values():
            if is_of_type(value, ValueType.String):
                value = quote(value)

            values.append(value)
        return tuple(values)

    @staticmethod
    def create():
        AccessToken.table.create()


class ThirdParty(BaseModel):
    table = Table(
        table_name.third_parties.value,
        columns=[
            Column("uuid", ColumnType.Text, unique=True),
            Column(
                "access_token",
                ColumnType.Text,
                foreign_key=ForeignKey(
                    "access_token",
                    reference=ForeignKeyReference("access_tokens", "uuid"),
                ),
            ),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(self, uuid: str, access_token: str):
        self.uuid = uuid
        self.access_token = access_token

    def from_row(row: Tuple[Any]) -> "ThirdParty":
        (uuid, access_token) = row
        token = AccessToken.from_row((access_token,))
        return ThirdParty(uuid, token)

    def table_name(self) -> str:
        return table_name.third_parties.value

    def __str__(self) -> str:
        return self.uuid

    @staticmethod
    def create():
        ThirdParty.table.create()


class Permission(BaseModel):
    name = table_name.permissions.value
    table = Table(
        table_name.permissions.value,
        columns=[
            Column("uuid", ColumnType.Text, unique=True),
            Column("key", ColumnType.Text),
            Column("value", ColumnType.Integer, default=0),
            Column(
                "account_address",
                ColumnType.Integer,
                foreign_key=ForeignKey(
                    "account_address",
                    reference=ForeignKeyReference("accounts", "address"),
                ),
            ),
            Column(
                "third_party_id",
                ColumnType.Text,
                foreign_key=ForeignKey(
                    "third_party_id",
                    reference=ForeignKeyReference("third_parties", "uuid"),
                ),
            ),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(
        self,
        uuid: str,
        key: str,
        value: int,
        account_id: str,
        third_party_id: str,
    ):
        self.uuid = uuid
        self.key = key
        self.value = value
        self.account_id = account_id
        self.third_party_id = third_party_id

    def from_row(row: Tuple[Any]) -> "Permission":
        (uuid, key, value, account_id, third_party_id) = row
        return Permission(uuid, key, value, account_id, third_party_id)

    @staticmethod
    def create():
        Permission.table.create()


class Account(BaseModel):
    table = Table(
        table_name.accounts.value,
        columns=[Column("address", ColumnType.Text, unique=True)],
        conn=BitsyConfig.conn,
    )

    def __init__(self, address: str):
        self.address = address

    def from_row(row: Tuple[Any]) -> "Account":
        (address,) = row
        return Account(address)

    @staticmethod
    def create():
        Account.table.create()


class Document(BaseModel):
    table = Table(
        table_name.documents.value,
        columns=[
            Column("cid", ColumnType.Text, unique=True),
            Column("metadata", ColumnType.Text),
            Column(
                "account_address",
                ColumnType.Integer,
                foreign_key=ForeignKey(
                    "account_address",
                    reference=ForeignKeyReference("accounts", "address"),
                ),
            ),
        ],
        conn=BitsyConfig.conn,
    )

    def __init__(self, cid: str, blob: bytes, account_id: str):
        self.cid = cid
        self.blob = DocumentBlob(blob)
        self.account_id = account_id

    def from_row(row: Tuple[Any]) -> "Document":
        (cid, blob, account_id) = row
        return Document(cid, blob, account_id)

    @staticmethod
    def create():
        Document.table.create()


class Model(enum.Enum):
    AccessToken = AccessToken
    ThirdParty = ThirdParty
    Permission = Permission
    Account = Account
    Document = Document
