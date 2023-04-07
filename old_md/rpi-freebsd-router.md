---
title: Raspberry Pi router with FreeBSD
tags: rpi, raspberry, pi, raspberry pi, router, FreeBSD, BSD, free
date: 2000-01-01 10:00
modified: 2000-01-01 10:00
summary: How i installed FreeBSD on my Raspberry Pi and used it as a router
---

[TOC]

This project is a result of an urge to relearn FreeBSD, as it is quite some time since I still used it. Also, I had spear Raspberry Pi and decided that using a laptop as a router for three people is kind of overkill.

I will in this post used the first Raspberry Pi model B, the one with 256mb ram.

## Download FreeBSD
In this post, I have used FreeBSD 10.1

You can get the Raspberry Pi version from [here](ftp://ftp.freebsd.org/pub/FreeBSD/releases/arm/armv6/ISO-IMAGES/10.1/)

## Creating the SD card
I mainly use OS X, so the few commands needed to create the SD card are specific. The _dd_ command can probably be used the same way on Linux.

**List disks:**

    :::
    diskutil list

**Unmount the SD card:**

    :::
    diskutil unmountDisk /dev/diskX

**Write the image to the SD card:**

    :::
    sudo dd bs=1m if=FreeBSD-10.1-RELEASE-arm-armv6-RPI-B.img of=/dev/diskX

In case of cached writes, force OS X to sync all writes to disk:

    :::
    sync

**Eject the disk:**

    :::
    diskutil eject /dev/diskX

## Bootstrapping FreeBSD
The first time the Pi boots up, it will not have anything set. Login will be root, with no password.

**Set password:**

    :::
    passwd

The date will also be very wrong, probably from when the image was created, so change that to:

    :::
    date yymmddhhmm

Then download the ports collection so we can build packages:

    :::
    portsnap fetch
    portsnap extract

This is going to take a long time, so you should probably set it up over night. This also goes for the things we are going to compile.
