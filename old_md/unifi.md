Title: Setting up Unifi for Ubiquiti APs
Tags: unifi, Debian, Jessie, ubiquiti
Date: 2014-12-25 20:48
Modified: 2014-12-25 20:48
Summary: Installing Unifi under Debian Jessie to control Ubiquiti APs

[TOC]

My parents have moved to a farm with multiple houses and a very big barn. So I got them to buy four of the Unifi APs.

## The setup
The "router" in my parents setup is a Debian Jessie box with three NICs for two internet connections and the local network. It runs iptables and ISC DHCP server. The ability to pass an IP address to the AP when it gets its address from DHCP is needed, so this was very convenient.

## Configure DHCP
We need to set up a option space and a class for the AP. This can be done like this:

    :::
    option space ubnt;
    option ubnt.unifi-address code 1 = ip-address;
    class "ubnt" {
        match if substring (option vendor-class-identifier, 0, 4) = "ubnt";
        option vendor-class-identifier "ubnt";
        vendor-option-space ubnt;
    }

We also need to tell the DHCP server to pass the option to the appropriate client. In my setup, I have a special pool for the APs where they only get IPs within a range and gets the unifi-address.

    :::
    pool {
        range 192.168.1.21 192.168.1.30;
        option ubnt.unifi-address 192.168.1.1;
        allow members of "ubnt";
    }

## Installing Unifi
Ubiquiti offers a Debian repository which we will use here.

Add the repo:

    :::
    echo "deb http://www.ubnt.com/downloads/unifi/distros/deb/wheezy wheezy ubiquiti" >> /etc/apt/sources.list.d/20ubiquiti.list
    apt-key adv --keyserver keyserver.ubuntu.com --recv C0A52C50

Not that as of this writing, there was no Jessie repo available, but you can use the Wheezy repo with a minor adjustment after installation.

We also need MongoDB, so we will add their repo too:

    :::
    echo "deb http://downloads-distro.mongodb.org/repo/ubuntu-upstart dist 10gen" >> /etc/apt/sources.list.d/21mongodb.list
    apt-key adv --keyserver keyserver.ubuntu.com --recv 7F0CEB10

Update and install:

    :::
    apt-get update
    apt-get install unifi

When I had installed unifi, it failed to start due to a problem to locate OpenJDK. This can be easily fixed by editing the init file located at /etc/init.d/unifi.

Just change the JAVA\_HOME variable with the location of the JDK and start it again.

In my case this was the correct setting:

    :::
    JAVA_HOME=/usr/lib/jvm/java-1.6.0-openjdk-amd64

When unifi now starts, you can access it at https://ip:8443

When you can also now plug in your APs to your network and they should automatically appear in the interface.
