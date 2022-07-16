from bitsy._t import *
from bitsy._uses import *
from bitsy._utils import *
from bitsy._models import *
from bitsy._crypto import *

from .conftest import *


class TestUsesCases(BaseTestClass):
    def setup_method(self):
        self.conn = self.setup_method_models()

    def test_can_create_store_get_access_token(self):
        token = create_access_token()
        assert isinstance(token, AccessToken)

        result = self.get_from_db(
            f"SELECT * FROM access_tokens WHERE uuid = '{token.uuid}';"
        )
        assert len(result) >= 1

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

    def test_can_create_store_get_account(self, keypair):
        keys = keypair
        account = create_account(keys.pubkey)
        assert isinstance(account, Account)

        result = self.get_from_db(
            f"SELECT * FROM accounts WHERE pubkey = '{account.pubkey}';"
        )
        assert len(result) == 1

        get_account = Account.from_row(result[0])
        assert account.pubkey == get_account.pubkey

        response = keystore.get_key(account.pubkey)
        assert response == keys.pubkey.to_hex()

        result = self.get_from_db(
            f"SELECT * FROM settings WHERE key = '{SettingKey.BitsyVaultDeletegation.value}' AND account_pubkey = '{account.pubkey}';"
        )
        assert len(result) == 1
        get_setting = Setting.from_row(result[0])

        result = self.get_from_db(
            f"SELECT * FROM access_tokens WHERE uuid = '{get_setting.access_token.uuid}';"
        )
        assert len(result) == 1

        get_account = AccessToken.from_row(result[0])

    def test_can_create_store_get_document(self, xml_doc, keypair):
        account = create_account(keypair.pubkey)
        doc = create_document_for_account(xml_doc, account)
        assert isinstance(doc, Document)

        result = self.get_from_db(
            f"SELECT * FROM documents WHERE cid = '{doc.cid}';"
        )
        assert len(result) == 1

        get_doc = Document.from_row(result[0])
        assert doc.cid == get_doc.cid
        assert doc.blob.data == get_doc.blob.data
        assert doc.key_img is not None

        key = keystore.get_bytes(doc.key_img)
        assert isinstance(key, bytes)

    def test_grant_perms_on_new_doc_for_third_party(self, keypair):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        _ = create_access_token_for_third_party(party)
        perm = grant_perms_on_new_doc_for_third_party(
            PermissionKey.Other, party, "hello world", account, 12
        )

        result = self.get_from_db(
            f"SELECT * FROM permissions WHERE document_cid = '{perm.document.cid}';"
        )
        assert len(result) == 1

        get_perm = Permission.from_row(result[0])
        assert perm.uuid == get_perm.uuid
        assert perm.document.blob.data == get_perm.document.blob.data

    def test_grant_perms_on_existing_doc_for_third_party(self, keypair):
        account = create_account(keypair.pubkey)
        doc = create_document_for_account("hello world", account)
        party = create_third_party()
        _ = create_access_token_for_third_party(party)
        perm = grant_perms_on_new_doc_for_third_party(
            PermissionKey.Other, party, doc.blob.data, account, 12
        )

        perm = grant_perms_on_existing_doc_for_third_party(
            PermissionKey.Other, party, account, doc
        )
        assert isinstance(perm, Permission)

        result = self.get_from_db(
            f"SELECT * FROM permissions WHERE document_cid = '{perm.document.cid}';"
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

    def test_revoke_third_party_perms_on_account(self, keypair):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        _ = create_access_token_for_third_party(party)
        perm = grant_perms_on_new_doc_for_third_party(
            PermissionKey.Other, party, "hello world", account, 12
        )

        updated_perm = revoke_third_party_perms_on_account(
            PermissionKey.Other, account, party
        )

        assert updated_perm.value == 0
        assert updated_perm.value != perm.value

    def test_list_all_third_party_perms_for_account(self, keypair):
        n = 3

        account = create_account(keypair.pubkey)

        for _ in range(n):
            party = create_third_party()
            _ = create_access_token_for_third_party(party)

            _ = grant_perms_on_new_doc_for_third_party(
                PermissionKey.Other, party, "hello world", account, 12
            )

        perms = list_all_third_party_perms_for_account(account)

        assert len(perms) == n

    @pytest.mark.skip(reason="Not Implemented")
    def test_toggle_setting_for_account(self):
        raise NotImplementedError

    def test_third_party_access_document_id(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        document = create_document_for_account(xml_doc, account)
        _ = grant_perms_on_existing_doc_for_third_party(
            PermissionKey.Read, party, account, document
        )
        updated_doc = third_party_access_document_id(
            party.uuid, document.cid, account.pubkey
        )

        hexkey = keystore.get_bytes(updated_doc.key_img)
        assert isinstance(hexkey, bytes)

        fernet = fernet_from(unhexlify(hexkey))
        plaintext = fernet.decrypt(encode(updated_doc.blob.data, Encoding.UTF8))
        assert plaintext == encode(xml_doc, Encoding.UTF8)

    def test_get_stats_for_account(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        document = create_document_for_account(xml_doc, account)
        _ = grant_perms_on_existing_doc_for_third_party(
            PermissionKey.Read, party, account, document
        )

        stats = get_stats_for_account(account)
        assert isinstance(stats, AccountStat)

        assert stats.account_age == 0
        assert stats.perm_count == 1

    def test_add_setting_to_account(self, keypair):
        account = create_account(keypair.pubkey)
        setting = add_setting_to_account(account, SettingKey.Other, 1)
        assert isinstance(setting, Setting)
        assert setting.access_token is None
        assert setting.account_pubkey == account.pubkey

        results = self.get_from_db(
            f"SELECT * FROM settings WHERE account_pubkey = '{account.pubkey}'"
        )
        get_setting = Setting.from_row(results[0])

        assert get_setting.enabled()

    def test_toggle_account_setting(self, keypair):
        account = create_account(keypair.pubkey)
        setting = add_setting_to_account(account, SettingKey.Other, 1)
        assert setting.enabled()

        updated_setting = toggle_account_setting(account, SettingKey.Other)
        assert updated_setting.disabled()

    def test_revoke_perms_on_existing_doc_for_third_party(
        self, keypair, xml_doc
    ):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        document = create_document_for_account(xml_doc, account)
        perm = grant_perms_on_existing_doc_for_third_party(
            PermissionKey.Read, party, account, document
        )

        get_perm = self.get_from_db(
            f"SELECT * FROM permissions WHERE uuid = '{perm.uuid}';"
        )
        assert get_perm is not None

        revoke_perms_on_existing_doc_for_third_party(
            party, document.cid, account, perm.key
        )

        get_perm = self.get_from_db(
            f"SELECT * FROM permissions WHERE uuid = '{perm.uuid}';"
        )
        assert not get_perm

    def test_update_existing_doc_for_account(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        document = create_document_for_account(xml_doc, account)

        new_blob = "Hello, world!"[::-1]
        updated_doc = update_existing_doc_for_account(
            account, new_blob, document
        )

        assert isinstance(updated_doc, Document)
        # .update_with_new_blob() updates original key_img pointer
        assert updated_doc.key_img == document.key_img

        hexkey = keystore.get_bytes(updated_doc.key_img)
        assert isinstance(hexkey, bytes)

        fernet = fernet_from(unhexlify(hexkey))
        plaintext = fernet.decrypt(encode(updated_doc.blob.data, Encoding.UTF8))
        assert plaintext == encode(new_blob, Encoding.UTF8)
