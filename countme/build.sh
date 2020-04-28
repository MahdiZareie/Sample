#!/bin/bash

mvn clean compile  package install
sudo docker build -t faster-server:latest .

