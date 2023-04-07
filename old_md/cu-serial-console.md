---
title: Cu Serial Console
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: serial, console, network
summary: Cu as a serial console program.
---

I have started to use cu instead of screen as my go to serial console program.

This is my notes in case i forget.

To start a terminal session with baud rate 9600 and ttyUSB0 as serial adapter:

    :::bash
    cu -l /dev/ttyUSB0 -s 9600

To exit the session, use the same exit procedure as SSH: <Enter> ~. <Enter>

There exist a bug in Debian that will give you a Permission denied error even if you are root.
This can easily be fixed by giving read/write permission to everyone.

    :::bash
    chmod 666 /dev/ttyUSB0

The bug still exists as of Debian Jessie 16.05.14.
