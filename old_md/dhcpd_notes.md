---
title: Notes about ISC DHCPD
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: network, networking, DHCP
summary: These are my notes for ISC DHCPD, some tricks, and stuff
---

### Find vendor identifier easy

Simply put the following string into the dhcpd.conf file, the information will then show up in the lease file.

    :::bash
    set vendor-string = option vendor-class-identifier;


### DHCP Links
#### Codes
https://www.iana.org/assignments/bootp-dhcp-parameters/bootp-dhcp-parameters.xhtml

#### ISC DHCP options/man
http://linux.about.com/od/commands/l/blcmdl5_dhcpopt.htm
