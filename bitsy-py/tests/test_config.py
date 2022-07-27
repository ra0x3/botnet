from bitsy._config import *


class TestConfig:
    def test_from_default_manifest_returns_config_with_default(self):
        c = BitsyConfig.from_default_manifest()

        assert c.pg_host == "127.0.0.1"

    def test_from_default_manifest_with_opts_overloads_base_config_with_opts(self):
        c = BitsyConfig.from_default_manifest_with_opts({"pg_host": "0.0.0.0", "workers": 10})
        assert c.pg_database == "bitsy"
        assert c.workers == 10
