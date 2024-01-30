FROM ficus_tests_base:latest

WORKDIR /pmide/ficus/src/python
EXPOSE 5000

ENTRYPOINT $python -m pytest .