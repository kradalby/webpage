Title: Strider Continous Deployment
Tags: Strider, strider, stridercd, cd, continuous, deployment, development
Date: 2014-12-13 21:41
Modified: 2014-12-19 18:27
Summary: Installation and notes for Strider CD

[TOC]

I have been looking for an alternative to Jenkins for personal use for a little while and after some googling I come over Strider CD, which actually focus on both integration and deployment. It is written in Node and have a very nice interface. The reason I wanted to stay away from Jenkins is the cluttered interface and the memory hungry Java, it also has a lot of features I do not need.

The rest of this post is documentation on how I installed and configured Strider. Over time, it will probably change as I add more features to my installation beyond Python testing and building.

## Installation
I use Debian as my server OS, for this install I use the latest Debian Jessie build.

Since the Node version in the Debian testing repositories is quite new, you can just install it.

    :::
    apt-get install nodejs

We will then run npm update and npm install:

    :::
    npm update
    npm install strider

Since npm does not offer to set up a user to run Strider as we will also do this:

    :::
    useradd -m -d /var/lib/strider -s /bin/sh strider

Note that I do not set a password for the strider user, as it should not have to log in ever.

### Configuration
Also, strider is configured via environment variables.so we will create a file with the following content for later use with systemd. This is the configuration variables I use, you may need less/more.

    :::
    PLUGIN_GITHUB_HOSTNAME=
    PLUGIN_GITHUB_APP_ID=
    PLUGIN_GITHUB_APP_SECRET=
    STRIDER_HOSTNAME=
    SERVER_NAME=
    SMTP_HOST=
    SMTP_FROM="Strider woot@example.com"
    NODE_ENV=production
    PORT=3000

I also do not change the port since I am going to proxy it behind a Nginx instance anyway.

To work with the systemd example below, save it at /var/lib/strider/settings

### systemd
Since systemd is the standard init manager on Jessie, I will use this, it is also convenient since it needs very little configuration.

Below is my systemd file for strider:

    :::
    # systemd service for StriderCD

    [Unit]
    Description= Strider CD
    After=network-online.target

    [Service]
    EnvironmentFile=-/var/lib/strider/settings
    User=strider
    ExecStart=/usr/bin/strider
    Restart=always
    SyslogIdentifier=strider

    [Install]
    WantedBy=multi-user.target

Save this to /etc/systemd/system/strider.service.

Reload systemd:

    :::
    systemctl daemon-reload

Start Strider:

    :::
    systemctl start strider.service

Check that its running:

    :::
    systemctl status strider.service

Strider should now be available at http://serverip:3000/

## Github
Strider has a good integration with Github, which allows it to watch and automatic build projects on different triggers. I had a little hassle with setting it up, but here's how it went down.

### Create app
To use our Strider installation with Github, we need to setup a Developer application.

This can be done at [github.com/settings/applications](https://github.com/settings/applications)

My personal app is just named strider, and the homepage URL should be for example http://strider.example.org/

The most important part of the app creation is the Authorization callback URL.

This _MUST_ be http://url-to-strider/auth/github/callback

After the app is configured and saved, take note of the ID and Secret as we are going to use them later on.

Also, for some ridiculous reason, you must also have a public email on your GitHub account.

### Strider configuration
Now we need to tell Strider to use our app and configure so the GitHub auth will succeed.

Edit /var/lib/strider/settings:

    :::
    PLUGIN_GITHUB_HOSTNAME=http://strider.example.org
    PLUGIN_GITHUB_APP_ID=xxxxx
    PLUGIN_GITHUB_APP_SECRET=xxxxx
    STRIDER_HOSTNAME=http://strider.example.org
    SERVER_NAME=http://strider.example.org

Save the file, restart strider and try to authorize with GitHub.

## Python
To use Python with Strider, follow these steps.

Make sure the python plugin for strider is installed:

    :::
    npm install strider-python

Install python and pip

    :::
    apt-get install python python-pip python3 python3-pip

Install virtualenv

    :::
    pip install virtualenv
