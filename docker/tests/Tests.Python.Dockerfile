FROM ficus_base:latest

WORKDIR /pmide/ficus/src/python

ENV FICUS_BACKEND_ADDR=http://ficus_backend:8080

ENTRYPOINT $python -m pytest .