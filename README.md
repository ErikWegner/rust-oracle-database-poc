# rust-oracle-database-poc

Access Oracle database from Rust application

Inspired by https://blogs.oracle.com/timesten/post/using-rust-with-oracle-databases

## Driver setup

See https://oracle.github.io/odpi/doc/installation.html#linux

```bash
# To be the root user, use sudo -i
apt install libaio1
mkdir -p /opt/oracle
cd /opt/oracle
wget https://download.oracle.com/otn_software/linux/instantclient/218000/instantclient-basiclite-linux.x64-21.8.0.0.0dbru.zip
unzip instantclient-basiclite-linux.x64-21.8.0.0.0dbru.zip
echo /opt/oracle/instantclient_21_8 > /etc/ld.so.conf.d/oracle-instantclient.conf
ldconfig
```

## Database setup

```
docker exec -it rust-oracle-database_devcontainer-db-1 /bin/bash

sqlplus / as sysdba

ALTER SESSION SET CONTAINER = XEPDB1;
CREATE USER rustapp IDENTIFIED BY kuMxNAZ2xYSh CONTAINER=CURRENT;
GRANT ALL PRIVILEGES TO rustapp CONTAINER=CURRENT;

exit

sqlplus rustapp/kuMxNAZ2xYSh@XEPDB1

CREATE TABLE Persons (
    PersonID int,
    LastName varchar(255),
    FirstName varchar(255),
    Address varchar(255),
    City varchar(255)
);

INSERT INTO Persons (PersonID, LastName, FirstName, Address, City) VALUES (1, 'Wegner', 'Erik', 'Blumenstraße 1', 'Neustadt');
INSERT INTO Persons (PersonID, LastName) VALUES (2, '++anonymized');
```

## Run

To run the application: `SVC_NAME=//db/XEPDB1 cargo run`
