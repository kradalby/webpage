---
title: Linux router with multiple ISPs
tags: linux, debian, jessie, multiple isp, isp, internet, multiple internet
date: 2014-12-25 21:36
modified: 2014-12-25 21:36
summary: Linux router with multiple ISPs and how to sort clients over the different connections.
---

[TOC]

This christmas, my parents got a very slow and crappy ADSL connections installed from HomeNet(who sucks) in addition to the 4G connection they already have from Netcom. Since the 4G connection has a limited amount of data, but is fast, and the ADSL has unlimited data, but a slow connection, the idea is that "low priority machines" will use the ADSL to save data and "high priority machines" can use the 4G.

The setup uses a Intel Atom box with Debian Jessie and three NICs.

## Routing tables
We will create one more routing table than the default. We want the traffic from some of the clients to use the default table and some of them to use the additional table.

Lets say we have two internet connections, eth0 and eth2. eth0 is the 4G and eth2 is the ADSL. The default routing table will have the 4G as the default route and the additional table will have the ADSL as the default route. We can achieve this by doing this.

    :::
    ip route flush table 2
    ip route add table 2 default via 8.8.8.8

I like to give the table a number based on the interface number. In this example 8.8.8.8 will be the gateway for the ADSL connection.

You can verify the table by running:

    :::
    ip route show table 2

## Selecting traffic to go where
We will use iptables to selecting and marking traffic for the appropriate connection.

You can mark whatever you want, but lets say we would like a ip range to use the ADSL connection:

    :::
    iptables -t mangle -A PREROUTING  -m iprange --src-range 192.168.1.11-192.168.1.30 -j MARK --set-mark 2

This will mark all traffic from hosts within the 192.168.1.11-192.168.1.30 with 0x2.

## Create a ip rule to bind things together
Now we need to create a rule that will actually take the traffic and send it through the gateway.

This can be done by taking traffic marked and directing it to the table.

    :::
    ip rule add fwmark 2 table 2
