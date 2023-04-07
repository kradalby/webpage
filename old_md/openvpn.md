---
title: OpenVPN with LDAP support
tags: VPN, LDAP, OpenLDAP, OpenVPN, private networking, virtual private network
date: 2014-01-01 10:00
modified: 2015-01-09 12:09
summary: How to set up OpenVPN with LDAP login instead of certs.
---

[TOC]

I was on a trip in the United States for two weeks and relied heavily on free and open networks from restaurants and hotels. Therefore, before we left I took a few hours to set up an OpenVPN server.

The reason I chose OpenVPN was ease of setup, the quite good Mac client [Tunnelblick](https://code.google.com/p/tunnelblick/) and the fact that it was an SSLVPN.

One of the benefits of SSLVPN is that you can configure it to run on port 443 over TCP (you can probably configure other VPN types in this way too). Quite a few of the Captive portals runs on this port and have no restrictions for another type of traffic than HTTPS. This means that the VPN can connect before you accept the terms for the WiFi or even pay in some situations.

## Installation
We will start with installing OpenVPN:

    :::
    apt-get install openvpn openvpn-auth-ldap

We will need to generate a CA certificate, so we will use the easy-rsa toolkit:

    :::
    cp -R /usr/share/easy-rsa /etc/openvpn

Edit the vars file in the easy-rsa directory to the appropriate information:

    :::
    export KEY_COUNTRY="CN"
    export KEY_PROVINCE="BJ"
    export KEY_CITY="BeiJing"
    export KEY_ORG="iRedMail"
    export KEY_EMAIL="www@example.com"

Initialize the PKI:

    :::
    cd /etc/openvpn/easy-rsa/
    chmod +rwx *
    source ./vars
    ./clean-all
    ./pkitool --initca

Generate server Certificates:

    :::
    ./pkitool --server server

Generate Diffie Hellman Parameters Link:

    :::
    ./build-dh

When all the keys have been generated, copy them to the OpenVPN directory:

    :::
    cp keys/{ca.crt,ca.key,server.crt,server.key,dh2048.pem} /etc/openvpn/

## Configuration
We will start off by configuring the LDAP support

### LDAP
First we will add a base configuration:

    :::
    <LDAP>
        URL     ldap://slowbro.fap

        BindDN      cn=admin,dc=fap,dc=no

        Password    test

        Timeout     15

        TLSEnable   no

        FollowReferrals yes

        TLSCACertFile   /usr/local/etc/ssl/ca.pem

        TLSCACertDir    /etc/ssl/certs

        TLSCertFile /usr/local/etc/ssl/client-cert.pem
        TLSKeyFile  /usr/local/etc/ssl/client-key.pem

        # Cipher Suite
        # The defaults are usually fine here
        # TLSCipherSuite    ALL:!ADH:@STRENGTH
    </LDAP>

    <Authorization>
        BaseDN      "ou=people,dc=fap,dc=no"

        SearchFilter    "(uid=%u)"

        RequireGroup    false

        # Add non-group members to a PF table (disabled)
        #PFTable    ips_vpn_users

        <Group>
            BaseDN      "ou=groups,dc=fap,dc=no"
            SearchFilter    "(cn=vpn)"
            #MemberAttribute    memberUid
            MemberAttribute uniqueMember
        </Group>
    </Authorization>

Take the base configuration and put it in /etc/openvpn/auth/auth-ldap.conf. Then it is pretty straight forward to edit it to your needs.

At the moment, I have not got the group filtering to work.

### The server
OpenVPN can run on both TCP and UDP as well as multiple ports, however, you will need to create a configuration for each of them.

Here is my base TCP configuration:

    :::
    plugin /usr/lib/openvpn/openvpn-auth-ldap.so /etc/openvpn/auth/auth-ldap.conf
    client-cert-not-required

    port 1194

    proto tcp

    dev tun

    ca ca.crt
    cert server.crt
    key server.key  # This file should be kept secret

    dh dh2048.pem

    server 10.8.0.0 255.255.255.0

    ifconfig-pool-persist ipp.txt
    push "dhcp-option DNS 8.8.8.8"
    push "dhcp-option DNS 129.241.0.200"
    keepalive 10 120
    comp-lzo
    persist-key
    persist-tun
    status openvpn-status.log
    log         openvpn_tcp.log
    log-append  openvpn_tcp.log
    verb 3

There is my base UDP configuration:

    :::
    plugin /usr/lib/openvpn/openvpn-auth-ldap.so /etc/openvpn/auth/auth-ldap.conf
    client-cert-not-required

    port 1194

    proto udp

    dev tun

    ca ca.crt
    cert server.crt
    key server.key  # This file should be kept secret

    dh dh2048.pem

    server 10.9.0.0 255.255.255.0

    ifconfig-pool-persist ipp.txt
    push "dhcp-option DNS 8.8.8.8"
    push "dhcp-option DNS 129.241.0.200"
    keepalive 10 120
    comp-lzo
    persist-key
    persist-tun
    status openvpn-status.log
    log         openvpn_tcp.log
    log-append  openvpn_tcp.log
    verb 3

The configuration files can be named tcp.conf and udp.conf. OpenVPN will detect them.

### IPTables
The last thing we need to do on the server is to set up iptables so it will allow the traffic to pass through and to masquerade it.

First, allow IP forwarding in /etc/sysctl.conf:

    :::
    net.ipv4.ip_forward = 1

Then add the masquerade rules:

    :::
    iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT
    iptables -A FORWARD -s 10.8.0.0/24 -j ACCEPT
    iptables -A FORWARD -s 10.9.0.0/24 -j ACCEPT
    iptables -A FORWARD -j REJECT
    iptables -t nat -A POSTROUTING -s 10.8.0.0/24 -o eth0 -j MASQUERADE
    iptables -t nat -A POSTROUTING -s 10.9.0.0/24 -o eth0 -j MASQUERADE

Also, if you have a firewall you will need to open the ports:

    :::
    iptables -A INPUT -m state --state NEW -p tcp --dport 1194 -j ACCEPT
    iptables -A INPUT -m state --state NEW -p udp --dport 1194 -j ACCEPT
