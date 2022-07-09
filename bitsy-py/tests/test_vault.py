from bitsy._models import *
from bitsy._uses import *
from bitsy._utils import *
from bitsy._crypto import *
from bitsy._config import *


from .conftest import *


db_path = "test_vault.db"


class TestVault(BaseTestClass):
    def setup_method(self):
        self.store = KeyStore_(
            BitsyConfig(keystore_provider=KeyStoreProvider.Vault.value)
        )

    def test_basic_set_and_get(self, keypair):
        pubkey = keypair.pubkey
        key_image = blake3_sha256(pubkey.to_hex())
        self.store.put(keypair.pubkey)

        key = self.store.get(key_image)
        assert key == pubkey.to_hex()

    @pytest.mark.skip(reason="Not implemented")
    def test_set_and_get_with_versioning(self, keypair):
        raise NotImplementedError


remove_file([db_path])
