Title: Why I think ssh config is more practical than alias
Tags: ssh, ssh config, config, alias, SCP, mosh
Date: 2015-03-09 16:44
Modified: 2015-03-09 19:44
Summary: Why I think ssh config is more practical than alias

[TOC]

Some years ago I decided to use the ssh user config instead of using an alias in my .zshrc file.

I have over the years found some practicalities of using the config file instead of aliases.

## Configuration of a host
In general, I think the ssh config is more understandable and better structured than a bunch of aliases.

### General host
ssh config:

    :::
    Host example
        Hostname host.example.com
        Port 22010
        User root

command:

    :::
    ssh example

alias:

    :::
    alias example='ssh host.example.com -p 22010 -l root'

### Connection with local portforwarding
ssh config:

    :::
    Host example
        Hostname host.example.com
        Port 22010
        User root
        LocalForward 6000 192.168.1.10:6000
        LocalForward 6001 192.168.1.10:6001

command:

    :::
    ssh example

alias:

    :::
    alias example='ssh host.example.com -p 22010 -l root -L 6001:192.168.1.10:6001 -L 6000:192.168.1.10:6000'

## ssh config advantages
My favorite advantage with an ssh config is that it works with all the other ssh related commands: rsync, scp, sftp and mosh.

You can for example do:

    :::
    mosh example

Or move a file with:

    :::
    scp somefile.txt example:/path/to/some/dir
