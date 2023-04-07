---
title: Useful SNMP notes
tags: snmp, snmpd, oid, oids, mib, mibs, linux, set, get
date: 2015-01-14 17:35
modified: 2015-01-14 17:35
summary: a Useful collection of SNMP information I have used and use frequently.
---

[TOC]

SNMP is a widely used protocol with a lot of great and not so great features. This is my notes on things related to this protocol that I use frequently.

## Useful OIDs

### Interface related

*Interface description*

OID value: 1.3.6.1.2.1.2.2.1.2

*Interface octet in*

64 bit counter for a total number of received octets/bytes.

OID value: 1.3.6.1.2.1.31.1.1.1.6

*Interface octet out*

64 bit counter for a total number of transmitted octets/bytes.

OID value: 1.3.6.1.2.1.31.1.1.1.10


## Turn of SNMPd cache
To turn off SNMPd cache on the Debian Linux package you have to, as far as I know, do this with SNMP.

First, check your current cache time:

    :::
    $ snmpwalk -v2c -c public 10.0.0.1 1.3.6.1.4.1.8072.1.5.3.1.2.1.3.6.1.2.1.2.2
    NET-SNMP-AGENT-MIB::nsCacheTimeout.1.3.6.1.2.1.2.2 = INTEGER: 15

The integer represents seconds.

To change it, from a rwcommunity issue:

    :::
    $ snmpset -v2c -c public 10.0.0.1 1.3.6.1.4.1.8072.1.5.3.1.2.1.3.6.1.2.1.2.2 i 0
    NET-SNMP-AGENT-MIB::nsCacheTimeout.1.3.6.1.2.1.2.2 = INTEGER: 0
