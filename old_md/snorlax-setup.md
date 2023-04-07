---
title: Setup documentation for Snorlax
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: backup
summary: Documentation covering the setup of Snorlax.
---

[TOC]

## Hard drives

### Overview

#### SATA2

    Brand: Kingston
    Type: SSD
    Size: 120gb
    Filesystem: EXT4
    Mountpoint: /

#### SATA3

    Brand: Western Digital VelociRaptor
    Type: HDD
    Size: 160gb
    Filesystem: EXT4
    Mountpoint: /media/160g

#### SATA5

    Brand: Seagate Barracuda
    Type: HDD
    Size: 4tb
    Filesystem: btrfs
    Raid: Storage

#### SATA6

    Brand: Seagate Barracuda
    Type: HDD
    Size: 4tb
    Filesystem: btrfs
    Raid: Storage

### Raids

#### Storage

    Disks: SATA5, SATA6
    Size: 4tb
    Type: RAID1
    Mountpoint: /storage


## Backup user

### Creating the user

    :::bash
    useradd -b /storage/backup -d /storage/backup -M -s /bin/bash meepo
    chown -R meepo.meepo /storage/backup

I do not set a password since I am only going to use keys for this account.


## Jottacloud


## Samba
Example samba config files are available on GitHub.
