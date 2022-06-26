import abc
import enum

from ._config import BitsyConfig
from ._t import *


class BaseKey:
    def __init__(self, iv: Optional[str] = None):
        self.iv = iv
        self.key = self._make_key()

    @abc.abstractmethod
    def _make_key(self) -> "BaseKey":
        raise NotImplementedError

    def encrypt(self, plaintext: str) -> bytes:
        raise NotImplementedError

    def sign(self, ciphertext: bytes) -> bytes:
        raise NotImplementedError

    def decrypt(self, ciphertext: bytes) -> str:
        raise NotImplementedError


class RatchetKey(BaseKey):
    def _make_key(self):
        raise NotImplementedError

    def tick(self) -> "RatchetKey":
        raise NotImplementedError


class PublicKey(BaseKey):
    def _make_key(self):
        raise NotImplementedError


BitsyKey = TypeVar("BitsyKey", bound=Union[PublicKey, RatchetKey])


class BaseConnection:
    def __init__(self, config: BitsyConfig):
        self.config = config
        self._store: Any = None
        self._init()

    @abc.abstractmethod
    def _init(self):
        raise NotImplementedError

    @abc.abstractmethod
    def get(self, id: str) -> BitsyKey:
        raise NotImplementedError

    @abc.abstractmethod
    def put(self, id: str, key: BitsyKey):
        raise NotImplementedError

    @abc.abstractmethod
    def ids(self) -> List[str]:
        raise NotImplementedError

    @abc.abstractmethod
    def keys(self) -> List[BitsyKey]:
        raise NotImplementedError


class KeyStoreProvider(enum.Enum):
    InMemory = "in-memory"
    Vault = "vault"


class InMemoryConnection(BaseConnection):
    def _init(self):
        self._store: Dict[str, BitsyKey] = {}

    def get(self, id: str) -> BitsyKey:
        return self._store[id]

    def put(self, id: str, key: BitsyKey):
        self._store[id] = key

    def ids(self) -> List[str]:
        return self._store.keys()

    def keys(self) -> List[BitsyKey]:
        return self._store.values()


class VaultConnection(BaseConnection):
    pass


Connection = TypeVar(
    "Connection",
    bound=Union[InMemoryConnection, VaultConnection],
    covariant=True,
)


class KeyStore:
    def __init__(self, config: BitsyConfig, *args, **kwargs):
        self.config = config
        self._conn: Connection = self._connection()
        self._store = {}

    def _connection(self) -> Connection:
        if self.config.keystore_provider == KeyStoreProvider.InMemory.value:
            return InMemoryConnection(self.config)
        raise NotImplementedError

    def get(self, id: str) -> BitsyKey:
        raise NotImplementedError

    def put(self, id: str, key: BitsyKey):
        raise NotImplementedError
