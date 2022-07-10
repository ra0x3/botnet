import eth_keys

from ._t import *
from ._db import *
from ._models import *
from ._crypto import *
from ._utils import *
from ._errors import *


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
def create_account(pubkey: PublicKey) -> Account:
    account = Account(key_image(pubkey.to_hex()))
    account.save()
    account.create_settings()
    _ = KeyStore.put_key(pubkey)
    return account


@use_case
def update_account_keys(
    prev_account_pubkey: str, new_account_pubkey: str
) -> Account:
    raise NotImplementedError


@use_case
def create_document_for_account(data: str, account: Account) -> Document:
    cid = blake3_sha256(uuid4())
    bundle = fernet_bundle()
    ciphertext = decode(
        bundle.key.encrypt(encode(data, Encoding.UTF8)), Encoding.UTF8
    )
    document = Document(cid, DocumentBlob(ciphertext), account, bundle.key_img)
    document.save()
    _ = KeyStore.put_bytes(bundle.hexkey)
    return document


def create_document_for_account_id(account_pubkey: str, data: str) -> Document:
    account = Account.get(where={"pubkey": account_pubkey})
    return create_document_for_account(data, account)


@use_case
def third_party_access_document_id(
    third_party_id: str, document_id: str, account_pubkey: str
) -> Optional[Document]:
    perm = Permission.get(
        where={
            "document_id": document_id,
            "third_party_id": third_party_id,
            "value": 1,
        }
    )

    if not perm:
        raise InvalidPermissionError(
            "Party({}) does not have Permission({}) for Document({})".format(
                third_party_id, PermissionKey.Read, document_id
            )
        )

    setting = Setting.get(
        where={
            "account_pubkey": account_pubkey,
            "key": SettingKey.BitsyVaultDeletegation.value,
            "value": 1,
        }
    )
    if not setting:
        return None

    document = perm.document

    hexkey = KeyStore.get_bytes(document.key_img)
    bundle = fernet_bundle(unhexlify(hexkey))
    plaintext = decode(
        bundle.key.decrypt(encode(document.blob.data, Encoding.UTF8)),
        Encoding.UTF8,
    )
    perm.document.set_text(plaintext)

    new_key_bytes = pbkdf2hmac_kdf(bundle.key_bytes)
    new_bundle = fernet_bundle(new_key_bytes)
    KeyStore.put_bytes(new_bundle.hexkey)
    new_blob = decode(
        new_bundle.key.encrypt(encode(document.blob.data, Encoding.UTF8)),
        Encoding.UTF8,
    )

    update_doc = Document.update(
        update={"key_image": new_bundle.key_img, "blob": new_blob},
        where={"cid": document.cid},
    )

    return update_doc


@use_case
def grant_perms_on_new_doc_for_third_party(
    key: PermissionKey,
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
    key: PermissionKey,
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
    key: PermissionKey,
    party: ThirdParty,
    account: Account,
    document: Document,
    ttl: int = -1,
) -> Permission:
    perm = Permission(uuid4(), key, document, 1, account, party, ttl)
    perm.save()
    return perm


def grant_perms_on_existing_doc_for_third_party_id(
    key: PermissionKey,
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
    key: PermissionKey, account: Account, party: ThirdParty
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
    raise NotImplementedError


@use_case
def register_new_account(privkey: PublicKey) -> Account:
    raise NotImplementedError
