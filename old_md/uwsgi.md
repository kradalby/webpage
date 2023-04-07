Title: uWSGI and Nginx on Debian Wheezy
Date: 2014-01-01 10:00
Modified: 2014-01-01 10:00
Tags: nginx, uwsgi, Django, python, Debian
Summary: Setup instructions for uWSGI on Debian Wheezy.

[TOC]


Preconditions: Nginx is installed on the server.

We will set up uWSGI in emperor mode, which basically means that a uWSGI daemon will spawn a master with workers for every uWSGI application we got.


## Install uWSGI
We are going to install uWSGI via pip, so we need to install:

    :::bash
    apt-get install build-essential python python-dev python-pip

Note: If you want to run python3, use python3, python3-dev and python3-pip.

Now install the latest version through pip:

    :::bash
    pip install uwsgi

Create a /etc directory for the uWSGI application ini files:

    :::bash
    mkdir -p /etc/uwsgi/apps-available
    mkdir -p /etc/uwsgi/apps-enabled

Now get the init script:

    :::bash
    cd /etc/init.d
    wget https://gist.githubusercontent.com/kradalby/3252b8bacca6622bf864/raw/0d6d2b034284a8256c646433782ee0217b04c437/uwsgi

    chmod +x uwsgi

    # Make it start automatically
    update-rc.d uwsgi defaults

[Link to gist](https://gist.github.com/kradalby/3252b8bacca6622bf864)

Create log directory:

    :::bash
    mkdir /var/log/uwsgi
    chown -R nginx.nginx /var/log/uwsgi

## Application configuration

### Python/Django
Currently i only got Django applications running on uWSGI. Below you will find a sample config from one of my applications:

    :::bash
    [uwsgi]
    socket          = /srv/www/dfektlan.no/lanweb/dfektlan.sock
    chdir           = /srv/www/dfektlan.no/lanweb
    wsgi-file       = dfektlan/wsgi.py
    env             = DJANGO_SETTINGS_MODULE=dfektlan.settings
    chmod-socket    = 664
    home            = /srv/www/dfektlan.no/env
    vacuum          = true
    master          = true
    processes       = 10
    daemonize       = /var/log/uwsgi/dfektlan.log


## Controlling uWSGI

With the provided init script you can use the Debian standard service command to control uWSGI:

Stop, start restart:

    :::bash
    service uwsgi start
    service uwsgi stop
    service uwsgi restart

Reload:

    :::bash
    service uwsgi reload

    # Or the send command directly
    uwsgi --reload /var/run/uwsgi/uwsgi.pid

A neat feature uWSGI also has, is auto reload if a config file for an application is changed. So every time you change a .ini file which is linked to the apps-enabled directory it will reload that particular app.


## Setting up a Nginx vhost with uWSGI
The first thing we will do is to define an upstream.

A Nginx upstream is a group of server/resources that can be referenced by the pass methods, like proxy, FastCGI or uwsgi.
The load is distributed between the given servers using a round-robin balancing method. Parameters that can be passed can be found [here](http://nginx.org/en/docs/http/ngx_http_upstream_module.html)

The upstream used with the uWSGI configuration above looks like this:

    :::
    upstream dfektlan {
        server unix:///srv/www/dfektlan.no/lanweb/dfektlan.sock;
    }


And then we need to define the uwsgi_pass inside a location where we want the uWSGI app to be available:

    :::
    location / {
        uwsgi_pass  dfektlan;
        include     /etc/nginx/uwsgi_params;
    }

Now restart Nginx and the application should be available.


# FreeBSD

portnap fetch update
-cd /usr/ports/www/uwsgi
-make install
-cd /usr/ports/www/uwsgi/work/uwsgi-2.0.11.2

get source
python uwsgiconfig.py --build core
PYTHON=python2.7 ./uwsgi --build-plugin "plugins/python python27"
PYTHON=python3.5 ./uwsgi --build-plugin "plugins/python python35"
mv python*plugin* /usr/local/lib/uwsgi/
mv uwsgi /usr/local/bin/
