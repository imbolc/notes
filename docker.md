Docker / Podman
===============

Images
------
- pull: `podman pull alpine`
- list: `podman image ls`

Containers
----------
- list all: `podman container ls -a`
- list running: `podman container ls`
- run interactively: `podman run -it alpine`


Create image using this directory's Dockerfile:

    docker build -t <image-name>

Run container:

    docker run -p 4000:80 friendlyhello

Run in background:

    docker run -d -p 4000:80 friendlyhello
    docker container ls

Stop, kill or remove container:

    docker container stop <hash>
    docker container kill <hash>
    docker container rm <hash>

Publish repository:

    docker login
    docker tag container username/repository:tag
    docker image ls
    docker push username/repository:tag

Pull and run from repository:

    docker run -p 4000:80 username/repository:tag
