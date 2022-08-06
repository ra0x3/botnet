import abc
import enum
import time
import datetime as dt
import logging

from ._t import *
from ._utils import *
from ._const import SQL_NULL
from ._errors import *
from ._crypto import fernet_bundle, FernetBundle
from ._config import config


logger = logging.getLogger("bitsy.models")


class _table_name(enum.Enum):
    access_tokens = "access_tokens"
    third_parties = "third_parties"
    permissions = "permissions"
    accounts = "accounts"
    documents = "documents"
    settings = "settings"
    webhooks = "webhooks"
    third_party_accounts = "third_party_accounts"
    access_requests = "access_requests"


class SettingKey(enum.Enum):
    Other = "Other"
    BitsyVaultDeletegation = "BitsyVaultDeletegation"
    ProgrammaticThirdPartyAccess = "ProgrammaticThirdPartyAccess"
    ThirdPartyNotifications = "ThirdPartyNotifications"
    Webhooks = "Webhooks"
    ProgrammaticWebhookAccess = "ProgrammaticWebhookAccess"


class PermissionKey(enum.Enum):
    Other = "Other"
    Read = "Read"
    Write = "Write"
    Delete = "Delete"


class ColumnType(enum.Enum):
    Varchar = "varchar(255)"
    Decimal = "decimal"
    Integer = "integer"
    Bytea = "bytea"
    Null = "Null"
    ForeignKey = "ForeignKey"
    Serial = "serial"


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
        serial: bool = False,
        primary_key: bool = False,
        unique: Optional[bool] = False,
        foreign_key: Optional[ForeignKey] = None,
        default: Optional[str] = None,
        null: Optional[bool] = None,
    ):
        self.name = name
        self.type = type
        self.serial = serial
        self.primary_key = primary_key
        self.unique = unique
        self.foreign_key = foreign_key
        self.default = default
        self.null = null

    def create_fragment(self) -> str:
        frag = f"  {self.name} {self.type.value}"

        if self.foreign_key:
            assert not (
                self.unique
                and not self.primary_key
                and not self.serial
                and not self.default
            ), "Cannot have a FK and other constraints"

        if self.unique:
            frag += " UNIQUE"

        if self.primary_key:
            frag += " PRIMARY KEY"

        if self.default:
            frag += f" DEFAULT {self.default}"

        if self.null:
            frag += " "

        frag += ",\n"

        if self.foreign_key:
            frag = frag + self.foreign_key.create_fragment() + ",\n"

        return frag


class Table:
    def __init__(
        self,
        name: str,
        columns: List[Column],
        conn: Any,
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
        logger.debug("creating table(%s)", self.name)
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
    def get(
        cls,
        where: Dict[str, Any],
        fail_if_not_found: bool = False,
        return_null: bool = True,
    ) -> Any:

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

        if not result and fail_if_not_found:
            raise ResourceDoesNotExist(
                "No resource found for clause: {}".format(where)
            )

        if not result and return_null:
            return None

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

    @classmethod
    def delete(cls, where: Dict[str, Any]):
        stmnt = f"DELETE FROM {cls.table.name} WHERE "
        parts = []
        for key, item in where.items():
            if isinstance(item, str):
                item = quote(item)
            parts.append(f"{key} = {item}")

        stmnt = stmnt + " AND ".join(parts) + ";"
        cursor = cls.table.conn.cursor()
        cursor.execute(stmnt)


class ThirdParty(BaseModel):
    table = Table(
        _table_name.third_parties.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column("name", ColumnType.Varchar),
        ],
        conn=config.connection,
    )

    def __init__(self, uuid: str, name: Optional[str] = None):
        self.uuid = uuid
        self.name = name or ThirdParty.default_name()

    @staticmethod
    def default_name() -> str:
        return f"unnamed-party-{dt.datetime.now().strftime('%Y-%m-%d')}"

    def to_row(self) -> Tuple[Any]:
        return (quote(self.uuid), quote(self.name))

    def from_row(row: Tuple[Any]) -> "ThirdParty":
        (uuid, name) = row
        return ThirdParty(uuid, name)

    def table_name(self) -> str:
        return _table_name.third_parties.value

    def __str__(self) -> str:
        return self.uuid

    @staticmethod
    def create():
        ThirdParty.table.create()


