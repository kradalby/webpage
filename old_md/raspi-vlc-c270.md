---
title: Streaming video with Raspi, VLC and Logitech C270
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: raspberry pi, vlc, Logitech, c270
summary: Streaming video with Raspi, VLC and Logitech C270
---


In this post, I will go through a few steps to have a successful MJPEG stream in 720p over network from a Raspberry Pi with a Logitech C270 using VLC.

First of all, in this example, I was using Rasbian Wheezy with all the GUI stripped away. I have a Raspberry Pi with 256 MB ram, but that does not seem like a problem since the task at hand is not very ram intensive. Another thing to note is that I use MJPEG as my video format. The reason for this is that the C270 can send MJPEG and RAW. The Raspi cannot transcode video as it is a very weak computer, so using RAW is out of the picture. We will focus on just "forwarding" the video from the camera over network.

So lets get started, first we need some packets since I do not like to have X on a server we install the command line vlc version:

    :::bash
    sudo apt-get install vlc-nox v4l-utils

v4l-utils provides us with a utility called v4l2-ctl which we will use to set the camera to MJPEG mode and set the resolution:

    :::bash
    sudo v4l2-ctl --device=/dev/video0 --set-fmt-video=width=1280,height=720,pixelformat=1

In my case, the camera is located at /dev/video0 and the MJPEG is pixelformat 1.

To check is this is the case for you, issue the command:

    :::bash
    sudo v4l2-ctl --device=/dev/video0 --list-formats

After all the settings is set we can start the stream:

    :::bash
    cvlc v4l2:///dev/video0:chroma=mjpg:width=1280:height=720:fps=25 --sout '#standard{access=http,mux=mpjpeg,dst=:8080}' -vvv

The command is relatively self-explained. This sets up a stream from the video device to the computers IP over HTTP and port 8080.

If you would like to check out who is viewing your stream you can use this command:

    :::bash
    watch -n1 'netstat -tn | grep :8080 | grep ESTA
