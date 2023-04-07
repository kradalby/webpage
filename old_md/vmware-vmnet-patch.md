Title: Patch VMware vmnet for kernel 3.19 and up
Tags: vmware, vmnet, kernel, 3.19, kernel 3.19, kernel 4.0, 4.0
Date: 2015-06-05 20:42
Modified: 2015-06-05 20:42
Summary: Patch VMware vmnet for kernel 3.19 and up

[TOC]

On one of my servers, I run VMware player. Since Linux kernel 3.19, the vmnet module has not compiled correctly. I am currently using Linux kernel 4.0.4 and the problem persists. Luckily, there is a patch available.

Download the patch:

    :::
    curl http://pastie.org/pastes/9934018/download -o /tmp/vmnet-3.19.patch

Extract the vmnet source:

    :::
    cd /usr/lib/vmware/modules/source
    tar -xf vmnet.tar

Apply the patch:

    :::
    patch -p0 -i /tmp/vmnet-3.19.patch

Recreate the archive:

    :::
    tar -cf vmnet.tar vmnet-only
    rm -rf *-only

Rebuild the modules:

    :::
    vmware-modconfig --console --install-all
