#!/bin/bash
##ARGUMENTS: 1. Docker target
set -e # fail on any error
set -u # treat unset variables as error

if [ "$CI_COMMIT_REF_NAME" == "master" ];
	then export DOCKER_BUILD_TAG="${SCHEDULE_TAG:-latest}";
	else export DOCKER_BUILD_TAG=$CI_COMMIT_REF_NAME;
fi
docker login -u $Docker_Hub_User_Susy -p $Docker_Hub_Pass_Susy

echo "__________Docker TAG__________"
echo $DOCKER_BUILD_TAG

echo "__________Docker target__________"
export DOCKER_TARGET=$1
echo $DOCKER_TARGET

echo "__________Docker build and push__________"
docker build --build-arg TARGET=$DOCKER_TARGET --no-cache=true --tag susy/$DOCKER_TARGET:$DOCKER_BUILD_TAG -f scripts/docker/hub/Dockerfile .
docker push susy/$DOCKER_TARGET:$DOCKER_BUILD_TAG
docker logout
