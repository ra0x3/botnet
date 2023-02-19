import random
import string
import os

import pytest
import eth_account
import eth_keys

from bitsy._utils import *

load_dot_env()

from bitsy._t import *
from bitsy._const import web3
from bitsy._models import Model
from bitsy._crypto import Keypair
from bitsy._config import config
from bitsy._uses import create_account, create_third_party_account

from .utils import *


class BaseTestClass:
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


def rand_string(n: int = 36) -> str:
    return "".join([random.choice(chars) for _ in range(n)])


@pytest.fixture
def eth_acct() -> eth_account.Account:
    return web3.eth.account.create()


@pytest.fixture
def test_account() -> Model.Account:
    keypair = keypair_func()
    password_hash = blake3_hexdigest(rand_string())
    return create_account(pubkey=keypair.pubkey, password_hash=password_hash)


@pytest.fixture
def test_party_account() -> Model.Account:
    keypair = keypair_func()
    password_hash = blake3_hexdigest(rand_string())
    return create_third_party_account(pubkey=keypair.pubkey, password_hash=password_hash)


@pytest.fixture
def keypair() -> Keypair:
    acct = web3.eth.account.create()
    privkey = eth_keys.keys.PrivateKey(acct._private_key)
    return Keypair(privkey, privkey.public_key)


def keypair_func() -> Keypair:
    acct = web3.eth.account.create()
    privkey = eth_keys.keys.PrivateKey(acct._private_key)
    return Keypair(privkey, privkey.public_key)


@pytest.fixture
def mnemonic() -> str:
    return RealMetamaskAcct.mnemonic
