import psycopg
import pytest


@pytest.fixture
def db_conn(database_url):
    with psycopg.connect(database_url) as conn:
        yield conn
