Title: Setting up Windows Active Directory with BIND9 as DNS
Date: 2014-01-01 10:00
Modified: 2014-01-01 10:00
Tags: DNS, ad, active directory, windows, bind, bind9, Linux, Debian
Summary: Setting up Windows Active directory with BIND9

Some days ago I had to change out the DNS setup we had at work because of a security issue with the current DNS software we were running (Windows 2003 DNS).
Since I personally am a Linux guy I choose to setup BIND9 as our new DNS server on a Debian Jessie installation (right after the package freeze).

This is not a guide on how to migrate, I will just point out the steps I had to go through to get it working.

## Getting the current zone file
Since I had a Windows host I needed to get the zone configuration from I started googling. I am no DNS expert, so actually just doing an AXFR zone transfer was nothing that came to mind.

The zone files on Windows are located under %systemroot%\system32\dns. Which can easily be accessed by doing a run -> %systemroot%\system32\dns.

Here you can find a text file that actually ain't that far from the bind syntax we want.
So I copied the file and used vim to quickly edit it and created my BIND zone file.

After this, i configured BIND and started the service, but not everything was working. Active directory actually uses a few SRV records to get the correct location and information about the AD, which I did not have in my zone file.

## Adding the SRV records
After a little googling and a forum post by the user _ghight_ to [linuxquestion.org](http://www.linuxquestions.org/questions/linux-networking-3/howto-ms-active-directory-with-bind-on-linux-379377/) I was able to find the needed lines.

    :::bind
    _ldap._tcp.DOMAIN.COM. SRV 0 0 389 DCHOSTNAME.DOMAIN.COM.
    _kerberos._tcp.DOMAIN.COM. SRV 0 0 88 DCHOSTNAME.DOMAIN.COM.
    _ldap._tcp.dc._msdcs.DOMAIN.COM. SRV 0 0 389 DCHOSTNAME.DOMAIN.COM.
    _kerberos._tcp.dc._msdcs.DOMAIN.COM. SRV 0 0 88 DCHOSTNAME.DOMAIN.COM.

Obviously, you will need to change out DOMAIN.COM with your own hostname and update the DCHOSTNAME to your Active Directory server.
