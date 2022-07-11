from bitsy._crypto import KeyStore_, eth_account_from_mnemnonic, key_image
from bitsy._config import *

# from bitsy._utils import
from .utils import *


class TestFuncs:
    def test_eth_account_from_mnemnonic(self):
        acct = eth_account_from_mnemnonic(RealMetamaskAcct.mnemnonic)
        assert acct.address == RealMetamaskAcct.address


class TestKeyStore:
    def test_in_memory_key_store_basic_funcs(self, keypair):
        store = KeyStore_(
            BitsyConfig(keystore_provider=KeyStoreProvider.InMemory.value)
        )
        pubkey = keypair.pubkey
        keyimg = key_image(pubkey.to_hex())
        store.put_key(pubkey)
        response = store.get_key(keyimg)
        assert response == pubkey.to_hex()

    def test_vault_key_store_basic_funcs(self, keypair):
        store = KeyStore_(BitsyConfig())
        pubkey = keypair.pubkey
        keyimg = key_image(pubkey.to_hex())
        store.put_key(pubkey)
        response = store.get_key(keyimg)
        assert response == pubkey.to_hex()
