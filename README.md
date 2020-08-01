# Getting started

## Use Docker to build and run

``` bash
$ git pull kwaa/ssvm-nodejs-starter:v1
$ docker build -t ssvm-nodejs:v1 .
$ docker run -p 3000:3000 --rm -it -v $(pwd):/app ssvm-nodejs:v1
(docker) # cd /app
(docker) # cargo build
(docker) # ssvmup build
(docker) # node node/app.js
```

From a second terminal window, you can test the local server.

``` bash
$ curl http://localhost:3000/?length=10
aaaaaaaaab
```