class AccessToken(BaseModel):
    table = Table(
        _table_name.access_tokens.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column(
                "third_party_id",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "third_party_id",
                    reference=ForeignKeyReference("third_parties", "uuid"),
                ),
            ),
            Column("name", ColumnType.Varchar),
            Column("expiry", ColumnType.Integer, default=-1),
            Column("active", ColumnType.Integer, default=0),
        ],
        conn=config.connection,
    )

    def __init__(
        self,
        uuid: str,
        third_party: ThirdParty,
        name: Optional[str] = None,
        expiry: Optional[int] = None,
        active: Optional[int] = 0,
    ):
        self.uuid = uuid
        self.third_party = third_party
        self.name = name or AccessToken.default_name()
        self.expiry = expiry or AccessToken.default_ttl()
        self.active = active

    @staticmethod
    def default_ttl() -> int:
        return int(time.time()) + 60 * 60 * 24

    @staticmethod
    def default_name() -> str:
        return f"unnamed-token-{dt.datetime.now().strftime('%Y-%m-%d')}"

    def from_row(row: Tuple[Any]) -> "AccessToken":
        (key, third_party_id, name, expiry, active) = row
        party = ThirdParty.get(where={"uuid": third_party_id})
        return AccessToken(key, party, name, expiry, active)

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.uuid),
            quote(self.third_party.uuid),
            quote(self.name),
            quote(self.expiry),
            quote(self.active),
        )

    @staticmethod
    def create():
        AccessToken.table.create()

    def is_null(self) -> bool:
        return self.uuid == SQL_NULL or self.uuid is None

    def is_expired(self) -> bool:
        return self.expiry > int(time.time())

    def toggle(self):
        self.active = 1 if self.active == 0 else 0
        AccessToken.update(
            update={"active": self.active}, where={"uuid": self.uuid}
        )


class Account(BaseModel):
    table = Table(
        _table_name.accounts.value,
        columns=[
            Column("address", ColumnType.Varchar, unique=True),
            Column("password_hash", ColumnType.Varchar),
            Column("created_at", ColumnType.Integer),
            Column("nonce", ColumnType.Varchar, unique=True),
            Column("pubkey", ColumnType.Varchar, unique=True),
        ],
        conn=config.connection,
    )

    def __init__(
        self,
        address: str,
        password_hash: str,
        created_at: int = now(),
        nonce: Optional[str] = None,
        pubkey: Optional[str] = None,
    ):
        self.address = address
        self.password_hash = password_hash
        self.created_at = created_at
        self.jwt: Optional[str] = None
        self.pubkey = pubkey
        self.nonce = nonce

    def set_jwt(self, jwt: str):
        self.jwt = jwt

    def to_row(self) -> Tuple[Any]:
        nonce = quote(self.nonce) if self.nonce is not None else SQL_NULL
        pubkey = quote(self.pubkey) if self.pubkey is not None else SQL_NULL
        return (
            quote(self.address),
            quote(self.password_hash),
            quote(self.created_at),
            nonce,
            pubkey,
        )

    def from_row(row: Tuple[Any]) -> "Account":
        (address, password_hash, created_at, nonce, pubkey) = row
        return Account(
            address=address,
            password_hash=password_hash,
            created_at=created_at,
            nonce=nonce,
            pubkey=pubkey,
        )

    @staticmethod
    def create():
        Account.table.create()

    def create_party_settings(self):
        settings = [
            Setting(self, SettingKey.Webhooks, 1),
            Setting(self, SettingKey.ProgrammaticWebhookAccess, 1),
            Setting(self, SettingKey.BitsyVaultDeletegation, 1),
        ]

        for setting in settings:
            setting.save()

    def create_account_settings(self):
        settings = [
            Setting(self, SettingKey.BitsyVaultDeletegation, 1),
            Setting(self, SettingKey.ProgrammaticThirdPartyAccess, 0),
            Setting(self, SettingKey.ThirdPartyNotifications, 1),
        ]

        for setting in settings:
            setting.save()


class AccountStat:
    def __init__(self, account_age: int, perm_count: int):
        self.account_age = account_age
        self.perm_count = perm_count

    def from_row(row: Tuple[Any]) -> "AccountStat":
        (account_age, perm_count) = row
        return AccountStat(account_age, perm_count)


class ThirdPartyAccount(BaseModel):
    table = Table(
        _table_name.third_party_accounts.value,
        columns=[
            Column(
                "third_party_id",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "third_party_id",
                    reference=ForeignKeyReference("third_parties", "uuid"),
                ),
            ),
            Column(
                "account_address",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_address",
                    reference=ForeignKeyReference("accounts", "address"),
                ),
            ),
        ],
        conn=config.connection,
    )

    def __init__(self, party: ThirdParty, account: Account):
        self.party = party
        self.account = account

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.party.uuid),
            quote(self.account.address),
        )

    def from_row(row: Tuple[Any]) -> "ThirdPartyAccount":
        (third_party_id, account_address) = row
        party = ThirdParty.get(where={"uuid": third_party_id})
        account = Account.get(where={"address": account_address})
        return ThirdPartyAccount(party, account)

    def table_name(self) -> str:
        return _table_name.third_party_accounts.value

    @staticmethod
    def create():
        ThirdPartyAccount.table.create()


