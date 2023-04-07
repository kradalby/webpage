---
title: Setting Docker options in Debian 8
tags: docker, debian, systemd, jessie
date: 2016-10-26 16:30
modified: 2016-10-26 16:50
summary: By default, Docker with systemd does not obey /etc/default/docker, lets fix it.
---

When using Debian 8 and the latest official Docker installed with the official repository from docker, the systemd configuration does not obey the /etc/default/docker configuration file. This is a little bit annoying as it is a logical and convention to put arguments for the daemon there.

This can easily be fixed by changing the systemd service file for Docker. Let's do it!

What we are going to do is to pass `/etc/default/docker` to systemd as an environment file and then pass the `DOCKER_OPTS` variable to the Docker daemon.

Lets open `/lib/systemd/system/docker.service`:

    :::
    [Unit]
    Description=Docker Application Container Engine
    Documentation=https://docs.docker.com
    After=network.target docker.socket
    Requires=docker.socket

    [Service]
    Type=notify
    # the default is not to use systemd for cgroups because the delegate issues still
    # exists and systemd currently does not support the cgroup feature set required
    # for containers run by docker
    EnvironmentFile=-/etc/default/docker
    ExecStart=/usr/bin/dockerd $DOCKER_OPTS -H fd://
    ExecReload=/bin/kill -s HUP $MAINPID
    # Having non-zero Limit*s causes performance problems due to accounting overhead
    # in the kernel. We recommend using cgroups to do container-local accounting.
    LimitNOFILE=infinity
    LimitNPROC=infinity
    LimitCORE=infinity
    # Uncomment TasksMax if your systemd version supports it.
    # Only systemd 226 and above support this version.
    #TasksMax=infinity
    TimeoutStartSec=0
    # set delegate yes so that systemd does not reset the cgroups of docker containers
    Delegate=yes
    # kill only the docker process, not all processes in the cgroup
    KillMode=process

    [Install]
    WantedBy=multi-user.target


Simply add EnvironmentFile and pass the variable to the ExecStart parameter:

    :::
    EnvironmentFile=-/etc/default/docker
    ExecStart=/usr/bin/dockerd $DOCKER_OPTS -H fd://


Reload systemd:

    :::
    systemctl daemon-reload

Now you can edit `/etc/default/docker` and restart Docker!
