FROM ficus_base:latest

WORKDIR /pmide/ficus/src/python

ENTRYPOINT $python -m pytest .