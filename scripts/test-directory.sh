#!/bin/bash

# This script will generates mock files/dirs for testing

ROOT_DIR=/tmp/plaenar-test/
PROJECTS_DIR=$ROOT_DIR/projects

mkdir -p $ROOT_DIR
mkdir -p $PROJECTS_DIR

cd $ROOT_DIR
mkdir -p anki planner play proto-projects

cd $PROJECTS_DIR

#  PROJECT 1
mkdir -p project_1
touch    project_1/tasks.yaml
# Add contents to task.yaml
cat > project_1/tasks.yaml <<'endmsg'
%YAML 1.2
---
project:
  - name: project_1

tasks:
  - name:       add a task to test directory
    id:         b4da39df
    descript:   Need to add at least one task for testing purposes. 
    time_esti:	0.1
    time_spent: 0.1
    completed: false
    note: |
        sample note

...
endmsg

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
mkdir -p project_3/module_1
    touch    project_3/module_1/tasks.yaml
mkdir -p project_3/module_2
    touch    project_3/module_2/tasks.yaml
touch    project_3/tasks.yaml
cat > project_3/tasks.yaml <<'endmsg'
%YAML 1.2
---
project:
  - name: project_1

tasks:
  - name:       add a task to test directory
    id:         b4da39df
    descript:   Need to add at least one task for testing purposes. 
    time_esti:	0.1
    time_spent: 0.1
    completed: false
    note: |
        sample note

...
endmsg
