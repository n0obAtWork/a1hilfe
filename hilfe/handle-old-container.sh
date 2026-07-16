#!/bin/bash

name=$1

# Check if the container exists
if docker container inspect "$name" > /dev/null 2>&1; then
    echo "The container $name exists."
    
    # Check if the container is running
    if $(docker inspect -f '{{.State.Status}}' "$name" | grep -q "running"); then
        echo "The container $name is running - stopping it now."
        docker stop "$name"
    fi

    echo "Removing container $name"
    docker rm "$name"
else
    echo "The container $name does not exist."
fi