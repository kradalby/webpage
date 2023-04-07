---
title: PostgreSQL usage
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: database, postgres, postgresql, sql
summary: Basic PostgreSQL commands that i use.
---

## Use

My preferred way to get access:

    :::bash
    su postgres -
    psql template1

    OR

    create/drop commands


**Connect to a database**

    :::bash
    \c databasename

## Information

**List attributes and settings for table**

    :::bash
    \d+ tablename

**List databases**

    :::bash
    \list

**List tables**

    :::bash
    \dt

**List users**

    :::bash
    \du


## Create and Drop

### Users

From shell:

    :::bash
    createuser username

    dropuser username

From psql:

    :::bash
    CREATE USER tom WITH PASSWORD 'myPassword';

### Database

From shell:

    :::bash
    createdb dbname

    dropdb dbname

From psql:

    :::bash
    CREATE DATABASE dbname;

#### Grant

Grant access for user on database with psql:

    :::bash
    GRANT ALL PRIVILEGES ON DATABASE dbname TO username;


We can also create a database with the user as owner:

    :::bash
    createdb --owner user dbname
