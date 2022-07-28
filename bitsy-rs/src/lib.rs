#[macro_use]
extern crate diesel;
extern crate diesel_codegen;


pub mod bitsy_diesel {
    pub mod prelude {
        pub use diesel::prelude::*;
    }

    pub mod sql_types {
        pub use diesel::sql_types::*;
    }
    pub mod result {
        pub use diesel::result::{DatabaseErrorKind, Error as ResultError, QueryResult};
    }
}

pub mod database {

    use anyhow::Result;
    use core::ops::Deref;
    use diesel::{prelude::PgConnection, sql_query, Connection, RunQueryDsl};
    use r2d2_diesel::ConnectionManager;

    #[allow(unused)]
    type PgConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
    pub struct ConnWrapper(PgConnection);

    impl Deref for ConnWrapper {
        type Target = PgConnection;

        fn deref(&self) -> &PgConnection {
            &self.0
        }
    }

    impl std::fmt::Debug for ConnWrapper {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "ConnWrapper(...)")
        }
    }

    #[derive(Debug)]
    pub struct Database {
        pub conn: ConnWrapper,
    }

    impl Database {
        pub fn new(db_conn: &str) -> Result<Database> {
            let conn = ConnWrapper(PgConnection::establish(db_conn)?);

            Ok(Database { conn })
        }

        pub fn start_transaction(&self) -> Result<usize> {
            Ok(sql_query("BEGIN").execute(&*self.conn)?)
        }

        pub fn commit_transaction(&self) -> Result<usize> {
            Ok(sql_query("COMMIT").execute(&*self.conn)?)
        }

        pub fn revert_transaction(&self) -> Result<usize> {
            Ok(sql_query("ROLLBACK").execute(&*self.conn)?)
        }
    }
}

pub mod tables {

    use diesel::table;

    table! {
        documents (cid) {
            cid -> VarChar,
            blob -> Bytea,
            account_address -> VarChar,
            key_image -> VarChar,
        }
    }

    table! {
            accounts (address) {
            pubkey -> VarChar,
            address -> VarChar,
            created_at -> Integer,
            nonce -> Nullable<VarChar>,
        }
    }
}

pub mod models {

    use super::tables::{accounts, documents};
    use diesel::{
        Insertable, Queryable, QueryableByName,
    };
    use serde::{Deserialize, Serialize};

    #[derive(
        Queryable, QueryableByName, Insertable, Debug, Associations, Serialize, Deserialize,
    )]
    #[table_name = "accounts"]
    pub struct Account {
        pub pubkey: String,
        pub address: String,
        pub created_at: i32,
        pub nonce: Option<String>,
    }

    #[derive(
        Queryable, QueryableByName, Insertable, Debug, Associations, Serialize, Deserialize,
    )]
    #[table_name = "documents"]
    #[belongs_to(parent = Account<'_>)]
    pub struct Document {
        pub cid: String,
        pub blob: Vec<u8>,
        pub account_address: String,
        pub key_image: String,
    }
}

pub mod web {}
