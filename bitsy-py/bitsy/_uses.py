import uuid
import enum

from eth_account import Account

from ._t import *
from ._db import *
from ._models import *
from ._utils import *


def use_case(func):
    def _use_case(*args, **kwargs):
        result = func(*args, **kwargs)
        return result

    return _use_case


@use_case
def create_access_token() -> AccessToken:
    token = AccessToken(uuid4())
    token.save()
    return token


@use_case
def create_third_party(token: AccessToken) -> ThirdParty:
    party = ThirdParty(uuid4(), token.uuid)
    party.save()
    return party


# @use_case
# def create_account(address: str) -> AccountEntity:
#     account = AccountEntity(uuid4())
#     account.save()
#     return account


# @use_case
# def add_third_party_perms_to_acct(
#     third_party_id: str, key: str, value: int, account_id: str
# ) -> PermissionEntity:
#     permission = PermissionEntity(
#         uuid4(), PermKey.Other, 1, account_id, third_party_id
#     )
#     permission.save()
#     return permission


# @use_case
# def create_document(data: bytes, account: AccountEntity) -> DocumentEntity:
#     cid = blake(uuid4())
#     document = DocumentEntity(cid, data, account.uuid)
#     document.save()
#     return document
