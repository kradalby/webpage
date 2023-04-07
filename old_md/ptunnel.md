---
title: ICMP tunnel with ptunnel
tags: icmp, tunnel, icmptunnel, ptunnel
date: 2015-12-11 17:06
modified: 2015-12-11 17:38
summary: ICMP tunnel with ptunnel
---

[TOC]

Sometimes, using a tunnel can be very practical to bypass things like captive portals and firewalls, usually ping/ICMP is not blocked in these kinds of services, and if that is the case, we can tunnel traffic over it.

## Installation

On a Debian server:

    :::
    apt-get install ptunnel

On Mac

    :::
    brew install ptunnel

## Server
Start a server:

    :::
    ptunnel -v -x <PASSWORD>


## Client
Connect to a server and forward traffic to a given host:

    :::
    sudo ptunnel -p <PROXY HOST> -lp <LOCAL PORT> -da <DESTINATION HOST> -dp <DEST PORT> -x <PASSWORD>
    e.g:
    sudo ptunnel -p butterfree.fap.no -lp 8000 -da m.fap.no -dp 21337 -x example
    sudo ptunnel -p 84.52.231.133 -lp 8000 -da 129.241.210.115 -dp 21337 -x example

The example above will forward TCP traffic to localhost:8000 to m.fap.no:21337.

Create SSH socks proxy:

    :::
    ssh localhost -p 8000 -l root -D 8080
