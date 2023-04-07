---
title: Netcom 4G incoming traffic
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: Netcom, 4g, port forward
summary: Little wrant about Netcom 4G and stupidity.
---

At my families home and warehouse we have to use Netcom's 4G network to have a usable internet connection. We use a Huawei B593s-22 router for this.

With the APN information Netcom gives and the default settings for their network, all incoming traffic are blocked.

This is very annoying as I have some servers and computers I want to connect to.

Fortunately, Netcom has an alternative address that can be used.

Netcom wants you to use:
    internet.netcom.no

But if you want control over your own internet connection:
    vpn.netcom.no

Netcom is probably using NAT on its mobile network. But they have been kind enough to create VPN so users that need VPN can have public IPs and no restrictions.
