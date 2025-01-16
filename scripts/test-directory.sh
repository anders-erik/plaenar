#!/bin/bash

# This script will generates mock files/dirs for testing

ROOT_DIR=~/ae/usb/
PROJECTS_DIR=$ROOT_DIR/projects

mkdir -p $ROOT_DIR
mkdir -p $PROJECTS_DIR

cd $ROOT_DIR
mkdir -p anki planner play proto-projects

cd $PROJECTS_DIR
mkdir -p project_1
mkdir -p project_1/assets
    touch    project_1/assets/fake.jpg

mkdir -p project_1/module_1
    touch    project_1/module_1/tasks.yaml

mkdir -p project_1/module_2
    touch    project_1/module_2/tasks.yaml
    
mkdir -p project_1/module_3
    touch    project_1/module_3/tasks.yaml





mkdir -p project_2
mkdir -p project_2/assets
    touch    project_2/assets/interesting_dummy_image.jpg





mkdir -p project_3
touch    project_3/tasks.yaml
mkdir -p project_3/module_1
    touch    project_3/module_1/tasks.yaml
mkdir -p project_3/module_2
    touch    project_3/module_2/tasks.yaml
