---
title: Cisco Catalyst Express 500
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: cisco, network, reset, express, crap
summary: How to reset a Cisco Express 500 and other stuff
---

[TOC]

I have one Cisco Catalyst Express 500 series switch and it is quite horrible. Here are some notes on how to reset it and configure it from scratch since it is somehow hard. Cisco actually only support Windows 2k or XP as OS for resetting the switch.

## Reset

### Laptop preparation

#### Windows 7 and above
If you use Windows 7 or any Windows higher than the officially supported ones you will need a little bit of luck.

My device will use 10.0.0.1 as its reset configuration IP, some devices use random 169.dot IPs.

If you know this information on forehand you can be able to get it working with Windows 7. By simply setting 10.0.0.2 as IP and 255.255.255.0 as the subnet mask. It is very _important_ that this is done _before_ you plug the laptop into the given configuration port.

#### Windows XP
If you use Windows XP set the network interface to obtain an IP automatically and set the DNS server to 127.0.0.1.


### Setting the switch in reset mode

1. Plug the power into the switch while holding the setup button
2. When the System, Alert, and Poe light turn amber release the setup button.
3. Wait for a light to indicate what port to plug your computer into.
4. a) If you use Windows 7 open 10.0.0.1 in your browser.
4. b) If you use Windows XP open your browser and enter the IP of your given gateway.
5. Follow the reset procedure in the browser.
