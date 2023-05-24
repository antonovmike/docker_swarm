# docker_swarm

The algorythm creates "config.yml" file

(Fix description)

STEP 1

Choose the image and run it
```bash
sudo docker image ls
sudo docker run -it --rm IMAGE_NAME /bin/sh
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

