from fastapi import FastAPI, Request, Response, APIRouter
from fastapi.routing import APIRoute
from pydantic import BaseModel
import strawberry

from ._models import PermissionKey, SettingKey
from ._uses import *
from ._utils import *

# IMPORTANT: FastAPI module should match definitions in _models.py


@strawberry.type
class AccessToken(BaseModel):
    uuid: str
    expiry: int


@strawberry.type
class ThirdParty(BaseModel):
    uuid: str
    access_token: AccessToken


@strawberry.type
class Account(BaseModel):
    pubkey: str


@strawberry.type
class DocumentBlob(BaseModel):
    data: str


@strawberry.type
class Document(BaseModel):
    cid: str
    blob: DocumentBlob
    account: Account
    key_image: str


@strawberry.type
class Permission(BaseModel):
    uuid: str
    key: str
    document: Document
    value: int
    account: Account
    third_party: ThirdParty


@strawberry.type
class Setting(BaseModel):
    id: int
    key: SettingKey
    value: int
    access_token: AccessToken


class DummyRoute(APIRoute):
    def get_route_handler(self) -> Callable:
        original_route_handler = super().get_route_handler()

        async def dummy_route_handler(request: Request) -> Response:
            response = await original_route_handler
            return response

        return dummy_route_handler


app = FastAPI()


# IMPORTANT: Routes have same name as the use-case they serve, but are prefixed with 'route_'


@app.get("/")
def route_index():
    return "Welcome to Bitsy!"


@app.post("/third-party")
async def route_create_third_party(request: Request):
    party: ThirdParty = create_third_party()
    return party


@app.post("/document")
async def route_create_document(request: Request):
    body = await request.json()
    doc: Document = create_document_for_account_id(
        body["account_pubkey"], encode(body["data"], Encoding.UTF8)
    )
    return doc


@app.post("/access-token")
async def route_new_access_token_for_third_party(request: Request):
    body = await request.json()
    third_party_id = body["uuid"]
    token: AccessToken = create_access_token_for_third_party_id(third_party_id)
    return token


@app.post("/account")
async def route_create_account(request: Request):
    body = await request.json()
    pubkey = mnemnonic_to_pubkey(body["mnemnonic"])
    account: Account = create_account(pubkey)
    return account


@app.post("/permission")
async def route_grant_perms_on_doc_for_third_party(request: Request):

    body = await request.json()

    doc_id = body.get("document_cid")

    perm: Permission
    if doc_id is None:
        perm: Permission = grant_perms_on_new_doc_for_third_party_id(
            PermissionKey(body["permission_key"]),
            body["party_id"],
            encode(body["data"], Encoding.UTF8),
            body["account_pubkey"],
        )
    else:
        perm: Permission = grant_perms_on_existing_doc_for_third_party_id(
            PermissionKey(body["permission_key"]),
            body["party_id"],
            body["account_pubkey"],
            body["document_cid"],
        )
    return perm


@app.delete("/permission")
async def route_revoke_perms_on_existing_doc_for_third_party_id(
    request: Request,
):
    body = await request.json()
    return revoke_perms_on_existing_doc_for_third_party_id(
        body["party_id"],
        body["document_cid"],
        body["account_pubkey"],
        PermissionKey[body["permission_key"]],
    )


@app.post("/third-party-access-doc")
async def route_third_party_access_document_id(request: Request):

    body = await request.json()

    third_party_id = body["third_party_id"]
    document_cid = body["document_cid"]
    account_pubkey = body["account_pubkey"]

    document = third_party_access_document_id(
        third_party_id, document_cid, account_pubkey
    )

    return document


@app.get("/account-stats")
async def route_get_stats_for_account_id(request: Request):
    body = await request.json()

    account_pubkey = body["account_pubkey"]
    return get_stats_for_account_id(account_pubkey)


@app.post("/setting")
async def route_add_setting_to_account_id(request: Request):
    body = await request.json()

    return add_setting_to_account_id(
        account_pubkey=body["account_pubkey"],
        key=SettingKey[body["setting_key"]],
        value=body["value"],
    )


@app.put("/setting")
async def route_toggle_account_setting_id(request: Request):
    body = await request.json()

    return toggle_account_setting_id(
        account_pubkey=body["account_pubkey"],
        key=SettingKey(body["setting_key"]),
    )


router = APIRouter(route_class=DummyRoute)

app.include_router(router)
