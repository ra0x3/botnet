import strawberry
from strawberry.fastapi import GraphQLRouter
from strawberry.schema.config import StrawberryConfig

from ._web import *
from ._models import Model


@strawberry.type
class Query:
    @strawberry.field
    def hello(self) -> str:
        return "Hello World"

    @strawberry.field
    def access_token(self, uuid: str) -> List[AccessToken]:
        return Model.AccessToken.get_many(where={"uuid": uuid})

    @strawberry.field
    def third_party(
        self, uuid: Optional[str] = None, access_token: Optional[str] = None
    ) -> ThirdParty:
        return Model.ThirdParty.get_many(
            where=remove_empty_keys(
                {"uuid": uuid, "access_token": access_token}
            )
        )

    @strawberry.field
    def account(self, pubkey: str) -> List[Account]:
        return Model.Account.get(where={"pubkey": pubkey})

    @strawberry.field
    def permission(
        self,
        uuid: Optional[str] = None,
        document_id: Optional[str] = None,
        value: Optional[int] = None,
        account_pubkey: Optional[str] = None,
        third_party_id: Optional[str] = None,
    ) -> List[Permission]:
        return Model.Permission.get_many(
            where=remove_empty_keys(
                {
                    "uuid": uuid,
                    "document_id": document_id,
                    "value": value,
                    "account_pubkey": account_pubkey,
                    "third_party_id": third_party_id,
                }
            )
        )

    @strawberry.field
    def document(
        self, cid: Optional[str], account_pubkey: Optional[str] = None
    ) -> List[Document]:
        return Model.Document.get_many(
            where=remove_empty_keys(
                {"cid": cid, "account_pubkey": account_pubkey}
            )
        )


schema = strawberry.Schema(
    Query, config=StrawberryConfig(auto_camel_case=False)
)

graphql_app = GraphQLRouter(schema)
