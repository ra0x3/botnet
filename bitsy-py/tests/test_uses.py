import pathlib
from .utils import create_test_db_and_bootstrap_tables

from bitsy._t import *
from bitsy._uses import *
from bitsy._utils import *
from bitsy._models import *
from bitsy._config import BitsyConfig


def get_from_db(conn, query):
    cursor = conn.cursor()
    cursor.execute(query)
    return cursor.fetchall()

class TestUsesCases:
    def setup_method(self):
        self.conn = create_test_db_and_bootstrap_tables("test_uses.db", models=[Model.AccessToken, Model.ThirdParty])


    def test_can_create_store_get_access_token(self):
        token = create_access_token()
        assert isinstance(token, AccessToken)

        result = get_from_db(self.conn, "SELECT * FROM access_tokens;")
        assert len(result) == 1

        get_token = AccessToken.from_row(result[0])
        assert get_token.uuid == token.uuid

    def test_can_create_store_get_third_party(self):
        token = create_access_token()
        party = create_third_party(token)
        assert isinstance(party, ThirdParty)
        
        result = get_from_db(self.conn, "SELECT * FROM third_parties;")
        assert len(result) == 1

        get_party = ThirdParty.from_row(result[0])
        assert get_party.uuid == party.uuid
        assert get_party.access_token.uuid == token.uuid


    def teardown_method(self):
        remove_test_db("test_uses.db")
