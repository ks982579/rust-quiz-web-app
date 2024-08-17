#!/bin/bash

# Check if certificates exist and are valid
if [ ! -f "./certs/fullchain.pem" ] || [ ! -f "./certs/privkey.pem"] || \
  ! open
