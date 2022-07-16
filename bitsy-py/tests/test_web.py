from fastapi.testclient import TestClient
from fastapi import status

from bitsy import _app
from bitsy._const import *
from bitsy._models import *
from bitsy._uses import *
from .utils import *
from .conftest import *


class TestWeb(BaseTestClass):
    def setup_method(self):
        self.client = TestClient(_app)
        self.conn = self.setup_method_models()

    def test_route_hello_world(self):
        response = self.client.get("/")
        assert response.status_code == status.HTTP_200_OK
        assert response.text == '"Welcome to Bitsy!"'

    def test_route_create_third_party(self):
        response = self.client.post("/third-party")
        rjson = response.json()

        assert response.status_code == status.HTTP_200_OK
        party = ThirdParty.from_row(tuple(rjson.values()))

        assert isinstance(party.uuid, str)
        assert party.uuid == rjson["uuid"]

    def test_route_create_document(self, keypair):
        account = create_account(keypair.pubkey)
        response = self.client.post(
            "/document",
            json={"data": "Attack at dawn!", "account_pubkey": account.pubkey},
        )
        assert response.status_code == status.HTTP_200_OK
        rjson = response.json()

        document = Document.from_row(
            (
                rjson["cid"],
                rjson["blob"]["data"],
                rjson["account"]["pubkey"],
                rjson["key_img"],
            )
        )

        hexkey = keystore.get_bytes(document.key_img)

        key = fernet_from(unhexlify(hexkey))

        assert isinstance(document.cid, str)
        assert (
            key.decrypt(encode(document.blob.data, Encoding.UTF8))
            == b"Attack at dawn!"
        )

    def test_route_new_access_token_for_third_party(self):
        party = create_third_party()

        response = self.client.post("/access-token", json={"uuid": party.uuid})
        rjson = response.json()

        token = AccessToken.from_row((rjson["uuid"], rjson["expiry"]))
        assert response.status_code == status.HTTP_200_OK
        assert isinstance(token.uuid, str)

    def test_route_create_account(self, mnemnonic):
        response = self.client.post("/account", json={"mnemnonic": mnemnonic})
        rjson = response.json()

        pubkey = mnemnonic_to_pubkey(mnemnonic)

        account = Account.from_row((rjson["pubkey"], rjson["created_at"]))
        assert response.status_code == status.HTTP_200_OK
        assert account.pubkey == blake3_sha256(pubkey.to_hex())

    def test_route_grant_perms_on_doc_for_third_party(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        _ = new_access_token_for_third_party(party)
        document = create_document_for_account(xml_doc, account)
        response = self.client.post(
            "/permission",
            json={
                "permission_key": PermissionKey.Read.value,
                "party_id": party.uuid,
                "account_pubkey": account.pubkey,
                "document_cid": document.cid,
            },
        )
        rjson = response.json()

        permission = Permission.from_row(
            (
                rjson["uuid"],
                rjson["key"],
                rjson["document"]["cid"],
                rjson["value"],
                rjson["account"]["pubkey"],
                rjson["third_party"]["uuid"],
                rjson["ttl"],
            )
        )

        assert response.status_code == status.HTTP_200_OK
        assert permission.account.pubkey == account.pubkey

    def test_route_get_stats_for_account_id(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        _ = new_access_token_for_third_party(party)
        document = create_document_for_account(xml_doc, account)

        response = self.client.get(
            "/account-stats", json={"account_pubkey": account.pubkey}
        )
        rjson = response.json()

        conditions = ["account_age" in rjson, "perm_count" in rjson]

        assert all(conditions)

    def test_route_add_setting_to_account_id(self, keypair):
        account = create_account(keypair.pubkey)
        response = self.client.post(
            "/setting",
            json={
                "account_pubkey": account.pubkey,
                "setting_key": SettingKey.Other.value,
                "value": 1,
            },
        )
        rjson = response.json()
        setting = Setting.from_row(
            (
                rjson["account_pubkey"],
                rjson["key"],
                rjson["value"],
                rjson["access_token"],
            )
        )
        assert setting.account_pubkey == account.pubkey
        assert setting.key == SettingKey.Other
        assert setting.enabled()

    def test_route_toggle_account_setting_id(self, keypair):
        account = create_account(keypair.pubkey)
        setting = add_setting_to_account(account, SettingKey.Other, 1)
        assert setting.enabled()
        response = self.client.put(
            "/setting",
            json={
                "account_pubkey": account.pubkey,
                "setting_key": SettingKey.Other.value,
            },
        )
        rjson = response.json()
        setting = Setting.from_row(
            (
                rjson["account_pubkey"],
                rjson["key"],
                rjson["value"],
                rjson["access_token"],
            )
        )
        assert setting.account_pubkey == account.pubkey
        assert setting.key == SettingKey.Other
        assert setting.disabled()

    def test_route_revoke_perms_on_existing_doc_for_third_party_id(
        self, keypair, xml_doc
    ):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        document = create_document_for_account(xml_doc, account)
        perm = grant_perms_on_existing_doc_for_third_party(
            PermissionKey.Read, party, account, document
        )

        _ = self.client.delete(
            "/permission",
            json={
                "party_id": party.uuid,
                "account_pubkey": account.pubkey,
                "document_cid": document.cid,
                "permission_key": PermissionKey.Read.value,
            },
        )

        perm = self.get_from_db(
            f"SELECT * FROM permissions WHERE uuid = '{perm.uuid}';"
        )
        assert not perm
