#!/bin/bash

container_name="spike-a1hilfe-mdbook"

# Check if the container exists
if docker inspect "$container_name" > /dev/null 2>&1; then
    echo "The container $container_name exists."
    
    # Check if the container is running
    if $(docker inspect -f '{{.State.Status}}' "$container_name" | grep -q "running"); then
        echo "The container $container_name is running - stopping it now"
        docker stop "$container_name"
    fi

    echo "Removing container $container_name"
    docker rm "$container_name"
fi