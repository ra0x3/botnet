import pytest

from bitsy._t import *
from bitsy._uses import *
from bitsy._utils import *
from bitsy._models import *

from .conftest import *


db_path = "test_uses.db"


class TestUsesCases(BaseTestClass):
    def setup_method(self):
        self.db_path = db_path
        self.conn = self.setup_method_models(self.db_path)

    def test_can_create_store_get_access_token(self):
        token = create_access_token()
        assert isinstance(token, AccessToken)

        result = self.get_from_db("SELECT * FROM access_tokens;")
        assert len(result) == 1

        get_token = AccessToken.from_row(result[0])
        assert get_token.uuid == token.uuid

    def test_can_create_store_get_third_party_without_token(self):
        party = create_third_party()
        assert isinstance(party, ThirdParty)

        result = self.get_from_db(
            f"SELECT * FROM third_parties WHERE uuid = '{party.uuid}';"
        )
        assert len(result) == 1

        get_party = ThirdParty.from_row(result[0])
        assert not get_party.access_token
        assert get_party.uuid == party.uuid

    def test_can_create_store_get_account(self, pubkey):
        account = create_account(pubkey)
        assert isinstance(account, Account)

        result = self.get_from_db(
            f"SELECT * FROM accounts WHERE pubkey = '{account.pubkey}';"
        )
        assert len(result) == 1

        get_account = Account.from_row(result[0])
        assert account.pubkey == get_account.pubkey

    def test_can_create_store_get_document(self, pubkey):
        account = create_account(pubkey)
        doc = create_document_for_account("hello world", account)
        assert isinstance(doc, Document)

        result = self.get_from_db(
            f"SELECT * FROM documents WHERE cid = '{doc.cid}';"
        )
        assert len(result) == 1

        get_doc = Document.from_row(result[0])
        assert doc.cid == get_doc.cid
        assert doc.blob.data == get_doc.blob.data

    def test_grant_perms_on_new_doc_for_third_party(self, pubkey):
        account = create_account(pubkey)
        party = create_third_party()
        _ = create_access_token_for_third_party(party)
        perm = grant_perms_on_new_doc_for_third_party(
            PermKey.Other, party, "hello world", account, 12
        )

        result = self.get_from_db(
            f"SELECT * FROM permissions WHERE document_id = '{perm.document.cid}';"
        )
        assert len(result) == 1

        get_perm = Permission.from_row(result[0])
        assert perm.uuid == get_perm.uuid
        assert perm.document.blob.data == get_perm.document.blob.data

    def test_grant_perms_on_existing_doc_for_third_party(self, pubkey):
        account = create_account(pubkey)
        doc = create_document_for_account("hello world", account)
        party = create_third_party()
        _ = create_access_token_for_third_party(party)
        perm = grant_perms_on_new_doc_for_third_party(
            PermKey.Other, party, doc.blob.data, account, 12
        )

        perm = grant_perms_on_existing_doc_for_third_party(
            PermKey.Other, party, account, doc
        )
        assert isinstance(perm, Permission)

        result = self.get_from_db(
            f"SELECT * FROM permissions WHERE document_id = '{perm.document.cid}';"
        )

        get_perm = Permission.from_row(result[0])
        assert perm.uuid == get_perm.uuid
        assert perm.document.blob.data == get_perm.document.blob.data

    def test_new_access_token_for_third_party(self):
        party = create_third_party()
        token = create_access_token_for_third_party(party)

        result = new_access_token_for_third_party(party)
        assert token.uuid != result.uuid

        result = self.get_from_db(
            f"SELECT * FROM third_parties WHERE access_token = '{result.uuid}';"
        )

        assert len(result) == 1

    def test_revoke_third_party_perms_on_account(self, pubkey):
        account = create_account(pubkey)
        party = create_third_party()
        _ = create_access_token_for_third_party(party)
        perm = grant_perms_on_new_doc_for_third_party(
            PermKey.Other, party, "hello world", account, 12
        )

        updated_perm = revoke_third_party_perms_on_account(
            PermKey.Other, account, party
        )

        assert updated_perm.value == 0
        assert updated_perm.value != perm.value

    def test_list_all_third_party_perms_for_account(self, pubkey):
        n = 3

        account = create_account(pubkey)

        for _ in range(n):
            party = create_third_party()
            _ = create_access_token_for_third_party(party)

            _ = grant_perms_on_new_doc_for_third_party(
                PermKey.Other, party, "hello world", account, 12
            )

        perms = list_all_third_party_perms_for_account(account)

        assert len(perms) == n

    def test_toggle_setting_for_account(self):
        pass


remove_file([db_path, "test_uses.db-journal"])
