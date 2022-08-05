from fastapi import FastAPI, Request, Response, APIRouter, status
from fastapi.responses import JSONResponse
import enum
from fastapi.routing import APIRoute
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from starlette.middleware.base import BaseHTTPMiddleware
import strawberry
import jwt
from eth_utils import to_checksum_address

from ._models import PermissionKey, SettingKey, Model
from ._uses import *
from ._errors import *
from ._utils import *

# IMPORTANT: FastAPI module should match definitions in _models.py


logger = logging.getLogger("bitsy.web")


class HttpMethod(enum.Enum):
    GET = "GET"
    POST = "POST"


@strawberry.type
class ThirdParty(BaseModel):
    uuid: str
    name: str


@strawberry.type
class AccessToken(BaseModel):
    uuid: str
    name: str
    expiry: int
    third_party: ThirdParty
    active: bool


@strawberry.type
class Account(BaseModel):
    pubkey: str
    address: str
    created_at: int
    nonce: Optional[str]
    jwt: Optional[str]


@strawberry.type
class DocumentBlob(BaseModel):
    data: str


@strawberry.type
class Document(BaseModel):
    cid: str
    name: str
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


@strawberry.type
class Webhook(BaseModel):
    uuid: str
    party: ThirdParty
    endpoint: str
    type: str
    active: int


@strawberry.type
class ThirdPartyAccount(BaseModel):
    account: Account
    third_party: ThirdParty


@strawberry.type
class AccessRequest(BaseModel):
    uuid: str
    third_party: ThirdParty
    account: Account
    document: Document
    status: str
    callback_url: str
    callback_data: Dict[str, str]


class DummyRoute(APIRoute):
    def get_route_handler(self) -> Callable:
        original_route_handler = super().get_route_handler()

        async def dummy_route_handler(request: Request) -> Response:
            response = await original_route_handler
            return response

        return dummy_route_handler


app = FastAPI()


origins = [
    "http://0.0.0.0:3000",
    "http://0.0.0.0",
    "http://127.0.0.1:3000",
    "http://127.0.0.1",
    "http://host.docker.internal:3000",
    "http://host.docker.internal",
    "http://localhost:3000",
    "http://localhost",
]


# https://github.com/tiangolo/fastapi/issues/1174
public_routes = {
    blake3_(HttpMethod.GET.value + "/"),
    blake3_(HttpMethod.POST.value + "/account"),
    blake3_(HttpMethod.POST.value + "/account/auth/verify"),
    blake3_(HttpMethod.POST.value + "/account/login"),
    blake3_(HttpMethod.POST.value + "/account/mnemonic"),
    blake3_(HttpMethod.POST.value + "/account/third-party"),
    blake3_(HttpMethod.POST.value + "/graphql"),
    blake3_(HttpMethod.POST.value + "/third-party"),
}


def _is_public_route(method: str, path: str) -> bool:
    return blake3_(method + path) in public_routes


class JwtMiddlware(BaseHTTPMiddleware):
    async def dispatch(self, request: Request, call_next) -> Request:
        if _is_public_route(request.method, request.url.path):
            response = await call_next(request)
            return response

        try:
            token: str = request.headers["Authorization"]
        except KeyError as err:
            return JSONResponse(
                content={"details": "Unauthorized"},
                status_code=status.HTTP_401_UNAUTHORIZED,
            )

        if token.startswith("Bearer"):
            token = token[7:]

        payload: str
        try:
            payload = jwt.decode(
                token, BitsyConfig.jwt_secret, algorithms=[BitsyConfig.jwt_algo]
            )
        except Exception as err:
            return JSONResponse(
                content={"details": "Invalid JWT"},
                status_code=status.HTTP_400_BAD_REQUEST,
            )

        account = Model.Account.get(
            where={"address": to_checksum_address(payload["address"])},
            fail_if_not_found=True,
        )

        party = Model.ThirdPartyAccount.get(
            where={"account_address": to_checksum_address(payload["address"])}
        )
        request.state.account = account
        if party:
            request.state.party_account = party
        response = await call_next(request)
        return response


class CatchAllMiddleware(BaseHTTPMiddleware):
    async def dispatch(self, request: Request, call_next) -> Request:
        try:
            return await call_next(request)
        except Exception as err:
            logger.error(str(err))
            return JSONResponse(
                content={"success": False, "details": "Server error"},
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            )


# IMPORTANT: Routes have same name as the use-case they serve, but are prefixed with 'route_'


@app.get("/")
def route_index():
    return "Welcome to Bitsy!"


@app.post("/third-party")
async def route_create_third_party(request: Request):
    body = await request.json()
    party: ThirdParty = create_third_party(body.get("name"))
    return party


@app.post("/account/third-party")
async def route_create_third_party(request: Request):
    body = await request.json()
    pubkey = recover_pubkey_from_compressed_hex(body["pubkey"])
    if pubkey.to_checksum_address() != to_checksum_address(body["address"]):
        raise RequestError(
            "Address dervied from the public key recovered from this mnemonic, and the address included in this request do not match. '{}' != '{}'".format(
                pubkey.to_address(), to_checksum_address(body["address"])
            )
        )
    party: ThirdParty = create_third_party_account(pubkey)
    return party


@app.post("/document")
async def route_create_document(request: Request):
    body = await request.json()
    doc: Document = create_document_for_account_id(
        body["name"],
        request.state.account.address,
        encode(body["data"], Encoding.UTF8),
    )
    return doc


@app.post("/access-token")
async def route_new_access_token_for_third_party(request: Request):
    body = await request.json()
    name = body.get("name")
    token: AccessToken = create_access_token_for_third_party_id(
        request.state.party_account.party.uuid, name
    )
    return token


