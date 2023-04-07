---
title: Using Nginx to serve bootstrap script
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: nginx, bootstrapping, Debian, lazy
summary: Using Nginx to serve bootstrap script
---

In this post, I will walk through the easy steps on how to host a bootstrap script on Github and make it possible to proxy it through a shorter domain.

The goal is to be able to setup the basic needed stuff on a new server with one simple command. The same way you install Oh my zsh and Brew.

I choose to host the script on Github as this provides me with flexibility and ease when I need to update and keep track of it.

In this example, I will use [this](https://raw.githubusercontent.com/kradalby/scripts/master/bs.sh) which I use to bootstrap Debian servers I use.

In my Nginx configuration file for the used domain, I have this:

    :::bash
    location /bs.sh {
        proxy_pass https://raw.githubusercontent.com/kradalby/scripts/master/bs.sh;
    }

This makes the Github script available at: https://kradalby.no/bs.sh

To use the script on a machine you will need curl or wget:

curl:

    :::bash
    curl https://kradalby.no/bs.sh | bash

We can do the same with wget:

    :::bash
    wget https://kradalby.no/bs.sh -O - | bash

You can add the -k option in curl and --no-check-certificate in wget to skip certificate check. If you use self-signed certs, this will be needed.
