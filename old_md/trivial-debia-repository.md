Title: Creating a Trivial Debian Repository
Tags: Debian, repositories, repo, repository, Jessie, nginx
Date: 2014-12-26 19:15
Modified: 2014-12-26 19:15
Summary: How to create a basic Trivial Debian Repository for local packages not located in any other repo

[TOC]

I use Xenserver on one of my servers for virtualization and I run quite many Debian installations on it. For every single one, I have to install the xe-guest-utilities which by default is only available on a CD/ISO. Since the CD has a .deb package, I decided to just create a local repo on one of the servers.

This is what the Debian docs call a Trivial repo, with just one binary folder, a Release file, and a Packages.gz. The repo in this post will not be signed, and therefore, be unauthenticated.

Also, a webserver is a precondition. I use Nginx.

## Structure
The structure of the repo folder will be like this:

    :::
    repo
    ├── binary
    │   ├── Packages.gz
    │   └── xe-guest-utilities_6.4.96-1360_amd64.deb
    └── Release

The first thing we have to is to create a repo folder on the webserver and then a binary folder inside.

    :::
    mkdir -p repo/binary

## Creating index files
In the repository, we need an index file of what it contains. We will generate this with the command dpkg-scanpackages. This command is not standard so we will need to install dpkg-dev.

    :::
    apt-get install dpkg-dev

Now, we can create the index file:

    :::
    dpkg-scanpackages binary /dev/null | gzip -9c > binary/Packages.gz

## Release file
Last we need a Release file on this format:

    :::
    Archive: archive
    Component: component
    Origin: YourCompany
    Label: YourCompany Debian repository
    Architecture: architecture

The explanation of the different keys from the Debian docs are good so I embedded them below.

**Archive**

The name of the distribution of Debian the packages in this directory belong to (or are designed for), i.e. stable, testing or unstable.

**Component**

The component of the packages in the directory, for example main, non-free, or contrib.

**Origin**

The name of who made the packages.

**Label**

Some label adequate for the packages or for your repository. Use your fantasy.

**Architecture**

The architecture of the packages in this directory, such as i386, Sparc or source.

## Adding the repository
Add the repo to source.list like this:

    :::
    deb http://ip_or_address/repo binary/
