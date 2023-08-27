from ...ficus.grpc_pipelines.context_values import *


def test_string_context_value():
    value = "asdasdasdsdasda"
    context_value = StringContextValue(value).to_grpc_context_value()
    assert value == context_value


def test_uint32_context_value():
    value = 123123
    context_value = Uint32ContextValue(value).to_grpc_context_value()
    assert value == context_value


def test_bool_context_value():
    value = True
    context_value = BoolContextValue(value).to_grpc_context_value()
    assert value == context_value


def test_names_log_context_value():
    raw_log = [["asdasdasdads", "asdasdasd"]]
    context_value = NamesLogContextValue(raw_log).to_grpc_context_value()
    assert raw_log[0][0] == context_value.log.traces[0].events[0]
    assert raw_log[0][1] == context_value.log.traces[0].events[1]


def test_hashes_log_context_value():
    raw_log = [[12312312, 323123123]]
    context_value = HashesLogContextValue(raw_log).to_grpc_context_value()
    assert raw_log[0][0] == context_value.log.traces[0].events[0]
    assert raw_log[0][1] == context_value.log.traces[0].events[1]
