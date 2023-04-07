---
title: Samba 4 AD on Debian
tags: samba, active directory, samba4, 4, ad
date: 2015-08-24 20:08
modified: 2017-01-29 22:44
summary: How to set up a Samba 4 Active Directory on Debian Linux
---

[TOC]

This is a quick guide/note on how to set up Samba 4 Active Directory on Debian Linux.

## Configuration

Edit /etc/fstab and add barrier=1 to the disk that Samba will use:

    :::
    /dev/...          /srv/samba/demo          ext4          defaults,barrier=1          1 1

Install Samba and Kerberos:

    :::
    apt-get install samba krb5-config samba-vfs-modules

Backup original configuration:

    :::
    mv /etc/samba/smb.conf /etc/samba/smb.conf.org

Provision the domain:

    :::
    samba-tool domain provision --use-rfc2307 --interactive

Example provision walkthrough:

    :::
    Realm [FAP.NO]: AD.FAP.NO
     Domain [FAP.NO]: FAP.NO
     Server Role (dc, member, standalone) [dc]: dc
     DNS backend (SAMBA_INTERNAL, BIND9_FLATFILE, BIND9_DLZ, NONE) [SAMBA_INTERNAL]: SAMBA_INTERNAL
     DNS forwarder IP address (write 'none' to disable forwarding) [192.168.1.1]: SOME_DNS_SERVER
    Administrator password: passw0rd
    Retype password: passw0rd

Rais the domain level to 2008_R2 for more functionality:

    :::
    samba-tool domain level raise --domain-level 2008_R2 --forest-level 2008_R2

Turn off password complexity:

    :::
    samba-tool domain passwordsettings set --complexity=off

Verify the level:

    :::
    samba-tool domain level show


## Multiple domain controllers

It is important that all the domain controllers use the DNS server that the "main" DC updates.

Add a record for the new server:

    :::
    samba-tool dns add mainDC ad.fap.no newDC A 10.60.0.11 -Uadministrator

Check the record on all servers:

    :::
    drill newDC.ad.fap.no A


Join the domain controller:

    :::
    samba-tool domain join ad.fap.no DC -U"FAP.NO\administrator" --dns-backend=SAMBA_INTERNAL

Check the replication status:

    :::
    samba-tool drs showrepl

If showrepl fails with "WERR_NOMEM", try restarting samba.

## ldapsearch

    :::
    ldapsearch \
        -x -h localhost \
        -D "read@fap.no" \
        -W \
        -b "dc=fap,dc=no" '(sAMAccountName=*)'

Filter by group:

    :::
    ldapsearch \
        -x -h localhost \
        -D "read@fap.no" \
        -W \
        -b "dc=fap,dc=no" '(&(sAMAccountName=kradalby)(memberof=CN=www,CN=Users,DC=ad,DC=fap,DC=no))'

LDAPS search without a signed certificat:

Edit /etc/ldap/ldap.conf

Add:

    TLS_REQCERT allow

Then run ldapserach with ldaps://

    ldapsearch \
        -x -W -H ldaps://slowking.fap \
        -D "cn=read,cn=users,dc=ad,dc=fap,dc=no" \
        -b "cn=users,dc=ad,dc=fap,dc=no" \
        '(sAMAccountName=*)'


Disable password expiration for user:

    samba-tool user setexpiry USER --noexpiry
