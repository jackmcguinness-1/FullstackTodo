the structure of the database is handled by sqlx migrations.
commands to create tables can be stored as .sql scripts.
any updates to the tables can also be stored as .sql scripts including removing tables
this allows sqlx to create a consistent structure for the database by using these migrations in order

to create a new migration run:
sqlx migrate add {migration name}
for example
sqlx migrate add create_users


APP FLOW:
1. website loads -> GET /auth/me