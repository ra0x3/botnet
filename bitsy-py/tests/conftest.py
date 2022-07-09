import random
import string

import pytest
import eth_account
import eth_keys

from bitsy._utils import *
from bitsy._t import *
from bitsy._const import web3
from bitsy._models import Model
from bitsy._crypto import Keypair

from .utils import *


def create_test_db_and_bootstrap_tables(path: str, models: List[Model]):
    conn = create_test_db(path)
    for model in models:
        model.table.conn = conn
        model.create()
    return conn


class BaseTestClass:
    def setup_method_models(self, db_path: str):
        return create_test_db_and_bootstrap_tables(
            db_path,
            models=[
                Model.AccessToken,
                Model.ThirdParty,
                Model.Account,
                Model.Document,
                Model.Permission,
                Model.Setting,
            ],
        )

    def get_from_db(self, query):
        cursor = self.conn.cursor()
        cursor.execute(query)
        return cursor.fetchall()


chars = list(string.ascii_letters)


@pytest.fixture
def xml_doc() -> str:
    return """<?xml version="1.0" encoding="UTF-8"?>
<Document>
   <CrediCard>
      <FirstName>John</FirstName>
      <LastName>Doe</LastName>
      <MiddleInitial>F.</MiddleInitial>
      <CardNumber>453337363738</CardNumber>
      <CV>435</CV>
      <Zipcode>90001</Zipcode>
   </CrediCard>
</Document>"""


@pytest.fixture
def pubkey(n: int = 36) -> str:
    return "".join([random.choice(chars) for _ in range(n)])


@pytest.fixture
def eth_acct() -> eth_account.Account:
    return web3.eth.account.create()


@pytest.fixture
def keypair() -> Keypair:
    acct = web3.eth.account.create()
    privkey = eth_keys.keys.PrivateKey(acct._private_key)
    return Keypair(privkey, privkey.public_key)


@pytest.fixture
def mnemnonic() -> str:
    return RealMetamaskAcct.mnemnonic
