import os.path
from os.path import *
from typing import Iterable


def data_dir() -> str:
    return os.path.join(os.path.curdir, 'test_data')


def sources_dir() -> str:
    return os.path.join(data_dir(), 'source')


def gold_dir() -> str:
    return os.path.join(data_dir(), 'gold')


def example_logs_dir() -> str:
    return os.path.join(sources_dir(), 'example_logs')


def get_example_log_path(log_name: str) -> str:
    return os.path.join(example_logs_dir(), log_name)


def all_example_logs() -> list[str]:
    return [join(example_logs_dir(), f) for f in os.listdir(example_logs_dir()) if
            isfile(join(example_logs_dir(), f)) and f.endswith('.xes')]


def repair_logs_dir() -> str:
    return os.path.join(sources_dir(), 'repair_logs')


def get_repair_example_path() -> str:
    return os.path.join(repair_logs_dir(), 'repairExample.xes')


def console_app_method2_log_path() -> str:
    return os.path.join(os.path.curdir, 'test_data', 'source', 'solutions_logs', 'consoleapp1Program.Method2.xes')


def all_test_split_traces() -> Iterable[str]:
    initial_dir = os.path.join(os.path.curdir, 'test_data', 'source', 'test_split_traces')
    for solution_dir in os.listdir(initial_dir):
        for log_path in os.listdir(os.path.join(initial_dir, solution_dir)):
            yield os.path.join(initial_dir, solution_dir, log_path)