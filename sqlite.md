# SQLite

## Session table
```shell
CREATE TABLE session (
  id VARCHAR(32) NOT NULL,
  user_id VARCHAR(32) NOT NULL,
  created_at NUMBER(8) NOT NULL
);
```