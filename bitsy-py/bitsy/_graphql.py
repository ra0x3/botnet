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
    def access_token(
        self, uuid: Optional[str] = None, name: Optional[str] = None
    ) -> AccessToken:
        return Model.AccessToken.get(
            where=remove_empty_keys({"uuid": uuid, "name": name})
        )

    @strawberry.field
    def third_party(
        self, uuid: Optional[str] = None, name: Optional[str] = None
    ) -> ThirdParty:
        return Model.ThirdParty.get(
            where=remove_empty_keys({"uuid": uuid, "name": name})
        )

    @strawberry.field
    def account(self, address: str) -> Account:
        return Model.Account.get(where={"address": address})

    @strawberry.field
    def document(
        self, cid: Optional[str], account_address: Optional[str] = None
    ) -> Document:
        return Model.Document.get(
            where=remove_empty_keys(
                {"cid": cid, "account_address": account_address}
            )
        )

    @strawberry.field
    def permission(
        self,
        uuid: Optional[str] = None,
        document_cid: Optional[str] = None,
        value: Optional[int] = None,
        account_address: Optional[str] = None,
        third_party_id: Optional[str] = None,
    ) -> Permission:
        return Model.Permission.get(
            where=remove_empty_keys(
                {
                    "uuid": uuid,
                    "document_cid": document_cid,
                    "value": value,
                    "account_address": account_address,
                    "third_party_id": third_party_id,
                }
            )
        )


schema = strawberry.Schema(
    Query, config=StrawberryConfig(auto_camel_case=False)
)

graphql_app = GraphQLRouter(schema)
