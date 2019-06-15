#!/usr/bin/env sh

# The image name
SUSY_IMAGE_REPO=${SUSY_IMAGE_REPO:-susy/susy}
# The tag to be used for builder image
SUSY_BUILDER_IMAGE_TAG=${SUSY_BUILDER_IMAGE_TAG:-build}
# The tag to be used for runner image
SUSY_RUNNER_IMAGE_TAG=${SUSY_RUNNER_IMAGE_TAG:-latest}

echo Building $SUSY_IMAGE_REPO:$SUSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")
docker build --no-cache -t $SUSY_IMAGE_REPO:$SUSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H") . -f docker/centos/Dockerfile.build

echo Creating $SUSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H"), extracting binary
docker create --name extract $SUSY_IMAGE_REPO:$SUSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H") 
mkdir docker/centos/susy
docker cp extract:/build/susy-sophon/target/release/susy docker/centos/susy

echo Building $SUSY_IMAGE_REPO:$SUSY_RUNNER_IMAGE_TAG
docker build --no-cache -t $SUSY_IMAGE_REPO:$SUSY_RUNNER_IMAGE_TAG docker/centos/ -f docker/centos/Dockerfile

echo Cleaning up ...
rm -rf docker/centos/susy
docker rm -f extract
docker rmi -f $SUSY_IMAGE_REPO:$SUSY_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")

echo Echoing Susy version:
docker run $SUSY_IMAGE_REPO:$SUSY_RUNNER_IMAGE_TAG --version

echo Done.
