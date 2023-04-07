---
title: Minimal Raspberry Pi Access Point
tags: rpi, debian, hostapd, raspberry, raspberry pi, access, access point, ap, wireless, wifi, wi-fi, linux, ralink, alfa, alfa networks
date: 2016-04-27 22:19
modified: 2016-04-27 22:36
summary: My minimal setup for a Raspberry Pi access point using USB wifi.
---

[TOC]

I needed an access point in my shared flat during my semester abroad in Spain as the shared routers performance dropped significantly when everyone was home. This is how I configured myself a 5 GHz access point using an old Raspberry Pi (Model B) and a Alfa Networks AWUS051NH v2. This networks card supports 5 GHz and N networking. My flat is in the middle of a big block of different apartments and networks, this is a really nice feature to have.

The Raspberry Pi should be up to date with the latest [Raspbian](https://www.raspbian.org) or [Minibian](https://minibianpi.wordpress.com).

## Setup

Install the necessary packages:

    apt install raspi-config hostapd bridge-utils wireless-tools iw firmware-linux-nonfree firmware-ralink

Configure hostapd location:

    vim /etc/default/hostapd

Configure the DAEMON_CONF:

    DAEMON_CONF="/etc/hostapd/hostapd.conf"

Configure hostapd:

    vim /etc/hostapd/hostapd.conf

Example:

    interface=wlan0
    bridge=br0
    hw_mode=a
    channel=0
    ieee80211d=1
    country_code=NO
    ieee80211n=1
    wmm_enabled=1

    ssid=blackpi
    auth_algs=1
    wpa=2
    wpa_key_mgmt=WPA-PSK
    rsn_pairwise=CCMP
    wpa_passphrase=herpaderp

Notable options:

**hw_mode**: a means 5 GHz (set to g for 2.4)

**channel**: 0 lets the software choose channel (recommended)

In this setup, the RPi will get its IP address from DHCP, so the only configuration we need to do to the interfaces is to prepare the bridge that wlan0 will use:

    vim /etc/network/interfaces

Minimal example:

    auto lo eth0 br0
    iface lo inet loopback

    iface eth0 inet manual

    iface br0 inet dhcp
       bridge_ports eth0
