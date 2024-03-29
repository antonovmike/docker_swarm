# docker_swarm

The algorythm creates "config.yml" file

(Fix description)

STEP 1

Choose the image and run it
```bash
sudo docker image ls
sudo docker run -it IMAGE_NAME sh
```

Or download and run:
```bash
sudo docker run -it --rm rust /bin/sh
```

STEP 2

Clone project
```bash
git clone https://github.com/antonovmike/docker_swarm.git
```

STEP 3

Start project
```bash
cd docker_swarm
cargo run
```

Build:
```bash
docker build --tag test_1 .
docker images
```
Run
```bash
docker run test_1
```

"." - Docker image should be built from a Dockerfile
Tag = latest
```bash
docker build --tag test_1:1.0 .
```
Tag = 1.0

Remove image created with no tag
```bash
docker rmi test_1
```
or with tag
```bash
docker rmi test_1:1.0
```
or by image's id
