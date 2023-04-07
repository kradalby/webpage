Title: Test hard drives
Tags: smartctl, badblocks, hard drives, hdd,
Date: 2016-01-18 11:52
Modified: 2016-01-18 12:43
Summary: A few commands to testing hard drives.

[TOC]

## Smart status
The smart status of a hard drive contains information about the health of the drive.

All the tests can be run in the forground by adding -C.

Start short test:

    :::
    smartctl -t short /dev/ada4

Start conveyance test (determine damage during transport):

    :::
    smartctl -t conveyance /dev/ada4

Start long test to test the entire drive:

    :::
    smartctl -t long /dev/ada4

View test results:

    :::
    smartctl -l selftest /dev/ada4

View disk information:

    :::
    smartctl -a /dev/ada4


## Bad blocks
The command badblocks can be used to check for bad blocks on a hard drive, i perfere to run this before i put the drive in to production.

Most hard drives stops working after a few days if they are defect from the store, so this is a really nice test to run.

The following command will test all the blocks on the drive, by writing to them, so it will **NUKE** all the data:

    :::
    badblocks -v -p 3 -b 512 -ws /dev/ada4

    -v: verbose output
    -p: number of passes (default: 0)
    -b: block size
    -w: write to all blocks
    -s: show progress

This will take a lot of time, and it should be set to run a few days in advance of installation.
