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
        self.conn = BitsyConfig.connection

    def test_route_hello_world(self):
        response = self.client.get("/")
        assert response.status_code == status.HTTP_200_OK
        assert response.text == '"Welcome to Bitsy!"'

    def test_route_create_third_party(self):
        response = self.client.post("/third-party", json={"name": "zoofoo"})
        rjson = response.json()

        assert response.status_code == status.HTTP_200_OK
        party = ThirdParty.from_row(tuple(rjson.values()))

        assert isinstance(party.uuid, str)
        assert party.uuid == rjson["uuid"]
        assert party.name == "zoofoo"

    @pytest.mark.skip(reason="Account exists")
    def test_route_create_account(self):
        response = self.client.post(
            "/account",
            json={
                "pubkey": RealMetamaskAcct.compressed_pubkey,
                "address": RealMetamaskAcct.address0,
            },
        )
        rjson = response.json()

        recovered_pubkey = recover_pubkey_from_compressed_hex(RealMetamaskAcct.compressed_pubkey)

        account = Account.from_row((rjson["pubkey"], rjson["address"], rjson["created_at"], None))
        assert response.status_code == status.HTTP_200_OK
        assert account.pubkey == blake3_(recovered_pubkey.to_hex())

    def test_route_create_document(self, keypair):
        account = create_account(keypair.pubkey)
        response = self.client.post(
            "/document",
            json={"data": "Attack at dawn!"},
            headers={"Authorization": account.jwt},
        )
        assert response.status_code == status.HTTP_200_OK
        rjson = response.json()

        document = Document.from_row(
            (
                rjson["cid"],
                rjson["blob"]["data"],
                rjson["account"]["address"],
                rjson["key_img"],
            )
        )

        hexkey = keystore.get_bytes(document.key_img)

        key = fernet_from(unhexlify(hexkey))

        assert isinstance(document.cid, str)
        assert key.decrypt(encode(document.blob.data, Encoding.UTF8)) == b"Attack at dawn!"

    def test_route_new_access_token_for_third_party(self, keypair):
        item = create_third_party_account(keypair.pubkey)

        response = self.client.post(
            "/access-token",
            json={"uuid": item.party.uuid, "name": "foobar"},
            headers={"Authorization": item.account.jwt},
        )
        rjson = response.json()

        token = AccessToken.from_row(
            (
                rjson["uuid"],
                rjson["third_party"]["uuid"],
                rjson["name"],
                rjson["expiry"],
                rjson["active"],
            )
        )
        assert response.status_code == status.HTTP_200_OK
        assert isinstance(token.uuid, str)
        assert not token.active
        assert token.name == "foobar"

    def test_route_grant_perms_on_doc_for_third_party(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        _ = new_access_token_for_third_party(party)
        document = create_document_for_account(xml_doc, account)
        response = self.client.post(
            "/permission",
            json={
                "permission_key": PermissionKey.Read.value,
                "party_ids": [party.uuid],
                "document_cid": document.cid,
            },
            headers={"Authorization": account.jwt},
        )
        rjson = response.json()[0]

        permission = Permission.from_row(
            (
                rjson["uuid"],
                rjson["key"],
                rjson["document"]["cid"],
                rjson["value"],
                rjson["account"]["address"],
                rjson["third_party"]["uuid"],
                rjson["ttl"],
            )
        )

        assert response.status_code == status.HTTP_200_OK
        assert permission.account.pubkey == account.pubkey
        assert permission.account.address == account.address

    def test_route_get_stats_for_account_id(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        _ = new_access_token_for_third_party(party)
        document = create_document_for_account(xml_doc, account)

        response = self.client.get(
            "/account-stats",
            headers={"Authorization": account.jwt},
        )
        rjson = response.json()

        conditions = ["account_age" in rjson, "perm_count" in rjson]

        assert all(conditions)

    def test_route_add_setting_to_account_id(self, keypair):
        account = create_account(keypair.pubkey)
        response = self.client.post(
            "/setting",
            json={
                "key": SettingKey.Other.value,
                "value": 1,
            },
            headers={"Authorization": account.jwt},
        )
        rjson = response.json()
        setting = Setting.from_row(
            (
                rjson["account"]["address"],
                rjson["key"],
                rjson["value"],
            )
        )
        assert setting.account.pubkey == account.pubkey
        assert setting.account.address == account.address
        assert setting.key == SettingKey.Other
        assert setting.enabled()

    def test_route_toggle_account_setting_id(self, keypair):
        account = create_account(keypair.pubkey)
        setting = add_setting_to_account(account, SettingKey.Other, 1)
        assert setting.enabled()
        response = self.client.put(
            "/setting",
            json={
                "key": SettingKey.Other.value,
            },
            headers={"Authorization": account.jwt},
        )
        rjson = response.json()
        setting = Setting.from_row(
            (
                rjson["account"]["address"],
                rjson["key"],
                rjson["value"],
            )
        )
        assert setting.account.pubkey == account.pubkey
        assert setting.account.address == account.address
        assert setting.key == SettingKey.Other
        assert setting.disabled()

    def test_route_toggle_third_party_token(self, keypair):
        party_acct = create_third_party_account(keypair.pubkey)
        token = create_access_token_for_third_party(party_acct.party, "bar")
        assert not token.active

        response = self.client.put(
            "/access-token",
            json={
                "uuid": token.uuid,
            },
            headers={"Authorization": party_acct.account.jwt},
        )

        rjson = response.json()
        token = AccessToken.from_row(
            (
                rjson["uuid"],
                rjson["third_party"]["uuid"],
                rjson["name"],
                rjson["expiry"],
                rjson["active"],
            )
        )

        assert token.active

    def test_route_revoke_perms_on_existing_doc_for_third_party_id(self, keypair, xml_doc):
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
                "document_cid": document.cid,
                "permission_key": PermissionKey.Read.value,
            },
            headers={"Authorization": account.jwt},
        )

        perm = self.get_from_db(f"SELECT * FROM permissions WHERE uuid = '{perm.uuid}';")
        assert not perm

    def test_route_create_third_party_webhook_id(self, keypair):
        item = create_third_party_account(keypair.pubkey)
        response = self.client.post(
            "/webhook",
            json={
                "third_party_id": item.party.uuid,
                "type": WebhookType.Incoming.value,
                "name": "foo",
                "endpoint": "/foo",
            },
            headers={"Authorization": item.account.jwt},
        )
        rjson = response.json()

        assert "third_party" in rjson
        assert not rjson["active"]

    def test_route_delete_third_party_webhook_id(self, keypair):
        party_acct = create_third_party_account(keypair.pubkey)
        webhook = create_third_party_webhook(
            party_acct.party, "/foo", WebhookType.Incoming, "foo", 0
        )

        assert isinstance(webhook, Webhook)

        response = self.client.delete(
            "/webhook",
            json={"uuid": webhook.uuid},
            headers={"Authorization": party_acct.account.jwt},
        )

        assert response.status_code == status.HTTP_200_OK
        result = self.get_from_db(f"SELECT * FROM webhooks WHERE uuid = '{webhook.uuid}'")
        assert not result
