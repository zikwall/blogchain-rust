<div align="center">
  <h1>Blogсhain Rust</h1>
</div>

### Requirements

#### Windows

- `Microsoft Visual C++ 2015-2019 Redistributable`
- `lmysqlclient` or another database packages (pg, sqllite)

#### Linux

- `build-essentials`
- `libmysqlclient-dev` or another database packages (pg, sqllite)

#### Database

- `$ mysql> CREATE USER 'blogchain'@'localhost' IDENTIFIED BY '123456';`
- `$ mysql> GRANT ALL PRIVILEGES ON blogchain . * TO 'blogchain'@'localhost';`
- `$ mysql> FLUSH PRIVILEGES;`

### Development

- `$ rustup default nightly | rustup override set nightly`
- `$ cargo install diesel_cli --no-default-features --features "mysql"`
- `$ echo DATABASE_URL=DATABASE_URL=mysql://blogchain:123456@localhost/blogchain > .env`
- `$ cargo run`