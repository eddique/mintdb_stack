<p align="center">
    <img width="400" src="./img/logo-light.png#gh-light-mode-only" alt="mintDB Logo">
    <img width="400" src="./img/logo.png#gh-dark-mode-only" alt="mintDB Logo">
</p>
<h2 align="center">An Open Source Vector Database</h2>
<p align="center">
    <img src="https://img.shields.io/badge/version-0.1.0-10d99d">
    <img src="https://img.shields.io/docker/pulls/eddique/mintdb-stack">
    <img src="https://img.shields.io/badge/built_with-Rust-dca282.svg">
    <img src="https://img.shields.io/badge/license-MIT-critical">
    <a href="https://www.linkedin.com/in/eric-rodriguez-3a402811b/"><img src="https://img.shields.io/badge/linkedIn-connect-4777AF"></a>
</p>

## Getting started

### Binary
MintDB Stack Server
```sh
mintdb-stack start path/to/data
```

MintDB Stack Repl
```sh
mintdb-stack repl path/to/data
```

### Docker Compose
```yaml
services:
  mintdb:
    image: eddique/mintdb-stack:latest --log-level info
    ports:
      - "3000:3000"
    command: start /data/mint.db
    volumes:
      - mintdb-data:/data/
volumes:
  mintdb-data:
    driver: local
```
### Helm
```sh
helm repo add mintdb https://eddique.github.io/helm-mintdb-stack/
helm install mintdb-stack mintdb/mintdb-stack
```

### Stateful Set
```sh
cargo run -- start mint.db --log-level info --pod mintdb-0
```
## API

### POST /sql
Executes a SQL-like statement based on the provided JSON payload.

Request Body:
```json
{
  "stmt": "statement_type",
  "tb": "table_name",
  "doc": "document_id",
  "key": "document_key",
  "data": "data_object",
  "query": ["query_string"],
  "embedding": [0.1, 0.2, ...],
  "top_n": 5
}
```
Response:
The response is a JSON object containing the result of the executed statement.

Example Usage
```json
Select Statement
{
  "stmt": "select",
  "tb": "users",
  "doc": "user123"
}
```

Insert Statement
```json
{
  "stmt": "insert",
  "tb": "users",
  "data": {
    "name": "John Doe",
    "email": "john@example.com"
  },
  "doc": "user123"
}
```
Delete Statement
```json
{
  "stmt": "delete",
  "tb": "users",
  "doc": "user123"
}
```
Query Statement
```json
{
  "stmt": "query",
  "tb": "documents",
  "embedding": [0.1, 0.2, 0.3, 0.4],
  "top_n": 10
}
```
Error Handling
The API returns appropriate error messages for missing or invalid fields. The error format is as follows:
```json
{
  "ok": false,
  "error": "Error message"
}
```

### GET /health

Health check endpoint. Will respond with 200 status code and json body
```json
{
  "ok": true
}
```

## SQL Statements

### 1. Select
Fetches data from the specified table.

#### Required Fields:
`stmt`: "select"
`tb`: Table name
#### Optional Fields:
`doc`: Document ID (if specified, fetches a single document)
`query`: List of query strings (if specified, performs a query)

### 2. Insert
Inserts data into the specified table.

#### Required Fields:
`stmt`: "insert"
`tb`: Table name
`data`: Data to be inserted

#### Optional Fields:
`doc`: Document ID (if specified, merges or inserts data with the given ID)
`key`: Key for the document (if specified, inserts data with the given key)

### 3. Delete
Deletes a document from the specified table.

#### Required Fields:
`stmt`: "delete"
`tb`: Table name
`doc`: Document ID to be deleted

### 4. Drop
Drops the specified table.

#### Required Fields:
`stmt`: "drop"
`tb`: Table name

### 5. Migrate
Creates a new collection in the datastore.

#### Required Fields:
`stmt`: "migrate"
`tb`: Table name

### 6. Query
Performs a query using embeddings.

#### Required Fields:
`stmt`: "query"
`tb`: Table name
`embedding`: List of embeddings for the query
#### Optional Fields:
top_n: Number of top results to return (default is 5)

### 7. Tables
Fetches the list of all tables in the datastore.

#### Required Fields:
`stmt`: "tables"