@app.put("/access-token")
async def route_toggle_third_party_token(request: Request):
    body = await request.json()
    return toggle_third_party_token(
        third_party_id=request.state.party_account.party.uuid,
        token_id=body["uuid"],
    )


@app.get("/access-token")
async def route_get_access_tokens_for_third_party(request: Request):
    return Model.AccessToken.get_many(
        where={"third_party_id": request.state.party_account.party.uuid}
    )


@app.delete("/access-token")
async def route_delete_third_party_access_token(request: Request):
    body = await request.json()
    return delete_third_party_access_token_id(
        request.state.party_account.party.uuid, body["uuid"]
    )


@app.post("/account")
async def route_create_account(request: Request):
    body = await request.json()
    pubkey = recover_pubkey_from_compressed_hex(body["pubkey"])
    if pubkey.to_checksum_address() != to_checksum_address(body["address"]):
        raise RequestError(
            "Address dervied from the public key recovered from this mnemonic, and the address included in this request do not match. '{}' != '{}'".format(
                pubkey.to_address(), to_checksum_address(body["address"])
            )
        )
    account: Account = create_account(pubkey)
    return account


@app.post("/account/auth/verify")
async def route_verify_nonce_signature(request: Request):
    body = await request.json()
    account = verify_nonce_signature(
        body["nonce"],
        body["signature"],
        body["input"],
        to_checksum_address(body["address"]),
    )

    if not account:
        return JSONResponse(
            content={"sucess": False},
            status_code=status.HTTP_200_OK,
        )
    return account


@app.post("/permission")
async def route_grant_perms_on_doc_for_third_party(request: Request):

    body = await request.json()

    doc_id = body.get("document_cid")

    perms: Permission = []
    for party_id in body["party_ids"]:
        perm: Permission
        if doc_id is None:
            perm: Permission = grant_perms_on_new_doc_for_third_party_id(
                PermissionKey(body["permission_key"]),
                party_id,
                body["name"],
                encode(body["data"], Encoding.UTF8),
                request.state.account.address,
            )
        else:
            perm: Permission = grant_perms_on_existing_doc_for_third_party_id(
                PermissionKey(body["permission_key"]),
                party_id,
                request.state.account.address,
                body["document_cid"],
            )
        perms.append(perm)
    return perms


@app.delete("/permission")
async def route_revoke_perms_on_existing_doc_for_third_party_id(
    request: Request,
):
    body = await request.json()
    return revoke_perms_on_existing_doc_for_third_party_id(
        body["party_id"],
        body["document_cid"],
        request.state.account.address,
        PermissionKey[body["permission_key"]],
    )


@app.post("/third-party-access-doc")
async def route_third_party_access_document_id(request: Request):

    body = await request.json()

    third_party_id = body["third_party_id"]
    document_cid = body["document_cid"]

    document = third_party_access_document_id(
        third_party_id,
        document_cid,
        request.state.account.address,
    )

    return document


@app.get("/account-stats")
async def route_get_stats_for_account_id(request: Request):
    return get_stats_for_account_id(request.state.account.address)


@app.get("/setting")
async def route_get_settings_for_account(request: Request):
    account_address = (
        request.state.party_account.account.address
        if hasattr(request.state, "party_account")
        else request.state.account.address
    )
    return get_settings_for_account(address=account_address)


@app.post("/setting")
async def route_add_setting_to_account_id(request: Request):
    body = await request.json()

    return add_setting_to_account_id(
        account_address=request.state.account.address,
        key=SettingKey[body["key"]],
        value=body["value"],
    )


@app.put("/setting")
async def route_toggle_account_setting_id(request: Request):
    body = await request.json()

    return toggle_account_setting_id(
        account_address=request.state.account.address,
        key=SettingKey(body["key"]),
    )


@app.post("/webhook")
async def route_create_third_party_webhook_id(request: Request):
    body = await request.json()

    return create_third_party_webhook_id(
        party_id=request.state.party_account.party.uuid,
        endpoint=body["endpoint"],
        type=WebhookType[body["type"]],
        name=body["name"],
        active=str_bool_to_int(body.get("active", False), True),
    )


@app.get("/webhook")
async def route_get_webhooks_for_third_party(request: Request):
    return Model.Webhook.get_many(
        where={"third_party_id": request.state.party_account.party.uuid}
    )


@app.put("/webhook")
async def route_toggle_third_party_webhook(request: Request):
    body = await request.json()
    return toggle_third_party_webhook(
        third_party_id=request.state.party_account.party.uuid,
        webhook_id=body["uuid"],
    )


@app.delete("/webhook")
async def route_delete_third_party_webhook_id(request: Request):
    body = await request.json()
    return delete_third_party_webhook(
        request.state.party_account.party.uuid, body["uuid"]
    )


@app.post("/access-request")
async def route_create_third_party_access_request_id(request: Request):
    body = await request.json()
    return create_third_party_access_request_id(
        request.state.party_account.party.uuid,
        body["account_address"],
        body["document_cid"],
        body["callback_url"],
        body["callback_data"],
    )


@app.get("/acccount/access-request")
async def route_create_third_party_access_request_id(request: Request):
    from ._models import AccessRequest

    request = AccessRequest.get_many(
        where={"account_address": request.state.account.address}
    )
    return request


@app.get("/access-request")
async def route_create_third_party_access_request_id(request: Request):
    from ._models import AccessRequest

    body = await request.json()
    request = AccessRequest.get(where={"uuid": body["access_request_id"]})
    return request


router = APIRouter(route_class=DummyRoute)

app.include_router(router)
app.add_middleware(JwtMiddlware)
if not is_pytest_session():
    app.add_middleware(CatchAllMiddleware)
app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
