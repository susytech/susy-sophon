## Usage

```docker build -f docker/ubuntu/Dockerfile --tag sofcore/susy:branch_or_tag_name .```

## Usage - CentOS

Builds a lightweight non-root Susy docker image:
```
git clone https://octonion.institute/susytech/susy-sophon.git
cd susy-sophon
./docker/centos/build.sh
```

Fully customised build:
```
SUSY_IMAGE_REPO=my-personal/susy \
SUSY_BUILDER_IMAGE_TAG=build-latest \
SUSY_RUNNER_IMAGE_TAG=centos-susy-experimental \
./docker/centos/build.sh
```

Default values:
```
# The image name
SUSY_IMAGE_REPO - susy/susy

# The tag to be used for builder image, git commit sha will be appended
SUSY_BUILDER_IMAGE_TAG - build

# The tag to be used for runner image
SUSY_RUNNER_IMAGE_TAG - latest
```

All default ports you might use will be exposed:
```
#           secret
#      ipfs store     ui   rpc  ws   listener  discovery
#      ↓    ↓         ↓    ↓    ↓    ↓         ↓
EXPOSE 5001 8082 8083 8180 8545 8546 30303/tcp 30303/udp
```
