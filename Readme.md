# Rust Server (template)

## Build using docker

How to build and run debian docker image

docker build -t rust-template-server -f ./debian/Dockerfile .

docker run -p 8000:8000 rust-template-server


The image won't stop with ctrl-c, need to run "docker ps" and "docker stop ${image_name}" to stop container. Where ${image_name} is the name of the container found with ps.

A nameless container with compilation data (2GB) will be created when building. To remove it run "docker system prune". It won't damage the 80MB compiled project.

To remove compiled image run "docker ps -a" then "docker rmi ${image_name} --force" where ${image_name} is the name of the newly created image.

## How to build NPM scripts (using docker):

 - build-docker: Build a docker image of this project.
 - run-docker: Run the previously built docker. Will need to open another terminal to close it. ("docker ps" then "docker stop ${container_name}", replace ${container_name} with the name of the container).
 - remove-docker: Remove the image built previously.
 - clear-docker: Clear the useless image built when compiling. And any other dangling images/containers.
 - build-local: Compile in a container, then copy the compiled file into the local "./target/release/" folder.


## Note

Didn't have time to add much to this project yet. But more will come, like testing and "simpler" builds.

## Testing

Will make automated testing later.

Manually we can test by running the docker container and opening request.http file using vscode.

If REST CLIENT (by Huachao Mao) is installed. There will be "Send request" written on top of the endpoints. Clicking them will send the requests and expect a response. (Hopefully) You will get a response.

