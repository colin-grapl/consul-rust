#!/bin/bash

docker-compose up \
--abort-on-container-exit \
--exit-code-from tests