class Permission(BaseModel):
    name = _table_name.permissions.value
    table = Table(
        _table_name.permissions.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column("key", ColumnType.Varchar),
            # TODO: Possibly store multiple document IDs in single perm
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
                "account_address",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_address",
                    reference=ForeignKeyReference("accounts", "address"),
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
            Column("created_at", ColumnType.Integer),
        ],
        conn=config.connection,
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
        created_at: int = now(),
    ):
        self.uuid = uuid
        self.key = key
        self.document = document
        self.value = value
        self.account = account
        self.third_party = third_party
        self.ttl = ttl
        self.created_at = created_at

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.uuid),
            quote(self.key.value),
            quote(self.document.cid),
            quote(self.value),
            quote(self.account.address),
            quote(self.third_party.uuid),
            quote(self.ttl),
            quote(self.created_at),
        )

    def from_row(row: Tuple[Any]) -> "Permission":
        (
            uuid,
            key,
            document_cid,
            value,
            account_address,
            third_party_id,
            ttl,
            created_at,
        ) = row
        document = Document.get({"cid": document_cid})
        account = Account.get({"address": account_address})
        party = ThirdParty.get({"uuid": third_party_id})
        return Permission(
            uuid,
            PermissionKey(key),
            document,
            value,
            account,
            party,
            ttl,
            created_at,
        )

    @staticmethod
    def create():
        Permission.table.create()


class Document(BaseModel):
    table = Table(
        _table_name.documents.value,
        columns=[
            Column("cid", ColumnType.Varchar, unique=True),
            Column("name", ColumnType.Varchar),
            Column("blob", ColumnType.Bytea),
            Column(
                "account_address",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_address",
                    reference=ForeignKeyReference("accounts", "address"),
                ),
            ),
            Column("key_image", ColumnType.Varchar),
            Column("creatd_at", ColumnType.Integer),
        ],
        conn=config.connection,
    )

    def __init__(
        self,
        cid: str,
        name: str,
        blob: DocumentBlob,
        account: Account,
        key_img: str = SQL_NULL,
        created_at: int = now(),
    ):
        self.cid = cid
        self.name = name
        self.blob = blob
        self.account = account
        self.key_img = key_img
        self.created_at = created_at

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.cid),
            quote(self.name),
            quote(self.blob.decode()),
            quote(self.account.address),
            quote(self.key_img),
            quote(self.created_at),
        )

    def from_row(row: Tuple[Any]) -> "Document":
        (cid, name, blob, account_address, key_img, created_at) = row
        account = Account.get(where={"address": account_address})
        return Document(
            cid,
            name,
            DocumentBlob(decode(blob, Encoding.UTF8)),
            account,
            key_img,
            created_at,
        )

    @staticmethod
    def create():
        Document.table.create()

    def set_text(self, text: str):
        self.blob = DocumentBlob(text)

    def update_with_new_blob(self, blob: str) -> FernetBundle:
        bundle = fernet_bundle()
        ciphertext = decode(
            bundle.key.encrypt(encode(blob, Encoding.UTF8)), Encoding.UTF8
        )
        self.key_img = bundle.key_img
        self.blob = DocumentBlob(ciphertext)
        return bundle


class Setting(BaseModel):
    table = Table(
        _table_name.settings.value,
        columns=[
            Column(
                "account_address",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_address",
                    reference=ForeignKeyReference("accounts", "address"),
                ),
            ),
            Column("key", ColumnType.Varchar),
            Column("value", ColumnType.Integer),
        ],
        conn=config.connection,
    )

    def __init__(
        self,
        account: Account,
        key: SettingKey,
        value: int,
    ):
        self.account = account
        self.key = key
        self.value = value

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.account.address),
            quote(self.key.value),
            quote(self.value),
        )

    def from_row(row: Tuple[Any]) -> "Setting":
        (account_address, key, value) = row
        account = Account.get(where={"address": account_address})
        return Setting(account, SettingKey(key), value)

    def enabled(self) -> bool:
        return self.value == 1

    def disabled(self) -> bool:
        return not self.enabled()

    def toggle(self):
        self.value = 0 if self.value == 1 else 1
        Setting.update(
            update={"value": self.value},
            where={
                "key": self.key.value,
                "account_address": self.account.address,
            },
        )

    @staticmethod
    def create():
        Setting.table.create()


