---
title: Setting up 6rd on my Linux router
date: 2014-11-22 10:12
modified: 2014-11-22 10:13
tags: 6rd, altibox, ipv6, tunnel, debian, linux
summary: Setting up 6rd on my Linux router
---

[TOC]

Altibox lets its users setup IPv6 via 6rd until their dual stack network is finished and rolled out. In this post i will show you how to set it up on a Linux box that is directly connected to a Altibox modem that is configured in bridge mode.

First of all we need the Altibox 6rd information:

    :::
    IPv4 BR adresse: 213.167.115.92
    IPv4 Prefix: 0
    IPv6 Prefix: 2a01:79c::
    IPv6 Prefix Length: 30
    IPv6 DNS: 2a01:798:0:8012::4

We also need:

* A Linux kernel that is IPv6 ready (and 6rd)
* ipv6calc (apt-get install ipv6calc)


## Calculate your IPv6 prefix

The first we need to do is to calculate what our IPv6 prefix will be based on our public IP.

To do this, we will use ipv6calc, some of the altibox data and our public IP.

    :::bash
    PREFIX=2a01:79c::
    PREFIX_LENGTH=30
    RELAY_PREFIX=213.167.115.92
    RELAY_PREFIX_LENGTH=0
    PUBLIC=$(curl http://canihazip.com/s)

    ipv6calc --action 6rd_local_prefix --6rd_prefix $PREFIX/$PREFIX_LENGTH --6rd_relay_prefix $RELAY_PREFIX/$RELAY_PREFIX_LENGTH $PUBLIC

When you run this in the terminal of the Linux host you will get three lines of output, ignore the first two and keep the actual IPv6 prefix.
It should look something like this 2a01:79d:aaaa:bbbb::/62

## Create the tunnel

Next we will configure and test the tunnel. My preferred way of doing this is to add every command to a script so I can run them all at once and to make it easier to re-run them.

In my script, there is two sections, the variables, and the IP commands we use to setup the tunnel.

Here is the variable part, with the IPv6 prefix we calculated earlier:

    :::bash
    # From Altibox
    PREFIX=2a01:79c::
    PREFIX_LENGTH=30
    RELAY_PREFIX=$(dig +short 6rd.altibox.net)
    RELAY_PREFIX_LENGTH=0
    PUBLIC=$(curl http://canihazip.com/s)

    # IPv6 Prefix
    IPv6_PREFIX=2a01:79d:aaaa:bbbb
    IPv6_PREFIX_LENGTH=62


Now we have to do the part where we actually create the tunnel and set up the interfaces.

    :::bash
    ip -6 addr add $IPv6_PREFIX::/$IPv6_PREFIX_LENGTH dev lo
    ip tunnel add 6rd mode sit local $PUBLIC ttl 64
    ip tunnel 6rd dev 6rd 6rd-prefix $PREFIX/$PREFIX_LENGTH
    ip link set 6rd up
    ip -6 addr add $IPv6_PREFIX:0::/$PREFIX_LENGTH dev 6rd
    ip -6 route add 2000::/3 via ::$RELAY_PREFIX dev 6rd

The commands over will add an IPv6 address to the loopback interface, then create the 6rd tunnel and apply the Altibox and prefix information. After the tunnel is created it will be brought up. When the link us up it will add an IPv6 address and add an IPv6 route to the routing table via the 6rd tunnel.

When all this commands are finished successfully you should be able to use IPv6 from this machine. To test issue:

    :::bash
    ping6 ipv6.google.com


## IPv6 forwarding to local network

If your box, like mine, is a router in my local network. You would probably like to make the other computers also get IPv6.

To accomplish this, we need to do three things. Give the LAN facing interface an IPv6 address, allow IPv6 forwarding and setup Radvd.

### Add an IPv6 address to LAN

    :::bash
    LAN=eth0
    ip -6 addr add $MY_IPv6_PREFIX:1::/64 dev $LAN

### Allow IPv6 forwarding

    :::bash
    sysctl -w net.ipv6.conf.all.forwarding=1

Note: This resets after boot, so you should add it to you IPv6 script.

### Radvd
We need to tell all the machines on the local network that there is IPv6 available, and this is done by using Router Advertising.

First we need to install Radvd:

    :::bash
    apt-get install radvd

And then configure it (/etc/radvd.conf):

    :::
    interface eth0 {
       AdvSendAdvert on;
       MinRtrAdvInterval 3;
       MaxRtrAdvInterval 10;
       AdvLinkMTU 1280;
     prefix 2a01:79d:aaaa:bbbb::/64 {
       AdvOnLink on;
       AdvAutonomous on;
       AdvRouterAddr on;
       AdvValidLifetime 86400;
       AdvPreferredLifetime 86400;
       };
    };

Remember to change the interface and prefix correctly.

You should also add service radvd start to your script as it does not start by default, and you don't want it running unless the tunnel is up.
