---
title: Samba setup
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: samba, smb, smbd, smb2
summary: Description on how to setup Samba.
---


This guide is written for Samba3 as that is what I have on my system.
Samba4 does not need a system user for every samba user.

## Installation

    :::bash
    apt-get install samba


## Setup

The sambaconfiguration file is located in /etc/samba/smb.conf

An example block for a share:

    :::bash
    [Storage]
        path = /storage
        browsable = yes
        guest ok = no
        read only = yes
        write list = kradalby
        valid users = kradalby meepo

Note: This block contains overlapping settings, do not, for example, use writable with write list.


### Adding users

To add a user in samba3:

    :::bash
    smbpasswd -a username

To add a user in samba4:

    :::bash
    samba-tool user add username


## Miscellaneous

### Follow symlinks
Samba is configured by default not to follow symlinks because of security. If you want to activate it still, add the following to the [global] section.

    :::bash
    follow symlinks = yes
    wide links = yes
    unix extensions = no
