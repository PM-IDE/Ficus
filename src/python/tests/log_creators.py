import os
from typing import Iterable

from .test_data_provider import all_example_logs
from ..ficus.log.functions import *


def create_log_from_filter_out_chaotic_events() -> MyEventLog:
    # https://link.springer.com/article/10.1007/s10844-018-0507-6
    raw_log = []
    for i in range(10):
        raw_log.append('a,b,c,x')
        raw_log.append('a,b,x,c')
        raw_log.append('a,x,b,c')

    return parse_log_from_strings(raw_log)


def create_log_from_filter_out_chaotic_events_with_noise() -> MyEventLog:
    # https://link.springer.com/article/10.1007/s10844-018-0507-6
    raw_log = []
    for i in range(10):
        raw_log.append('d,v,d,d,a,d,b,c,x,d,d,d,d,d')
        raw_log.append('a,d,d,d,d,b,d,x,c,d')
        raw_log.append('d,d,d,v,d,a,x,b,c,d')

    return parse_log_from_strings(raw_log)


def enumerate_example_logs() -> Iterable[MyEventLog]:
    for log_file in all_example_logs():
        yield read_log_from_xes(log_file)


def create_tandem_array_log_taxonomy_of_patterns() -> MyEventLog:
    raw_log = insert_separator(create_tandem_array_raw_string_taxonomy_of_patterns())
    return parse_log_from_string(raw_log)


def create_tandem_array_raw_string_taxonomy_of_patterns() -> str:
    return 'gdabcabcabcabcafica'


def insert_separator(single_chars_events: str, sep: str = default_separator) -> str:
    return sep.join(list(single_chars_events))


def create_list_of_raw_events_for_maximal_repeat() -> list[str]:
    raw_log_strings = ['aabcdbbcda', 'dabcdabcbb', 'bbbcdbbbccaa', 'aaadabbccc', 'aaacdcdcbedbccbadbdebdc']
    return raw_log_strings