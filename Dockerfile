# Use the official Ubuntu base image
FROM ubuntu:latest

# Update and install iproute2 (for 'ip' command)
RUN apt-get update && apt-get install -y iproute2

# Set the working directory inside the container
WORKDIR /app
