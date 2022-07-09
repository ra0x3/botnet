import abc
import eth_account
import eth_keys
import hvac

from ._const import web3
from ._config import *
from ._utils import *
from ._t import *


class PublicKey(eth_keys.keys.PublicKey):
    pass


class PrivateKey(eth_keys.keys.PrivateKey):
    pass


class Keypair:
    def __init__(self, privkey: PrivateKey, pubkey: PublicKey):
        self.privkey = privkey
        self.pubkey = pubkey


def eth_account_from_mnemnonic(m: str) -> eth_account.Account:
    return web3.eth.account.from_mnemonic(m, account_path="m/44'/60'/0'/0/0")


def mnemnonic_to_pubkey(m: str) -> PublicKey:
    acct = eth_account_from_mnemnonic(m)
    privkey = eth_keys.keys.PrivateKey(acct._private_key)
    return privkey.public_key


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


class BaseConnection:
    def __init__(self, config: BitsyConfig):
        self.config = config
        self._store: Any = None
        self._init()

    @abc.abstractmethod
    def _init(self):
        raise NotImplementedError

    @abc.abstractmethod
    def get(self, id: str) -> PublicKey:
        raise NotImplementedError

    @abc.abstractmethod
    def put(self, key: PublicKey):
        raise NotImplementedError

    @abc.abstractmethod
    def ids(self) -> List[str]:
        raise NotImplementedError

    @abc.abstractmethod
    def keys(self) -> List[PublicKey]:
        raise NotImplementedError


class InMemoryConnection(BaseConnection):
    def _init(self):
        self._store: Dict[str, PublicKey] = {}

    def get(self, id: str) -> PublicKey:
        return self._store[id]

    def put(self, key: PublicKey):
        hexkey = key.to_hex()
        key_image = blake3_sha256(hexkey)
        self._store[key_image] = hexkey

    def ids(self) -> List[str]:
        return self._store.keys()

    def keys(self) -> List[PublicKey]:
        return self._store.values()


class VaultConnection(BaseConnection):
    def _init(self):
        self._store = hvac.Client(
            url=self.config.vault_address, token=env_var("VAULT_ROOT_TOKEN")
        )

    def get(self, id: str) -> PublicKey:
        response = self._store.secrets.kv.v2.read_secret(id)
        return response["data"]["data"]["key"]

    def put(self, key: PublicKey):
        hexkey = key.to_hex()
        key_image = blake3_sha256(hexkey)
        self._store.secrets.kv.v2.create_or_update_secret(
            key_image, secret={"key": hexkey}
        )

    def ids(self) -> List[str]:
        return self._store.keys()

    def keys(self) -> List[PublicKey]:
        return self._store.values()


Connection = TypeVar(
    "Connection",
    bound=Union[InMemoryConnection, VaultConnection],
    covariant=True,
)


class KeyStore_:
    def __init__(self, config: BitsyConfig, *args, **kwargs):
        self.config = config
        self._store: Connection = self._connection()

    def _connection(self) -> Connection:
        if self.config.keystore_provider == KeyStoreProvider.InMemory.value:
            return InMemoryConnection(self.config)
        elif self.config.keystore_provider == KeyStoreProvider.Vault.value:
            return VaultConnection(self.config)
        else:
            raise NotImplementedError

    def get(self, id: str) -> PublicKey:
        return self._store.get(id)

    def put(self, key: PublicKey):
        self._store.put(key)


KeyStore = KeyStore_(BitsyConfig)
