---
title: Installing Nginx on Debian Wheezy
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: webserver wheezy Debian nginx
summary: Installation guide for the latest Nginx on Debian Wheezy
---

[TOC]

## Add the Official Nginx repository

Download the signed key and add it:

    :::
    wget http://nginx.org/keys/nginx_signing.key
    apt-key add nginx_signing.key

Then add the repos to source.list:

    :::
    deb http://nginx.org/packages/debian/ codename nginx
    deb-src http://nginx.org/packages/debian/ codename nginx

Remember to swap out codename for the current Debian version, as of now, wheezy.

## Install
Now that we have repositories for installing Nginx we can go ahead and do so:

    :::
    apt-get update
    apt-get install nginx


## Running a Django app from a suburl
If we want to run a Django application from a suburl (kradalby.no/someapp) we need to make the app aware of that path when we send the url to it.
If we use a proxy to send request, add this line:

    :::
    proxy_set_header SCRIPT_NAME /myapp;


## WebSocket proxy
To proxy websockets through nginx we need to pass upgrade through the Connection proxy header like this:

    :::
    location /wsapp/ {
        proxy_pass http://wsbackend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }


## Node app proxy

To proxy most node apps I have been using, you will need to have a subdomain for the app.
The settings below where used to proxy strider cd.

    :::
    location / {
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header HOST $http_host;
        proxy_set_header X-NginX-Proxy true;

        proxy_pass http://127.0.0.1:8002;
        proxy_redirect off;
    }

## Google verification
To verify that you own a domain to google, you can simply add a return statement for the url the where the verification file would be:

    :::
    location = /googled1085b59adc211cd.html {
            rewrite ^/(.*)  $1;
            return 200 "google-site-verification: $uri";
    }
