---
title: Fix Windows 8 install error VirtualBox
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: windows, 8, server, 2012, virtualbox, 0x000000C4
summary: Fix the 0x000000C4 install error when trying to install Windows 8 or Windows Server 2012 in VirtualBox
---

[TOC]


Fix the 0x000000C4 install error when trying to install Windows 8 or Windows Server 2012 in VirtualBox

This is mainly written for Mac as that is what I use, but should work on every OS.

## How to fix

First we need to get the names of your Virtual machines:

    :::bash
    VBoxManage list vms

Then we will get the following example output:

    "XP" {f1bd07df-aa36-43ec-8335-8afaa45fed6e}
    "Windows 8 64" {18e20ca4-4de1-4b80-a164-b4863ebf5081}

Now to fix the error on the Windows 8 Machine:

    :::bash
    Vboxmanage setextradata "Windows 8 64" VBoxInternal/CPUM/CMPXCHG16B 1

Now it should work :)
