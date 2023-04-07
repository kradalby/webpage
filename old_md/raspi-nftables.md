---
title: Testing nftables with Raspberry Pi and Arch
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: raspberry pi, nftables, arch Linux
summary: Testing the new Linux packet filter, nftables on a Raspberry Pi and Arch.
---

[Work in progress]
This project is currently on hold as I have little time, and had some problems with compiling the kernel.

Yesterday I learned that iptables will be replaced by the successor nftables. Nftables was merged into the Linux kernel on 19. January 2014 and was released with the 3.13 kernel.

nftables will make it possible to write rules more efficient and reduce code duplication. The userspace utility nft will replace iptables, ip6tables, arptables and ebtables.
Currently, one of my favorite features is that you can write rules for both ipv4 and ipv6 at the same time, or specify if you want to.

In this setup, i will use a Raspberry Pi with the Arch Linux ARM 2014.04 build. However, I would recommend to always get the latest version if setting up a new Pi. You can find this [Here](http://archlinuxarm.org/platforms/armv6/raspberry-pi). I will also be using an Apple USB Ethernet adapter as the second NIC as I will try to NAT.


## Upgrading to the newest kernel
When I was writing this guide, the latest available stable precompiled kernel for RPI was 3.10. Luckily the Arch guys are really quick, and the latest 3.14 build of the kernel was available from the repositories.

Install the newest kernel:

    :::bash
    pacman -Syu
    pacman -S linux-raspberrypi-latest linux-raspberrypi-latest-headers


Install the userpace utility for nftables:

    :::bash
    pacman -S nftables