### 8. Count
Counts the number of documents in the specified table.

#### Required Fields:
`stmt`: "count"
`tb`: Table name

## Query Structure
The query key should be a list of query strings, where each string represents a filter condition. The query strings are parsed to create filter conditions that are applied to the data in select statements.

### Query String Format
A query string follows this format:

#### key operator value
* key: The field in the document to filter on.
* operator: The comparison operator (e.g., =, !=, >, <, >=, <=).
* value: The value to compare the field against.

#### Example Query Strings
* age > 30
* name = "John Doe"
* status != "inactive"

### Supported Operators
The following operators are supported in query strings:

* `=`: Equals
* `!=`: Not equals
* `>`: Greater than
* `<`: Less than
* `>`=: Greater than or equal to
* `<=`: Less than or equal to

## CLI

### Server
The `start` command will initialize the server

#### Command Structure
```sh
mintdb-stack start <path> [OPTIONS]
```
#### Positional Arguments
* `<path>`: Positional argument Database path used for storing data (default: "mint.db").
#### Options
* `-u`, `--username` <username>: The username for the initial database root user. Only if no other root user exists.
* `-p`, `--password` <password>: The password for the initial database root user. Requires username.
* `-c`, `--cert` <crt>: Path to the SSL certificate. Requires key.
* `-k`, `--key` <key>: Path to the SSL key.
* `-l`, `--log`: Enable logging.
* `-L`, `--log-level` <log_level>: Set the logging level.

#### Example Usage

```sh
mintdb-stack start mydatabase.db -u admin -p secret -l -L debug
```

### REPL
The `repl` command starts an interactive Read-Eval-Print Loop for executing SQL-like commands.

#### Command Structure
```sh
mintdb-stack repl <path> [OPTIONS]
```

#### Positional Arguments
* `<path>`: Positional argument Database path used for storing data (default: "mint.db").

#### Options
* `-u`, `--username` <username>: The username for the initial database root user. Only if no other root user exists.
* `-p`, `--password` <password>: The password for the initial database root user. Requires username.

#### SQL Commands
The REPL supports various SQL-like commands:

##### Select Command
Fetches data from the specified table.

###### Command Structure
```sh
select [OPTIONS] <columns>...
```
###### Options
* `-t`, `--table` `<table>`: Table to query.
* `-d`, `--document` `<document>`: Document ID to fetch.
* `-q`, `--query` `<query>`: Query conditions.

###### Example Usage
```sh
select -t users -d user123
select -t users -q "age > 30" "status = 'active'"
```

##### Insert Command
Inserts data into the specified table.

###### Command Structure
```sh
insert [OPTIONS]
```
###### Options
* `-t`, `--table` `<table>`: Table to insert data into.
* `-i`, `--id` `<id>`: Document ID.
* `-k`, `--key` `<key>`: Document key.
* `-d`, `--data` `<data>`: Data to insert (in JSON format).
* `-q`, `--query` `<query>`: Query conditions.
###### Example Usage
```sh
insert -t users -d '{"name": "John Doe", "email": "john@example.com"}'
```
##### Delete Command
Deletes a document from the specified table.

###### Command Structure
```sh
delete [OPTIONS]
```
###### Options
* `-t`, `--table` `<table>`: Table to delete from.
* `-d`, `--document` `<document>`: Document ID to delete.
* `-q`, `--query` `<query>`: Query conditions.
###### Example Usage
```sh
delete -t users -d user123
```
##### Drop Command
Drops the specified table.

###### Command Structure
```sh
drop [OPTIONS]
```
###### Options
* `-t`, `--table` `<table>`: Table to drop.
###### Example Usage
```sh
drop -t users
```

##### Migrate Command
Creates a new collection in the datastore.

###### Command Structure
```sh
migrate [OPTIONS]
```
###### Options
* `-t`, `--table` `<table>`: Table to create.
###### Example Usage
```sh
migrate -t new_table
```
##### Count Command
Counts the number of documents in the specified table.

###### Command Structure
count [OPTIONS]
###### Options
* `-t`, `--table` `<table>`: Table to count documents in.
###### Example Usage
```sh
count -t users
```
##### Quit Command
Exits the REPL.
```sh
quit
```

##### Ping Command
Checks the connection to the datastore.

```sh
ping
```
