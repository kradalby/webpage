---
title: IP Plan
date: 2014-01-01 10:00
modified: 2014-01-01 10:00
tags: servers
summary: IP overview for fap.no
---

[TOC]

## Terrahost

Public: 185.125.169.220
Servers: 10.60.0.0/24
VPN: 10.60.8.0/24

| IP        | Name    | Service | Alt DNS           |
|-----------|---------|---------|-------------------|
| 10.60.0.1 | horse   | PFsense | terra.fap.no      |
| 10.60.0.2 | dewgong | ESXi    | esxi.terra.fap.no |
| 10.60.0.5 | -       | LDAP/AD | ldap.terra.fap.no |

## NTNU 1

Public: 129.241.210.115
Servers: 10.60.10.0/24
VPN: 10.60.18.0/24

| IP         | Name    | Service | Alt DNS           |
|------------|---------|---------|-------------------|
| 10.60.10.1 | diglett | PFsense | ntnu1.fap.no      |
| 10.60.10.2 | machamp | ESXi    | esxi.ntnu1.fap.no |
| 10.60.10.5 | slowbro | LDAP/AD | ldap.ntnu1.fap.no |


## NTNU 2

Public: 129.241.210.106
Servers: 10.60.20.0/24
VPN: 10.60.28.0/24

| IP         | Name    | Service | Alt DNS           |
|------------|---------|---------|-------------------|
| 10.60.20.1 | -       | PFsense | ntnu2.fap.no      |
| 10.60.20.2 | onix    | ESXi    | esxi.ntnu2.fap.no |

## OAV

Public: 84.52.231.133
Servers: 10.60.30.0/24
VPN: 10.60.38.0/24

| IP          | Name       | Service | Alt DNS           |
|-------------|------------|---------|-------------------|
| 10.60.30.1  | butterfree | PFsense | oav.fap.no        |
| 10.60.30.20 | kramac     | iMac    | -                 |
