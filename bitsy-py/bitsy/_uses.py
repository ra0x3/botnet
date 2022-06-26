from ._t import *
from ._db import *
from ._models import *
from ._crypto import *
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
def create_third_party() -> ThirdParty:
    party = ThirdParty(uuid4())
    party.save()
    return party


def create_access_token_for_third_party_id(party_id: str) -> AccessToken:
    party = ThirdParty.get(where={"uuid": party_id})
    return create_access_token_for_third_party(party)


@use_case
def create_access_token_for_third_party(party: ThirdParty) -> AccessToken:
    token = create_access_token()
    _ = party.update(
        update={"access_token": token.uuid}, where={"uuid": party.uuid}
    )

    return token


@use_case
def create_account(pubkey: str) -> Account:
    account = Account(blake3_sha256(pubkey))
    account.save()
    return account


@use_case
def create_document_for_account(data: str, account: Account) -> Document:
    cid = blake3_sha256(uuid4())
    document = Document(cid, DocumentBlob(data), account)
    document.save()
    return document


def create_document_for_account_id(account_pubkey: str, data: str) -> Document:
    account = Account.get(where={"pubkey": account_pubkey})
    return create_document_for_account(data, account)


@use_case
def grant_perms_on_new_doc_for_third_party(
    key: PermKey,
    party: ThirdParty,
    data: str,
    account: Account,
    ttl: int,
) -> Permission:
    cid = blake3_sha256(uuid4())

    document = Document(cid, DocumentBlob(data), account)
    document.save()
    permission = Permission(
        uuid4(),
        key,
        document,
        1,
        account,
        party,
        ttl,
    )
    permission.save()
    return permission


def grant_perms_on_new_doc_for_third_party_id(
    key: PermKey,
    party_id: str,
    data: str,
    account_pubkey: str,
    ttl: int = -1,
) -> Permission:
    account = Account.get(where={"pubkey": account_pubkey})
    party = ThirdParty.get(where={"uuid": party_id})
    return grant_perms_on_new_doc_for_third_party(
        key, party, data, account, ttl
    )


@use_case
def grant_perms_on_existing_doc_for_third_party(
    key: PermKey,
    party: ThirdParty,
    account: Account,
    document: Document,
    ttl: int = -1,
) -> Permission:
    perm = Permission(uuid4(), key, document, 1, account, party, ttl)
    perm.save()
    return perm


def grant_perms_on_existing_doc_for_third_party_id(
    key: PermKey,
    party_id: str,
    account_pubkey: str,
    document_id: str,
    ttl: int = -1,
) -> Permission:
    account = Account.get(where={"pubkey": account_pubkey})
    party = ThirdParty.get(where={"uuid": party_id})
    document = Document.get(where={"cid": document_id})
    return grant_perms_on_existing_doc_for_third_party(
        key, party, account, document, ttl
    )


@use_case
def new_access_token_for_third_party(party: ThirdParty) -> AccessToken:
    token = create_access_token()
    party = ThirdParty.update(
        update={"access_token": token.uuid}, where={"uuid": party.uuid}
    )
    return token


@use_case
def revoke_third_party_perms_on_account(
    key: PermKey, account: Account, party: ThirdParty
) -> Permission:
    perm = Permission.update(
        update={"value": 0},
        where={
            "key": key.value,
            "account_pubkey": account.pubkey,
            "third_party_id": party.uuid,
        },
    )
    return perm


@use_case
def list_all_third_party_perms_for_account(
    account: Account,
) -> List[Permission]:
    perms = Permission.get_many(where={"account_pubkey": account.pubkey})
    return perms


@use_case
def toggle_setting_for_account(account: Account) -> Setting:
    pass


@use_case
def register_new_account(privkey: PublicKey) -> Account:
    account = create_account()
    pass
