---
title: Docker
tags: docker, container, ci, continuous integration, Linux, Debian
date: 2016-04-09 03:01
modified: 2016-06-19 22:23
summary: How I should build docker images for Drone CI.
---

[TOC]

I use Docker sometimes. Currently, I use a CI called Drone, which runs tests in Docker containers customized for the project (usually to mimic the production environment). I like this a lot as I don't get one big blob with everything installed for all the different projects dependencies. This page has the docker commands that i need to use to build and push the different images.


Build image from Dockerfile (where . is the location of a dir with a Dockerfile):

    docker build -t <username>/<imagename>:latest .

To push the image to hub.docker.com, run the following command. NOTE that the repository must already be created on docker hub:

    docker push <username>/<imagename>:latest

Start a Docker image with shell

    docker run -it <username>/<imagename> /bin/sh

To list docker images:

    docker images

Sometimes you have a lot of images with identifier <none>, delete them:

    docker images | grep none | awk '{print $3}' | while read i
    do
    docker rmi -f $i
    done

Alternatively:

    docker rmi (docker images | awk '{ print $3; }')

To list running docker containers:

    docker ps
