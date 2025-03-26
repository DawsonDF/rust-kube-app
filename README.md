# Rust Application - Minikube Deployment Guide

This repository contains a very simple Rust application that can be deployed to a local Kubernetes cluster using Minikube.
This is for learning purposes to practice K8s and does not do anything interesting.

## Getting Started

- [Docker](https://docs.docker.com/get-docker/)
- [Minikube](https://minikube.sigs.k8s.io/docs/start/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [Rust](https://www.rust-lang.org/tools/install)

## Project Structure

```
.
├── src/                 # Rust application source code
├── Cargo.toml           # Rust package configuration
├── Cargo.lock           # Dependency lock file
├── Dockerfile           # Docker container definition
├── deployment.yaml      # Kubernetes deployment configuration
└── README.md            # This file
```

## Deployment Instructions

### Start Minikube

```bash
minikube start
```

### Point to Minikube's Docker Daemon

```bash
eval $(minikube docker-env)
```

### Build the Docker Image

```bash
docker build -t rust-kube-app .
```

### Deploy to Minikube cluster

```bash
kubectl apply -f deployment.yaml
```

### Validate deployment worked

```bash
kubectl get deployments
kubectl get pods
kubectl get services
```

### Start the tunnel for the service

```bash
minikube service rust-kube-app-service
```

## Updating the Application

### Rebuilding the Docker Image

```bash
docker build -t rust-kube-app .
```

### Restarting the Deployment

```bash
kubectl rollout restart deployment/rust-kube-app
```

### Stopping the Cluster
```bash
minikube stop
```
