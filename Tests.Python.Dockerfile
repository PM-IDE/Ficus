FROM ficus_tests_base:latest

WORKDIR /pmide/ficus/src/python
EXPOSE 5000

RUN $python -m pip install pytest

ENTRYPOINT $python -m pytest .