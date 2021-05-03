#!/bin/bash
cd /root
wget https://github.com.cnpmjs.org/ymcoming/realm/releases/download/V1.2.1/realm.zip && unzip realm.zip && chmod +x ./realm && mkdir /etc/realm && mv realm /etc/realm && mv config.json /etc/realm && mv realm.service  /etc/systemd/system && systemctl enable --now realm && systemctl start realm && systemctl status realm