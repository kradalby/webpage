---
title: Compile latest Linux kernel for Debian
date: 2014-01-01 10:00
modified: 2015-12-20 14:25
tags: Linux, kernel, Debian, latest
summary: a Quick collection of commands on how to compile the latest Linux kernel for Debian.
---

## Install needed packages

    :::bash
    apt-get install fakeroot kernel-package libncurses5-dev

## Download latest Kernel source
As of this writing, the latest source is 4.3.3

    :::bash
    wget https://cdn.kernel.org/pub/linux/kernel/v4.x/linux-4.3.3.tar.xz
    export KVERSION=4.3.3
    tar xvJf linux-$KVERSION.tar.xz

## Start configuring

    :::bash
    cd linux-$KVERSION

Get current config

    :::bash
    cp /boot/config-`uname –r` .config

Start menuconfig and change the stuff you want to:

    :::bash
    make menuconfig

Clean:

    :::bash
    make-kpkg clean

## Building
Export the wanted concurrency (number of cores + 1) and build a Debian package of the kernel.

    :::bash
    export CONCURRENCY_LEVEL=5
    fakeroot make-kpkg --append-to-version "-customkernel" --revision "1" --initrd kernel_image kernel_headers

When this command is complete, we will have two dpkg files, one for the kernel and one for headers. The great thing about this is that we can just install them and it will fix grub for us.

    :::bash
    dpkg -i linux-headers-$KVERSION-customkernel_1_amd64.deb linux-image-$KVERSION-customkernel_1_amd64.deb

## Cleaning
Remove everything we do not need anymore:

    :::bash
    cd ~
    rm -f linux-$KVERSION.tar.xz
    rm -rf linux-$KVERSION
    rm -f linux-headers-$KVERSION-customkernel_1_amd64.deb linux-image-$KVERSION-customkernel_1_amd64.deb

## Remove a custom kernel
To remove a kernel that we are not using, or that does not work just uninstall it.

    :::bash
    apt-get remove linux-image-$KVERSION-customkernel linux-headers-$KVERSION-customkernel
