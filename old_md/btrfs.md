---
title: Btrfs
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: btrfs, filesystem, storage, raid1
summary: General usage and setup for Btrfs
---

I am currently running btrfs on the storage part of my home server. It has two 4TB drives in btrfs raid1. This is the commands I needed to get it working and that I use to check status and such.

## Installation

First of all, you should use the most stable kernel you can get your hands on since btrfs is under active development. I am currently running 3.15.1 on the server I got btrfs.

Since btrfs requires some userspace applications and they do not come with Debian we will need to install them.

Install btrfs tools:

    :::bash
    apt-get install btrfs-tools


## Setup
There is no need for creating partitions.

The following commands use -m and -d for arguments. -d means data and -m means metadata. This is so you can choose how you will stripe the two over more than one disk.

### Non raid

To setup one drive with plain btrfs:

    :::bash
    mkfs.btrfs /dev/sda


### Raid 1

To create a raid1 from two drives:

    :::bash
    mkfs.btrfs -m raid1 -d raid1 /dev/sda /dev/sdb

### Raid 5 and 6
As of today (29/06/14). There exists working raid5/6 functionality, but the recovery/rebuilding part does not work. Do not use it.

### Mount

#### fstab

Since my drive will be used for saving important things like photos and mainly archiving stuff I used the recommended options for this from Arch Wiki:

    :::bash
    /dev/sdd on /storage type btrfs (rw,relatime,compress-force=zlib,nospace_cache,autodefrag)

The line from fstab:

    :::bash
    UUID=96f21485-8527-4ddb-8876-2a4cdb1af8ac /storage  btrfs  compress-force=zlib,autodefrag,nospace_cache  0 1

## Useful commands

### Information/Status

Show information about the setup:

    :::bash
    btrfs filesystem show /dev/sda

Show df information: (the standard df command will report a little funny)

    :::bash
    btrfs filesystem df /mountpoint

### Checks/Diagnostics

Check the filesystem:

    :::bash
    btrfsck /dev/sda

Defragment the filesystem:

    :::bash
    btrfs filesystem defrag /mnt
