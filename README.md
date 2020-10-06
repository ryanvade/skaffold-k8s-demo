# Skaffold K8S Demo

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/a2b4d922f376483e95ad78aea9823821)](https://app.codacy.com/gh/ryanvade/skaffold-k8s-demo?utm_source=github.com&utm_medium=referral&utm_content=ryanvade/skaffold-k8s-demo&utm_campaign=Badge_Grade)

## Setup

1.  Install [Minikube](https://kubernetes.io/docs/tasks/tools/install-minikube/)
2.  Install [Skaffold](https://skaffold.dev/docs/install/)
3.  Install [Docker](https://docs.docker.com/get-docker/)
4.  Install [Istio](https://istio.io/latest/docs/setup/getting-started/)

## Initialize Kubernetes Cluster

1.  Run Minikube `minikube start --cpus=4 --memory 8096 --disk-size 32g`
2.  Install Istio in the cluster `istioctl install --set values.kiali.enabled=true --set values.tracing.enabled=true`
3.  In a separate terminal, open a Minikube tunnel `minikube tunnel`
4.  Install the services in the cluster `skaffold run`
5.  Open the Ingress Gateway `minikube -n istio-system service istio-ingressgateway`

-   Note: Service A is at istio-gateway:80/a and Service B is at istio-gateway:80/b
