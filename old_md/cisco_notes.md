---
title: Notes about configuring Cisco equipment
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: network, networking, cisco, switch
summary: My notes for configuring cisco switches and routers, at some point, this will have both basic and advanced stuff.
---

[TOC]

## Setting IP on Vlan1 for basic management

This is the basic steps for setting a static IP to a cisco switch and giving it a default gateway.
This will configure the standard Vlan1, which may not be a good idea in most setups.

    :::C
    Switch> enable
    Switch# conf t
    Switch(config)# int vlan1

    # You can also use IP address DHCP.
    Switch(config-if)# ip address 192.168.1.10 255.255.255.0

    # Remember to activate the interface
    Switch(config-if)# no shutdown
    Switch(config-if)# end
    Switch(config)# end


## Backup or Restore via TFTP

To backup:

    :::C
    Switch# copy running-config tftp://ip/filename

To restore:

    :::C
    Switch# copy tftp://ip/filename running-config


## Write and erase config

To save:

    :::C
    Switch# copy runn start
    or
    Switch# write mem
    Switch# reload

To erase

    :::C
    Switch# write erase
    Switch# reload

## Set enable password

    :::C
    Switch> en
    Switch# conf t
    Switch(config)# enable password derp
    Switch(config)# end

## Copy firmware to device

### Set boot firmware

THERE IS A FAULT IN THIS INSTRUCTION!

    :::C
    Switch> en
    Switch# conf t
    Switch# boot system path:to:firmware.bin


## Enable remote login

### Telnet

!Not recommended!

    :::C
    Switch> en
    Switch# conf t
    Switch(config)#line vty 0 4
    Switch(config-line)#login
    % Login disabled on line 1, until 'password' is set
    % Login disabled on line 2, until 'password' is set
    % Login disabled on line 3, until 'password' is set
    % Login disabled on line 4, until 'password' is set
    % Login disabled on line 5, until 'password' is set
    Switch(config-line)#password derp

### SSH

If supported.

Setup a user account:

    :::C
    Switch> en
    Switch# conf t
    Switch(config)# aaa new-model
    Switch(config)# username kradalby password 0 derp

Note: The password is clear text in the config.

Enable remote management:

    :::C
    Switch(config)# line vty 0 4
    Switch(config)# transport input SSH

Enable SSH:

    :::C
    Switch(config)# ip domain-name fap.no
    Switch(config)# crypto key generate rsa

    The name for the keys will be: Switch.fap.no
    Choose the size of the key modulus in the range of 360 to 2048 for your
      General Purpose Keys. Choosing a key modulus greater than 512 may take
      a few minutes.

    How many bits in the modulus [512]: 2048
    % Generating 2048 bit RSA keys ...[OK]

    Switch(config)#ip ssh time-out 60
    Switch(config)#ip ssh authentication-retries 3

Note: ssh on your Linux and Mac computer will not by default let you connect to a server with a key smaller than 768.

## DHCP autoconfiguration

Most Cisco switches, to my knowledge, can be autoconfigured with a combination of DHCP and TFTP. This is a great feature.
My main use of these switches is table switches at LAN parties.
With some proper configuration, we can configure the DHCP, core switch, and TFTP server so that every table switch can fetch its correct configuration on boot. It will also give great flexibility when it comes to switches that break during the event or something else. There will actually be possible to just put in a drop-in replacement, and it will automatically be configured correctly.

### Notes from testing C2950-24
I have been testing when the C2950 fetches a configuration and when it is not.

It will always fetch the configuration when it has no configuration (duhh).
It will not save the configuration. Which is great for us, since we want it to always fetch the latest.
It will not fetch a configuration once the switch has a startup config.


## Firmware

On 3560 series switches and other 3000 series switches, the optimal firmware is ipservices.

ipservices has routing protocols, ipbase don't. ipbase still has some layer 3 functionality.

### Verify
To verify files, mainly firmware, issue the following command:

    :::C
    verify /md5 filesystem:filename [md5-hash]

## Diagnostic
To trigger diagnostics:

    :::
    diagnostic start switch 1 test all

# Interface

## Make an interface L3

    :::C
    Switch> enable
    Switch# conf t
    Switch(config)# int G0/1
    Switch(config-if)# no switchport

You also want to activate IP routing, and set an up on the interface.

    :::C
    Switch(config)# ip routing



## SNMP
To activate a read-only public community execute the following commands in configuration mode:

    :::C
    SNMP-server community public RO
    SNMP-server location Lake Travis (Austin) Dial POP
    SNMP-server contact net-admin@aurora.the.net

Note: Change the location and contact to the correct information.
If using observium, use very accurate location to get map functionality to work.

## Creating VLANs and assigning ports

To create a VLAN on a C3560G enter configure mode and execute:

    :::C
    Switch# configure terminal
    Switch(config)# vlan 20
    Switch(config-vlan)# name test20
    Switch(config-vlan)# end

To assign a port or portrange to the newly created VLAN:

    :::C
    Switch# configure terminal
    Switch(config)# interface g0/1
    Switch(config-if)# switchport mode access
    Switch(config-if)# switchport access vlan 20
    Switch(config-if)# end


To make a port capable of passing multiple vlans through, put it in trunk mode:

    :::C
    Switch# configure terminal
    Switch(config)# interface g0/1
    Switch(config-if)# switchport mode access
    Switch(config-if)# switchport trunk
    Switch(config-if)# end

## Routing LAN network to InterWebs (StudLAN 2014h)

To get this working, we basically need two L3 interfaces or one L3 interface against the internet and a local VLAN for the internal network.

The basic idea is that one L3 interface has a given IP configuration against the ISP and that we have an IP route against its gateway.

To configure the uplink:

    :::C
    Switch# configure terminal
    Switch(config)# interface g0/1
    Switch(config-if)# no switchport
    Switch(config-if)# no shutdown
    Switch(config-if)# ip address <ip given by isp> <netmask>
    Switch(config-if)# description Uplink
    Switch(config-if)# end

Set up routing and the route:

    :::C
    Switch# configure terminal
    Switch(config)# ip routing
    Switch(config)# ip route 0.0.0.0 0.0.0.0 <ip to gateway for uplink>

We also need to set up a local L3 or a VLAN, depending on the network layout.
