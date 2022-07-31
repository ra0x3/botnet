from bitsy._db import *
from bitsy._models import *
from bitsy._utils import *
from bitsy._config import *


class TestIndex:
    def test_index_create_fragment_returns_proper_fragment(self):
        index = Index("bar", unique=True, table_name="foo", column_name="baz")
        assert index.create_fragment() == "CREATE UNIQUE INDEX IF NOT EXISTS bar ON foo(baz)"


class TestColumn:
    def test_column_create_fragment_returns_frag(self):
        column = Column("foo", ColumnType.Null)
        assert column.create_fragment() == "  foo Null,\n"

    def test_column_create_fragment_returns_frag_for_primarykey(self):
        column = Column("foo", ColumnType.Null, primary_key=True)
        assert column.create_fragment() == "  foo Null PRIMARY KEY,\n"

    def test_column_create_fragment_returns_frag_for_autoincrement(self):
        column = Column("foo", ColumnType.Null, primary_key=True)
        assert column.create_fragment() == "  foo Null PRIMARY KEY,\n"

    def test_can_add_foreign_key_to_column(self):
        column = Column(
            "foo",
            ColumnType.Integer,
            foreign_key=ForeignKey("foo", reference=ForeignKeyReference("bar", "id")),
        )
        assert (
            column.create_fragment()
            == "  foo integer,\n  FOREIGN KEY(foo) REFERENCES bar(id) ON DELETE NO ACTION,\n"
        )


class TestTable:
    def setup_method(self):
        self.conn = BitsyConfig.connection

    def test_can_create_basic_table(self):
        table = Table(
            "foo",
            columns=[
                Column(
                    "id",
                    ColumnType.Integer,
                    primary_key=True,
                ),
                Column("uuid", ColumnType.Varchar),
                Column("bytes", ColumnType.Bytea),
            ],
            conn=self.conn,
        )

        stmnt = table._create_stmnt()
        assert (
            stmnt
            == """CREATE TABLE IF NOT EXISTS foo (
  id integer PRIMARY KEY,
  uuid varchar(255),
  bytes bytea
);"""
        )

        table.create()

    def test_can_create_table_with_constraints(self):
        table = Table(
            "fools",
            columns=[
                Column(
                    "id",
                    ColumnType.Integer,
                    serial=True,
                ),
                Column("name", ColumnType.Varchar),
            ],
            conn=self.conn,
        )
        docs = Table(
            "zeros",
            columns=[
                Column(
                    "id",
                    ColumnType.Serial,
                    primary_key=True,
                ),
                Column("uuid", ColumnType.Varchar),
                Column(
                    "fool_id",
                    ColumnType.Integer,
                    foreign_key=ForeignKey(
                        "fool_id",
                        reference=ForeignKeyReference("fools", "id"),
                    ),
                ),
            ],
            conn=self.conn,
        )

        stmnt = docs._create_stmnt()
        assert (
            stmnt
            == """CREATE TABLE IF NOT EXISTS zeros (
  id serial PRIMARY KEY,
  uuid varchar(255),
  fool_id integer,
  FOREIGN KEY(fool_id) REFERENCES fools(id) ON DELETE NO ACTION
);"""
        )

        table.create()


class TestDabase:
    def test_foo(self):
        assert 1 == 2 - 1
