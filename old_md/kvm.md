---
title: Libvirt with KVM/QEMU on Debian Linux
tags: KVM, VM, Virtualization, virtual, virtual machine, debian, linux, windows, qemu, libvirt, virsh
date: 2015-12-18 07:29
modified: 2015-12-21 10:21
summary: KVM/QEMU on Debian Linux
---

[TOC]

KVM requires that the processor has virtualization support, named VT-x for Intel, and AMD-V for AMD.

Check by issuing lscpu:

    :::
    $ lscpu
    Architecture:          x86_64
    CPU op-mode(s):        32-bit, 64-bit
    Byte Order:            Little Endian
    CPU(s):                4
    On-line CPU(s) list:   0-3
    Thread(s) per core:    1
    Core(s) per socket:    4
    Socket(s):             1
    NUMA node(s):          1
    Vendor ID:             GenuineIntel
    CPU family:            6
    Model:                 15
    Model name:            Intel(R) Core(TM)2 Quad CPU    Q6600  @ 2.40GHz
    Stepping:              11
    CPU MHz:               3604.054
    BogoMIPS:              7208.10
    Virtualization:        ___VT-x___
    L1d cache:             32K
    L1i cache:             32K
    L2 cache:              4096K
    NUMA node0 CPU(s):     0-3

Install libvirt, QEMU, and KVM:

    :::
    apt-get install qemu-kvm libvirt-bin virtinst



## Installing Windows 10
Create a Windows 10 VM with virt-install:

    :::
    virt-install --name win10 --memory 1024 --cdrom /storage/software/Microsoft/windows/SW_DVD5_WIN_ENT_10_64BIT_English_MLF_X20-26061.ISO --disk /var/lib/libvirt/images/win10.qcow2,size=30 --network=bridge:bridge-lan --graphics vnc,listen=0.0.0.0,password=test

Before starting the VM, we need to change the disk bus and network bus to VirtIO. This will give drastically better performance.

Edit the configuration:

    :::
    virsh edit win10

In the disk section, change (remember to remove <address>):

    :::
    From:
    <disk type='file' device='disk'>
      <driver name='qemu' type='raw'/>
      <source file='/var/lib/libvirt/images/win10.qcow2'/>
      <target dev='hda' bus='ide'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x06' function='0x0'/>
    </disk>

    To:
    <disk type='file' device='disk'>
      <driver name='qemu' type='raw'/>
      <source file='/var/lib/libvirt/images/win10.qcow2'/>
      <target dev='hda' bus='virtio'/>
    </disk>

In the network section:

    :::
    From:
    <interface type='bridge'>
      <mac address='52:54:00:39:33:64'/>
      <source bridge='bridge-lan'/>
      <model type='rlt8888'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x03' function='0x0'/>
    </interface>

    To:
    <interface type='bridge'>
      <mac address='52:54:00:39:33:64'/>
      <source bridge='bridge-lan'/>
      <model type='virtio'/>
    </interface>

Before installing, we also needs to mount a floppy with VirtIO drivers. The drivers can be obtained [here](https://fedoraproject.org/wiki/Windows_Virtio_Drivers).

Edit the Virtual machine, and add the following below the CDrom:

    :::
    <disk type='file' device='floppy'>
        <source file='/path/to/floppy'/>
        <target dev='fda'/>
    </disk>

The ISO with the QEMU guest agent and graphics drivers, can also be obtained from the Fedora Wiki.
The ISO can be downloaded and mounted inside the Windows VM or mounted as a CDROM by changing the configuration.



## Virsh usage

Get the VNC port:

    :::
    virsh vncdisplay <VM>

Edit the configuration of a VM:

    :::
    virsh edit <VM>

Start:

    :::
    virsh start <VM>

Stop/shutdown:

    :::
    virsh shutdown <VM>

Restart/reboot:

    :::
    virsh reboot <VM>

Set the VM to autostart:

    :::
    virsh autostart win10


## Configure for bridge

Install bridge-utils:

    :::
    apt-get install bridge-utils

Configure bridge in /etc/network/interfaces:

    :::
    auto eth0
    iface eth0 inet manual

    auto bridge-lan
    iface bridge-lan inet static
        address 192.168.1.10
        netmask 255.255.255.0
        gateway 192.168.1.1
        bridge_ports    eth0
        bridge_stp      off
        bridge_fd       0
        bridge_maxwait  0
