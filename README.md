# plrust-lib-example

Install required pgrx specific version

```
cargo install --version 0.11.0 --locked cargo-pgrx
```

Set PGRX_HOME to avoid cargo build error

```
export PGRX_HOME="${HOME}/.cargo/"
```

> fish

```
set -x PGRX_HOME $HOME/.cargo/

```

Run pgrx init 

```
cargo pgrx init --pg16 ~/Desktop/tc/trustify/.trustify/postgres/16.3.0/bin/pg_config
```

Build

```
cargo build
```
