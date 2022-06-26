import pathlib
from bitsy._db import *
from bitsy._models import *
from bitsy._utils import *


class TestIndex:
    def test_index_create_fragment_returns_proper_fragment(self):
        index = Index("bar", unique=True, table_name="foo", column_name="baz")
        assert (
            index.create_fragment()
            == "CREATE UNIQUE INDEX IF NOT EXISTS bar ON foo(baz)"
        )


class TestColumn:
    def test_column_create_fragment_returns_frag(self):
        column = Column("foo", ColumnType.Null)
        assert column.create_fragment() == "  foo Null,\n"

    def test_column_create_fragment_returns_frag_for_primarykey(self):
        column = Column("foo", ColumnType.Null, primary_key=True)
        assert column.create_fragment() == "  foo Null PRIMARY KEY,\n"

    def test_column_create_fragment_returns_frag_for_autoincrement(self):
        column = Column("foo", ColumnType.Null, auto_increment=True)
        assert column.create_fragment() == "  foo Null AUTOINCREMENT,\n"

    def test_can_add_foreign_key_to_column(self):
        column = Column(
            "foo",
            ColumnType.Integer,
            foreign_key=ForeignKey(
                "foo", reference=ForeignKeyReference("bar", "id")
            ),
        )
        assert (
            column.create_fragment()
            == "  foo Integer,\n  FOREIGN KEY(foo) REFERENCES bar(id) ON DELETE NO ACTION,\n"
        )


class TestTable:
    def setup_method(self):
        self.conn = create_test_db("test_db.db")

    def test_can_create_basic_table(self):
        table = Table(
            "foo",
            columns=[
                Column(
                    "id",
                    ColumnType.Integer,
                    auto_increment=True,
                    primary_key=True,
                ),
                Column("uuid", ColumnType.Text),
                Column("bytes", ColumnType.Blob),
            ],
            conn=self.conn,
        )

        stmnt = table._create_stmnt()
        assert (
            stmnt
            == """CREATE TABLE IF NOT EXISTS foo (
  id Integer PRIMARY KEY AUTOINCREMENT,
  uuid Text,
  bytes Blob
);"""
        )

        table.create()

        results = list(self.conn.execute("SELECT * FROM foo;"))
        assert isinstance(results, list)

    def test_can_create_table_with_constraints(self):
        table = Table(
            "accounts",
            columns=[
                Column(
                    "id",
                    ColumnType.Integer,
                    auto_increment=True,
                    primary_key=True,
                ),
                Column("name", ColumnType.Text),
            ],
            conn=self.conn,
        )
        docs = Table(
            "documents",
            columns=[
                Column(
                    "id",
                    ColumnType.Integer,
                    auto_increment=True,
                    primary_key=True,
                ),
                Column("uuid", ColumnType.Text),
                Column(
                    "account_id",
                    ColumnType.Integer,
                    foreign_key=ForeignKey(
                        "account_id",
                        reference=ForeignKeyReference("accounts", "id"),
                    ),
                ),
            ],
            conn=self.conn,
        )

        stmnt = docs._create_stmnt()
        assert (
            stmnt
            == """CREATE TABLE IF NOT EXISTS documents (
  id Integer PRIMARY KEY AUTOINCREMENT,
  uuid Text,
  account_id Integer,
  FOREIGN KEY(account_id) REFERENCES accounts(id) ON DELETE NO ACTION
);"""
        )

        table.create()

    def teardown_method(self):
        remove_file(["test_db.db"])


class TestDabase:
    def test_foo(self):
        assert 1 == 2 - 1
