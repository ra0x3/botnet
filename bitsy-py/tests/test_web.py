from pydoc import doc
from fastapi.testclient import TestClient
from fastapi import status

from bitsy import app
from bitsy._const import *
from bitsy._models import *
from bitsy._uses import *
from .utils import *
from .conftest import *

db_path = "test_web.db"


class TestWeb(BaseTestClass):
    def setup_method(self):
        self.client = TestClient(app)
        self.db_path = db_path
        self.conn = self.setup_method_models(self.db_path)

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
            json={"data": "Attack at dawn!", "pubkey": account.pubkey},
        )
        assert response.status_code == status.HTTP_200_OK
        rjson = response.json()

        document = Document.from_row(
            (rjson["cid"], rjson["blob"]["data"], rjson["account"]["pubkey"])
        )

        assert isinstance(document.cid, str)
        assert document.blob.data == "Attack at dawn!"

    def test_route_new_access_token_for_third_party(self):
        party = create_third_party()

        response = self.client.post("/access_token", json={"uuid": party.uuid})
        rjson = response.json()

        token = AccessToken.from_row((rjson["uuid"],))
        assert response.status_code == status.HTTP_200_OK
        assert isinstance(token.uuid, str)

    def test_route_create_account(self, mnemnonic):
        response = self.client.post("/account", json={"mnemnonic": mnemnonic})
        rjson = response.json()

        pubkey = mnemnonic_to_pubkey(mnemnonic)

        account = Account.from_row((rjson["pubkey"],))
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
                "key": PermKey.Read.value,
                "party_id": party.uuid,
                "pubkey": account.pubkey,
                "document_id": document.cid,
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


remove_file(db_path)
