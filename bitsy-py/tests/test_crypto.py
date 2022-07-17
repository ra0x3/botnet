import pytest
import eth_keys
from bitsy._crypto import *
from bitsy._config import *
from .utils import *


class TestFuncs:
    def test_eth_account_from_mnemonic(self):
        acct = eth_account_from_mnemonic(RealMetamaskAcct.mnemonic)
        assert acct.address == RealMetamaskAcct.address0

    def test_can_recover_pubkey_using_compressed_pubkey_hex(self):
        acct = eth_account_from_mnemonic(RealMetamaskAcct.mnemonic)
        privkey = eth_keys.keys.PrivateKey(acct._private_key)
        recovered = recover_pubkey_from_compressed_hex(RealMetamaskAcct.compressed_pubkey)
        assert recovered.to_hex() == privkey.public_key.to_hex()
        assert recovered.to_checksum_address() == RealMetamaskAcct.address0


class TestKeyStore:
    def test_in_memory_key_store_basic_funcs(self, keypair):
        store = KeyStore_(BitsyConfig(keystore_provider=KeyStoreProvider.InMemory.value))
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
