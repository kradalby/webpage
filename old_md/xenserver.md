Title: Xenserver
Date: 2014-01-01 10:00
Modified: 2014-01-01 10:00
Tags: xenserver, xenserver tools, tools, centos, linux, vm, virtualization
Summary: Xenserver notes

[TOC]

## Installing Xenserver Tools on debian

Mount the ISO through Xencenter or Xen Orchestra.

Mount the ISO to the filesystem:

    :::
    mkdir /mnt/xs-tools
    mount /dev/xvdd /mnt/xs-tools

Run install script:

    :::
    cd /mnt/xs-tools/Linux
    ./install.sh
    reboot
    Title: Xenserver
    Date: 2014-01-01 10:00
    Modified: 2014-01-01 10:00
    Tags: xenserver, xenserver tools, tools, centos, linux, vm, virtualization
    Summary: Xenserver notes

    [TOC]

    ## Installing XenServer Tools on Debian

    Mount the ISO through Xencenter or Xen Orchestra.

    Mount the ISO to the filesystem:

        :::
        mkdir /mnt/xs-tools
        mount /dev/xvdd /mnt/xs-tools

    Run install script:

        :::
        cd /mnt/xs-tools/Linux
        ./install.sh
        reboot


    ## Apply hotfixes using the command line
    First, you will need to have the .xsupdate file on the server with xe.

    To upload the patch to Xenserver:

        :::
        xe patch-upload file-name=file.xsupdate

    This command will output a UUID that we need to copy, this will be used in the next command.

    To apply the patch to one server:

        :::
        xe patch-apply uuid=<patch-uuid> host-uuid=<host-uuid>

    To apply the patch to all servers:

        :::
        xe patch-pool-apply uuid=<patch-uuid>

    Reboot after the patches has been applied.


    ## Make it possible for VMs to autoboot
    By default XenServer will not let your VMs start with the XenServer host.

    Get pool uuid:

        :::
        xe pool-list

    To enable the autoboot for a Xenserver Pool:

        :::
        xe pool-param-set uuid=Pool-UUID other-config:auto_poweron=true

    ### Set a virtual machine to start at boot

    Find the VMs UUID:

        :::
        xe vm-list

    Set the VM to power on when the Xenserver starts:

        :::
        xe vm-param-set uuid=VM-UUID other-config:auto_poweron=true


    ## Add additional vif to VM

    Create a network for the new interface to be attached to, and returns the network-uuid

        :::
        xe network-create name-label="alice-network"

    Find out vm-uuid of the VM

        :::
        xe vm-list name-label=<vm's name-label>

    Create eth1 interface for the VM, and gets the UUID of the new interface

        :::
        xe vif-create vm-uuid=<vm-uuid> network-uuid=<network-uuid> device=1 mac=random

    Hot-plug the created interface to the VM

        :::
        xe vif-plug uuid=<new_vif_uuid>

    Now eth1 is visible from inside the VM.


## Apply hotfixes using the command line
First, you will need to have the .xsupdate file on the server with xe.

To upload the patch to Xenserver:

    :::
    xe patch-upload file-name=file.xsupdate

This command will output a UUID that we need to copy, this will be used in the next command.

To apply the patch to one server:

    :::
    xe patch-apply uuid=<patch-uuid> host-uuid=<host-uuid>

To apply the patch to all servers:

    :::
    xe patch-pool-apply uuid=<patch-uuid>

Reboot after the patches has been applied.


## Make it possible for VMs to autoboot
By default Xenserver will not let your VMs start with the Xenserver host.

Get pool uuid:

    :::
    xe pool-list

To enable the autoboot for a Xenserver Pool:

    :::
    xe pool-param-set uuid=Pool-UUID other-config:auto_poweron=true

### Set a virtual machine to start at boot

Find the VMs UUID:

    :::
    xe vm-list

Set the VM to power on when the Xenserver starts:

    :::
    xe vm-param-set uuid=VM-UUID other-config:auto_poweron=true


## Add additional vif to VM

Create a network for the new interface to be attached to, and returns the network-uuid

    :::
    xe network-create name-label="alice-network"

Find out vm-uuid of the VM

    :::
    xe vm-list name-label=<vm's name-label>

Create eth1 interface for the VM, and gets the uuid of the new interface

    :::
    xe vif-create vm-uuid=<vm-uuid> network-uuid=<network-uuid> device=1 mac=random

Hot-plug the created interface to the VM

    :::
    xe vif-plug uuid=<new_vif_uuid>

Now eth1 is visible from inside the VM.
