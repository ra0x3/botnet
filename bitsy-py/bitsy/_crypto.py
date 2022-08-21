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


class PublicKey(eth_keys.keys.PublicKey):
    pass


class PrivateKey(eth_keys.keys.PrivateKey):
    pass


# FIXME: Obviously this is a no-no
def key_image(key: Union[str, bytes]) -> str:
    return blake3_hexdigest(key)


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


kdf = PBKDF2HMAC(algorithm=hashes.SHA256(), length=32, salt=salt(), iterations=390000)


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


def recover_pubkey_from_compressed_hex(hex: str) -> PublicKey:
    return eth_keys.keys.PublicKey.from_compressed_bytes(unhexlify(hex[2:]))


class Keypair:
    def __init__(self, privkey: PrivateKey, pubkey: PublicKey):
        self.privkey = privkey
        self.pubkey = pubkey
        self.address = self.pubkey.to_checksum_address()


def eth_account_from_mnemonic(m: str, password: Optional[str] = None) -> eth_account.Account:
    # https://www.reddit.com/r/seedstorage/comments/voixjj/comment/iedodmv/?utm_source=share&utm_medium=web2x&context=3
    if password:
        return web3.eth.account.from_mnemonic(mnemonic=m, account_path="m/44'/60'/0'/0/0", passphrase=password)
    return web3.eth.account.from_mnemonic(mnemonic=m, account_path="m/44'/60'/0'/0/0")


def mnemonic_to_pubkey(m: str, password: Optional[str] = None) -> PublicKey:
    acct = eth_account_from_mnemonic(m, password)
    privkey = eth_keys.keys.PrivateKey(acct._private_key)
    return privkey.public_key


def keypair_func() -> Keypair:
    acct = web3.eth.account.create()
    privkey = eth_keys.keys.PrivateKey(acct._private_key)
    return Keypair(privkey, privkey.public_key)


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
    def put_key(self, key: PublicKey):
        raise NotImplementedError

    @abc.abstractmethod
    def get_hex(self, id: str) -> str:
        raise NotImplementedError

    @abc.abstractmethod
    def put_hex(self, key: str):
        raise NotImplementedError


class InMemoryConnection(BaseConnection):
    def _init(self):
        self._store: Dict[str, PublicKey] = {}

    def get_key(self, id: str) -> PublicKey:
        return self._store[id]

    def put_key(self, key: PublicKey):
        hexkey = key.to_hex()
        key_img = key_image(hexkey)
        self._store[key_img] = hexkey

    def get_hex(self, id: str) -> str:
        return self._store[id]

    def put_hex(self, key: str):
        key_img = key_image(decode(key, Encoding.UTF8))
        self._store[key_img] = key


class VaultConnection(BaseConnection):
    def _init(self):
        self._store = hvac.Client(url=self.config.vault_address, token=env_var("VAULT_ROOT_TOKEN"))

    def get_key(self, id: str) -> PublicKey:
        response = self._store.secrets.kv.v2.read_secret(id)
        return response["data"]["data"]["key"]

    def put_key(self, key: PublicKey):
        hexkey = key.to_hex()
        key_img = key_image(hexkey)
        self._store.secrets.kv.v2.create_or_update_secret(key_img, secret={"key": hexkey})

    def get_hex(self, id: str) -> str:
        response = self._store.secrets.kv.v2.read_secret(id)
        return encode(response["data"]["data"]["key"], Encoding.UTF8)

    def put_hex(self, key: str):
        key_img = key_image(key)
        self._store.secrets.kv.v2.create_or_update_secret(key_img, secret={"key": decode(key, Encoding.UTF8)})


_Connection = TypeVar(
    "_Connection",
    bound=Union[InMemoryConnection, VaultConnection],
    covariant=True,
)


class KeyStore_:
    def __init__(self, config: BitsyConfig, *args, **kwargs):
        self.config = config
        self._store: _Connection = self._connection()

    def _connection(self) -> _Connection:
        if self.config.keystore_provider == KeyStoreProvider.InMemory.value:
            return InMemoryConnection(self.config)
        elif self.config.keystore_provider == KeyStoreProvider.Vault.value:
            return VaultConnection(self.config)
        else:
            raise NotImplementedError

    def get_key(self, id: str) -> PublicKey:
        return self._store.get_key(id)

    def get_hex(self, id: str) -> bytes:
        return self._store.get_hex(id)

    def put_key(self, key: PublicKey):
        self._store.put_key(key)

    def put_hex(self, key: bytes):
        self._store.put_hex(key)


keystore = KeyStore_(BitsyConfig)
