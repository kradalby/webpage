Title: Sympa
Date: 2014-01-01 10:00
Modified: 2014-01-01 10:00
Tags: sympa, mail, maillist, mailinglist, Debian
Summary: How I install Sympa on Debian

[TOC]

## Install Sympa

Install Sympa using apt:

    :::bash
    apt-get --no-install-recommends install sympa libmime-types-perl libxml-sax-expat-perl libfile-nfslock-perl libsoap-lite-perl libcrypt-ciphersaber-perl libmail-dkim-perl doc-base mysql-server

Note: The command over will prevent Sympa from installing apache. It will install MySQL, if you want to use PostgreSQL, remove mysql-server.

## WWSympa with Nginx
First of all, we need to make Sympa use FastCGI. This is done by setting use_fast_cgi to 1 in /etc/sympa/wwsympa.conf.

### Install fcgiwrap
We need the fcgiwrap package to be able to execute the Sympa fcgi files.

    :::bash
    apt-get install fcgiwrap

It is important to change the fcgiwrap user in the init.d script to nginx instead of www-data.

### Configuring Nginx

In your Nginx configuration directory create a file called sympa_fcgi.conf. This will contain all the fastcgi settings for sympa. Add the following content:

    :::
    gzip off;
    fastcgi_pass   unix:/run/fcgiwrap.socket;
    fastcgi_split_path_info ^(/sympa)(.+)$;
    fastcgi_param  QUERY_STRING       $query_string;
    fastcgi_param  REQUEST_METHOD     $request_method;
    fastcgi_param  CONTENT_TYPE       $content_type;
    fastcgi_param  CONTENT_LENGTH     $content_length;
    fastcgi_param  PATH_INFO          $fastcgi_path_info;
    fastcgi_param  SCRIPT_NAME        $fastcgi_script_name;
    fastcgi_param  REQUEST_URI        $request_uri;
    fastcgi_param  DOCUMENT_URI       $document_uri;
    fastcgi_param  DOCUMENT_ROOT      $document_root;
    fastcgi_param  SERVER_PROTOCOL    $server_protocol;
    fastcgi_param  GATEWAY_INTERFACE  CGI/1.1;
    fastcgi_param  SERVER_SOFTWARE    nginx;
    fastcgi_param  REMOTE_ADDR        $remote_addr;
    fastcgi_param  REMOTE_PORT        $remote_port;
    fastcgi_param  SERVER_ADDR        $server_addr;
    fastcgi_param  SERVER_PORT        $server_port;
    fastcgi_param  SERVER_NAME        $server_name;
    fastcgi_param  REMOTE_USER        $remote_user;
    fastcgi_param  SCRIPT_FILENAME    $document_root/wwsympa-wrapper.fcgi;
    fastcgi_param  HTTP_HOST          $server_name;
    fastcgi_intercept_errors on;

Then we need to create a vhost for every domain you want mailing list for. I have them all in one file called sympa_vhost.conf and the first entry looks like this:

    :::
    server {
        listen       80;
        server_name  lists.klatrerosen.no;                                  
        root         /usr/lib/cgi-bin/sympa;
        access_log   /var/log/nginx/lists.klatrerosen.no.access.log main;   
        error_log    /var/log/nginx/lists.klatrerosen.no.error.log;         
        error_page   403 500 502 503 504 /50x.html;

        rewrite ^/$ http://lists.klatrerosen.no/sympa/ permanent;           

        location /static-sympa {
            alias /var/lib/sympa/static_content/;
            access_log off;
        }

        location /50x.html {
            root /usr/share/nginx/html;
        }

        location ~* \.(php|pl|py|jsp|asp|sh|cgi|bin|csh|ksh|out|run|o)$ {
            deny all;
        }

        location ~ /\.ht {
            deny all;
        }

        location / {
            include sympa_fcgi.conf;
        }
    }

Now restart Nginx and you should be good to go.


## Configuring virtual hosts
When creating a sympa vhost we need a couple of files and directories:

    :::
    /etc/sympa/my.domain.tld/robot.conf
    /var/lib/sympa/list_data/my.domain.tld/
    /etc/nginx/conf.d/sympa_vhost.conf

The vhost configuration information can be found in the Nginx part.

### robot.conf
Is the file that Sympa itself uses to define a vhost. It holds information about listmaster, who can create a list, where the web interface is located and the name. It has to be located in the Sympa configuration directory within a folder that is named after the domain it will cover.

Example:

    :::

### Sympa list_data directory for domain
Sympa also needs a place to save list data. This will be in /var/lib/sympa/list_data/ in a folder named after the domain. To create:

    :::bash
    mkdir /var/lib/sympa/list_data/my.domain.tld
    chown -R sympa:sympa /var/lib/sympa/list_data
