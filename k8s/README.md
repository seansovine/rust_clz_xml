# Running our app in Kubernetes

We will start by running the Deno web app in Kubernetes,
and will include instructions on how to run the app in
Minikube. Later we will build out the Kubernetes app more
by adding more services and maybe running our database in
a stateful pod.

## Running the web app in Minikube

First we need to build the container image:

```shell
cd $PROJECT_ROOT/webapp && \
	docker build -t deno-novolume -f docker/deno.no-volume.dockerfile .
```

Now start a Minikube cluster with

```shell
minikube start
```

Now load the container image into Minikube with

```shell
minikube image load deno-novolume
```

We can see the loaded image with

```shell
minikube image ls
```

*Create Kubernetes deployment:*

We can create the deployment config with

```shell
kubectl create deployment deno-web-app --image=deno-novolume --dry-run=client -o yaml \
	> deploy-deno-web-app.yaml
```

and in the output YAML file add

```yaml
imagePullPolicy: Never # Since we use a local image.
```

after the container name.

Now we can create the running deployment with

```shell
kubectl apply -f deploy-deno-web-app.yaml
```

and see it and its pods with

```shell
kubectl get deployments
kubectl get pods
```

We can view the logs of the container running in the pod with

```shell
kubectl logs deno-web-app-<ID FROM get pods COMMAND>
```

to verify that everything has started up correctly.

*Expose the pod's port outside of Minikube:*

```shell
kubectl expose deployment deno-web-app --type=NodePort --port 8000
```

and verify it with

```shell
kubectl get services
```

This will show the external port mapped to the pod's server port `8000`.

Then we can find the Minikube external IP address using

```shell
minikube ip
```

or using

```shell
minikube service deno-web-app --url
```

And we can navigate to the `IP:port` we've discovered by these
commands and verify that our web app is running.

However, at this point, viewing the logs using the command above shows
that there is a database connection error. That will be fixed in the
next section.

*Clean up:*

To clean up the deployment and Minikube cluster
after we're done using them, we can run:

```shell
kubectl delete services deno-web-app
kubectl delete deployments deno-web-app
minikube stop
minikube delete
```

## Connecting to the Minikube app

Eventually we will consider adding the database to our pod as a stateful
service, but for now we will keep running it locally. We can do this
locally, and run it in Docker Compose by using

```shell
cd $PROJECT_ROOT && \
	docker compose up -d mariadb
```

TODO: Finish this section by showing how to make the external service
available to the Kubernetes deployment.

## Resources

For getting started with Minikube:

+ [Hello Minikube](https://kubernetes.io/docs/tutorials/hello-minikube/)

Helpful tutorial on deploying a basic app with Minikube:

+ [Minikube tutorial](https://medium.com/@areesmoon/writing-and-deploying-your-first-app-on-minikube-81c373089e10)
