import pytest
from fastapi.testclient import TestClient
from fastapi import status

from bitsy import _app
from bitsy._uses import *
from .conftest import BaseTestClass
from .utils import double_quote


class TestGraphQLApi(BaseTestClass):
    def setup_method(self):
        self.client = TestClient(_app)
        self.conn = self.setup_method_models()

    def test_field_hello(self):
        response = self.client.post(
            "/graphql", json={"query": "query { hello }"}
        )
        assert response.status_code == status.HTTP_200_OK
        assert response.json() == {"data": {"hello": "Hello World"}}

    def test_access_token(self):
        token = create_access_token()
        response = self.client.post(
            "/graphql",
            json={
                "query": "query { access_token(uuid: "
                + double_quote(token.uuid)
                + ") { uuid } }"
            },
        )
        assert response.status_code == status.HTTP_200_OK
        data = response.json()["data"]["access_token"]
        assert data["uuid"] == token.uuid

    def test_third_party(self):
        party = create_third_party()
        token = create_access_token_for_third_party(party)
        response = self.client.post(
            "/graphql",
            json={
                "query": "query { third_party(uuid: "
                + double_quote(party.uuid)
                + ") { uuid access_token { uuid } } }"
            },
        )

        data = response.json()["data"]["third_party"]
        assert data["uuid"] == party.uuid
        assert data["access_token"]["uuid"] == token.uuid

    def test_account(self, keypair):
        account = create_account(keypair.pubkey)
        response = self.client.post(
            "/graphql",
            json={
                "query": "query { account(pubkey: "
                + double_quote(account.pubkey)
                + ") { pubkey }}"
            },
        )

        data = response.json()["data"]["account"]
        assert data["pubkey"] == account.pubkey

    def test_document(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        document = create_document_for_account(xml_doc, account)
        response = self.client.post(
            "/graphql",
            json={
                "query": "query { document(cid: "
                + double_quote(document.cid)
                + ") { cid blob { data } account { pubkey } }}"
            },
        )

        data = response.json()["data"]["document"]
        assert data["cid"] == document.cid

        ciphertext = data["blob"]["data"]
        hexkey = keystore.get_bytes(document.key_img)
        fernet = fernet_from(unhexlify(hexkey))
        assert fernet.decrypt(encode(ciphertext, Encoding.UTF8)) == encode(
            xml_doc, Encoding.UTF8
        )

    def test_permission(self, keypair, xml_doc):
        account = create_account(keypair.pubkey)
        party = create_third_party()
        token = create_access_token_for_third_party(party)
        document = create_document_for_account(xml_doc, account)
        perm = grant_perms_on_existing_doc_for_third_party(
            PermissionKey.Read, party, account, document
        )
        response = self.client.post(
            "/graphql",
            json={
                "query": "query { permission(uuid: "
                + double_quote(perm.uuid)
                + ") { uuid key document { cid blob { data } account { pubkey } } value account { pubkey } }}"
            },
        )

        data = response.json()["data"]["permission"]
        assert data["uuid"] == perm.uuid
        assert data["value"] == 1