class WebhookType(enum.Enum):
    Incoming = "Incoming"
    Outgoing = "Outgoing"


class Webhook(BaseModel):
    table = Table(
        _table_name.webhooks.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column(
                "third_party_id",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "third_party_id",
                    reference=ForeignKeyReference("third_parties", "uuid"),
                ),
            ),
            Column("endpoint", ColumnType.Varchar),
            Column("type", ColumnType.Varchar),
            Column("name", ColumnType.Varchar),
            Column("active", ColumnType.Integer, default=0),
        ],
        conn=config.connection,
    )

    def __init__(
        self,
        uuid: str,
        third_party: ThirdParty,
        endpoint: str,
        type: WebhookType,
        name: str,
        active: int = 0,
    ):
        self.uuid = uuid
        self.third_party = third_party
        self.endpoint = endpoint
        self.type = type
        self.name = name
        self.active = active

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.uuid),
            quote(self.third_party.uuid),
            quote(self.endpoint),
            quote(self.type.value),
            quote(self.name),
            quote(self.active),
        )

    @staticmethod
    def from_row(row: Tuple[Any]) -> "Webhook":
        (uuid, third_party_id, endpoint, type, name, active) = row
        party = ThirdParty.get(where={"uuid": third_party_id})
        return Webhook(uuid, party, endpoint, WebhookType[type], name, active)

    @staticmethod
    def create():
        Webhook.table.create()

    def toggle(self):
        self.active = 0 if self.active == 1 else 1
        Webhook.update(
            update={"active": self.active}, where={"uuid": self.uuid}
        )


class AccessRequestStatus(enum.Enum):
    Pending = "Pending"
    Denied = "Denied"
    Granted = "Granted"


class AccessRequest(BaseModel):
    table = Table(
        _table_name.access_requests.value,
        columns=[
            Column("uuid", ColumnType.Varchar, unique=True),
            Column(
                "third_party_id",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "third_party_id",
                    reference=ForeignKeyReference("third_parties", "uuid"),
                ),
            ),
            Column(
                "account_address",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "account_address",
                    reference=ForeignKeyReference("accounts", "address"),
                ),
            ),
            Column(
                "document_cid",
                ColumnType.Varchar,
                foreign_key=ForeignKey(
                    "document_cid",
                    reference=ForeignKeyReference("documents", "cid"),
                ),
            ),
            Column("status", ColumnType.Varchar),
            Column("callback_url", ColumnType.Varchar),
            Column("callback_data", ColumnType.Varchar),
            Column("created_at", ColumnType.Integer),
            Column("expiry", ColumnType.Integer),
        ],
        conn=config.connection,
    )

    def __init__(
        self,
        uuid: str,
        third_party: ThirdParty,
        account: Account,
        document: Document,
        status: AccessRequestStatus,
        callback_url: str,
        callback_data: Dict[str, str],
        created_at: int = now(),
        expiry: int = now() + 60 * 60 * 24,  # 24-hour expiry
    ):
        self.uuid = uuid
        self.third_party = third_party
        self.account = account
        self.document = document
        self.status = status
        self.callback_url = callback_url
        self.callback_data = callback_data
        self.created_at = created_at
        self.expiry = expiry

    def to_row(self) -> Tuple[Any]:
        return (
            quote(self.uuid),
            quote(self.third_party.uuid),
            quote(self.account.address),
            quote(self.document.cid),
            quote(self.status.value),
            quote(self.callback_url),
            quote(json.dumps(self.callback_data)),
            quote(self.created_at),
            quote(self.expiry),
        )

    @staticmethod
    def from_row(row: Tuple[Any]) -> "AccessRequest":
        (
            uuid,
            third_party_id,
            account_address,
            document_cid,
            status,
            callback_url,
            callback_data,
            created_at,
            expiry,
        ) = row
        party = ThirdParty.get(where={"uuid": third_party_id})
        account = Account.get(where={"address": account_address})
        document = Document.get(where={"cid": document_cid})
        return AccessRequest(
            uuid,
            party,
            account,
            document,
            AccessRequestStatus[status],
            callback_url,
            json.loads(callback_data),
            created_at,
            expiry,
        )

    @staticmethod
    def create():
        AccessRequest.table.create()


class Model:
    AccessToken = AccessToken
    ThirdParty = ThirdParty
    Permission = Permission
    Account = Account
    Document = Document
    Setting = Setting
    Webhook = Webhook
    ThirdPartyAccount = ThirdPartyAccount
    AccessRequest = AccessRequest
