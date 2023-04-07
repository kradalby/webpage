---
title: HP ACU cli utility
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: hp, acu, p410i, raid, cli, linux
summary: Little collection of some HP ACU commands i might use.
---

[TOC]

## Usage

Display controller and disk status:

    :::
    ctrl all show config

View Controller status:

    :::
    ctrl all show status

View Drive Status

    :::
    ctrl slot=0 pd all show status

View Individual Drive Status

    :::
    ctrl slot=0 pd 2I:1:6 show detail

View All Logical Drives

    :::
    ctrl slot=0 ld all show

View Detailed Logical Drive Status

    :::
    ctrl slot=0 ld 2 show

Blink Physical Disk LED

    :::
    ctrl slot=0 ld 2 modify led=on

Add raid controller license:

    :::
    ctrl slot=0 add licensekey=XXXXX-XXXXX...
