from bitsy._models import *
from bitsy._uses import *
from bitsy._utils import *
from bitsy._crypto import *
from bitsy._config import *


from .conftest import *


db_path = "test_vault.db"


class TestVault(BaseTestClass):
    def setup_method(self):
        self.store = KeyStore_(BitsyConfig(keystore_provider=KeyStoreProvider.Vault.value))

    @pytest.mark.skipif(
        BitsyConfig.keystore_provider == KeyStoreProvider.InMemory.value,
        reason="Vault keystore unavailable.",
    )
    def test_basic_set_and_get(self, keypair):
        pubkey = keypair.pubkey
        keyimg = key_image(pubkey.to_hex())
        self.store.put_key(keypair.pubkey)

        key = self.store.get_key(keyimg)
        assert key == pubkey.to_hex()

    @pytest.mark.skip(reason="Not implemented")
    def test_set_and_get_with_versioning(self, keypair):
        raise NotImplementedError


remove_file([db_path])
