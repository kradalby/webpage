---
title: Getting Intel D2500CC graphics working on Debian
tags: debian, intel, integrated graphics, graphics, intel d2500cc, d2500, d2500cc
date: 2015-01-06 18:14
modified: 2015-01-06 18:14
summary: Getting Intel D2500CC integrated graphics working on Debian
---

[TOC]

I have a [Intel DC2500CC](http://www.intel.com/content/www/us/en/motherboards/desktop-motherboards/desktop-board-d2500cc.html) motherboard which I use as a router/firewall appliance. It has really great specs for this purpose. Unfortunately, out of the box, when finished installing Debian, it will not provide any graphics output. Heres how I solved it.

The motherboard uses the Intel GMA 3600 integrated graphics, which for some reason defaults to the LVDS also known as the laptop screen as default. Since this is a desktop motherboard, it will just send the display data to nowhere.

To fix this, we need to add an option to the grub boot line which disables the LVDS from boot, and forces the driver to choose another display.

We need to add:

    :::
    video=LVDS-1:d

On the default boot option in my /boot/grub/grub.cfg, this is how mine is configured:

    :::
    linux   /boot/vmlinuz-3.16.0-4-amd64 video=LVDS-1:d root=UUID=5fd3abb1-3ea6-40d9-8dff-d57de685bec1 ro  quiet

Reboot, and you should get information on your display again.

[Source](http://forums.debian.net/viewtopic.php?f=7&t=106713)
