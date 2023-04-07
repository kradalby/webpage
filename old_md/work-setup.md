Title: Work setup
Date: 2014-01-01 10:00
Modified: 2014-01-01 10:00
Tags: work
Summary: Documentation showing work setup.
URL: private/work-setup.html
save_as: private/work-setup.html

[TOC]

## Setup

### Office Computer

    :::
    Hostname: kontor1
    Username: kavik
    IP: 192.168.1.101
    MAC: 84:2B:2B:AC:BA:D2
    Language: Norwegian

### Anders Computer

    :::
    Hostname: kontor2
    Username: kavik
    IP: 192.168.1.102
    MAC: 78:2B:CB:84:02:96
    Language: Norwegian

## Software

### Microsoft Security Essentials
Get it [here](http://windows.microsoft.com/nb-no/windows/security-essentials-download).

### Microsoft Office
Get it from Dropbox or Snorlax.

### LibreOffice
Get it [here](http://www.libreoffice.org/download/libreoffice-fresh/).

### Adobe Reader
Get it [here](http://get.adobe.com/reader/otherversions/).

### Thunderbird
Get it [here](http://www.mozilla.org/en-US/thunderbird/).

Configurated with lager@klatrerosen.no

### Dropbox
Get it [here](https://www.dropbox.com/install).

Configurated with lager@klatrerosen.no

### BullZip
Get it [here](http://www.bullzip.com/products/pdf/download.php).

### Tupo
Get it from Dropbox or Snorlax.

Copy acucorp to C:\

Copy shortcut to Desktop

Install Fonts

### TWSDATA and DAS
To be able to send files to Tupperware Nordic, and to do web order stuff, we need twsdata and DAS, get them from Dropbox.

#### TWSDATA
Copy folder to the root of the C Drive.
Start the sbsdatac application, it will give a user not set error.
Click ok, and go to parameters, check settings and click ok.

#### DAS
To install, go into DAS folder and web communicator. From there make a shortcut from WebCommunicatorReader to the desktop.

Open it, and click on configure.
Username: Kristine-Breiland.Dalby
Password: Atomic

Make sure that the drive letter for every folder is correct and is on the network drive.
Also, make sure that the distributor number is correct: 2045.


### Java
Get it [here](http://java.com/en/download/manual.jsp).

### Flash
Get it [here](http://get.adobe.com/flashplayer/otherversions/).

### Internet Explorer
Homepages:

    :::
    https://www.google.no/
    http://81.7.144.10/login.aspx?

## Printers

### Canon iRC2380i

    :::
    Model: Canon iRC2380i
    IP: 192.168.1.5
    Driver: Canon UFR II/ UFRII LT Printer Driver v14.00

Get it [here](http://software.canon-europe.com/software/0043699_0010555.asp?model=)

Installing the driver must be done from the extracted directory.

Printer setup on both machines has one color and one black/white.

#### Mac
To get this printer working on OS X Mavericks:

http://software.canon-europe.com/software/0044814.asp

When you have installed the software, you need to add the printer by going to IP and select Line Printer Daemon (LPD) as the protocol. Then select the correct software and it should be good.

Apple discussion: https://discussions.apple.com/message/23658255

## Remote control

### LogMeIn
Since it is Tupperware Nordic that uses LogMeIn, we need them to send an install link.
Contact Olga: OlgaPalashina@tupperware.com

### VNC
Get it [here](http://www.tightvnc.com/download.php).

VNC is set up with the default port: 5900.

## Misc

### Windows Shares
Windows drives are currently installed in this manner:

    :::
    Status       Lokalt    Eksternt                  Nettverk

    -------------------------------------------------------------------------------
    OK           P:        \\192.168.1.3\programmer         MWN
    OK           T:        \\192.168.1.3\data               MWN
    OK           Z:        \\192.168.1.3\data\Dokument      MWN
