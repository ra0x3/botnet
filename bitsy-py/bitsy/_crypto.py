import abc
import base64
import eth_account
import eth_keys
import hvac
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.fernet import Fernet

from ._const import web3
from ._config import *
from ._utils import *
from ._t import *


# FIXME: Obviously this is a no-no
def key_image(key: Union[str, bytes]) -> str:
    return blake3_sha256(key)


def salt(n: int = 16) -> bytes:
    return os.urandom(n)


class FernetBundle:
    def __init__(self, key_bytes: bytes, key_img: str, hexkey: str):
        self.key: Fernet = fernet_from(key_bytes)
        self.key_bytes = key_bytes
        self.key_img = key_img
        self.hexkey = hexkey


def fernet_bundle(key_bytes: Optional[bytes] = None) -> FernetBundle:
    key_bytes = key_bytes or fernet_bytes()
    hexkey = hexlify(key_bytes)
    key_img = key_image(hexkey)
    return FernetBundle(key_bytes, key_img, hexkey)


kdf = PBKDF2HMAC(
    algorithm=hashes.SHA256(), length=32, salt=salt(), iterations=390000
)


def pbkdf2hmac_kdf(key: bytes) -> bytes:
    return base64.b64encode(kdf.derive(key))


def fernet_from(key: bytes) -> Fernet:
    return Fernet(key)


def aes_key(n: int = 32, iv: int = 16) -> algorithms.AES:
    key = os.urandom(n)
    iv = os.urandom(iv)
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv))
    return cipher


def fernet_bytes() -> bytes:
    return Fernet.generate_key()


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
    def get_key(self, id: str) -> PublicKey:
        raise NotImplementedError

    @abc.abstractmethod
    def get_bytes(self, id: str) -> bytes:
        raise NotImplementedError

    @abc.abstractmethod
    def put_key(self, key: PublicKey):
        raise NotImplementedError

    @abc.abstractmethod
    def put_bytes(self, key: bytes):
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

    def get_key(self, id: str) -> PublicKey:
        return self._store[id]

    def get_bytes(self, id: str) -> bytes:
        return self._store[id]

    def put_key(self, key: PublicKey):
        hexkey = key.to_hex()
        key_img = key_image(hexkey)
        self._store[key_img] = hexkey

    def put_bytes(self, key: bytes):
        key_img = key_image(decode(key, Encoding.UTF8))
        self._store[key_img] = key

    def ids(self) -> List[str]:
        return self._store.keys()

    def keys(self) -> List[PublicKey]:
        return self._store.values()


class VaultConnection(BaseConnection):
    def _init(self):
        self._store = hvac.Client(
            url=self.config.vault_address, token=env_var("VAULT_ROOT_TOKEN")
        )

    def get_key(self, id: str) -> PublicKey:
        response = self._store.secrets.kv.v2.read_secret(id)
        return response["data"]["data"]["key"]

    def get_bytes(self, id: str) -> bytes:
        response = self._store.secrets.kv.v2.read_secret(id)
        return encode(response["data"]["data"]["key"], Encoding.UTF8)

    def put_key(self, key: PublicKey):
        hexkey = key.to_hex()
        key_img = key_image(hexkey)
        self._store.secrets.kv.v2.create_or_update_secret(
            key_img, secret={"key": hexkey}
        )

    def put_bytes(self, key: bytes):
        key_img = key_image(key)
        self._store.secrets.kv.v2.create_or_update_secret(
            key_img, secret={"key": decode(key, Encoding.UTF8)}
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

    def get_key(self, id: str) -> PublicKey:
        return self._store.get_key(id)

    def get_bytes(self, id: str) -> bytes:
        return self._store.get_bytes(id)

    def put_key(self, key: PublicKey):
        self._store.put_key(key)

    def put_bytes(self, key: bytes):
        self._store.put_bytes(key)


KeyStore = KeyStore_(BitsyConfig)
