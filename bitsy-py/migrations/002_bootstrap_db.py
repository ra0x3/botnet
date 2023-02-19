import os
import json
import random
from eth_utils import to_checksum_address

from bitsy._utils import blake3_hexdigest
from bitsy._crypto import keypair_func
from bitsy._uses import *
from bitsy import config
from migrations import logger, workdir


def up():

    account = json.load(open(os.path.join(workdir, "tests", "account.json"), "r"))

    num_accounts = 10
    num_third_parties = 20
    num_access_requests = random.randint(5, 35)
    num_documents = random.randint(3, 10)
    num_webooks = random.randint(2, 150)

    accounts = []
    third_parties = []
    access_requests = []
    documents = []
    webhooks = []

    cursor = config.connection.cursor()

    addr = to_checksum_address(account["address0"])

    cursor.execute(f"SELECT * FROM accounts WHERE address = '{addr}';")
    default_acct = cursor.fetchall()
    if not default_acct:
        _ = create_account(password_hash=blake3_hexdigest("password"), address=addr)

    # for _ in range(num_accounts):
    #     keypair = keypair_func()
    #     acct = create_account(password_hash=blake3_hexdigest("password"))
    #     accounts.append(acct)

    #     for _ in range(num_documents):
    #         pass

    # for _ in range(num_third_parties):
    #     party = create_third_party_account(blake3_hexdigest("password123"))
    #     third_parties.append(party)

    #     for _ in range(num_webooks):
    #         pass

    #     for _ in range(num_access_requests):
    #         pass
