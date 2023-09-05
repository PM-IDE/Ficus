from ficus.grpc_pipelines.context_values import StringContextValue
from ficus.grpc_pipelines.grpc_pipelines import Pipeline2, ReadLogFromXes2, TracesDiversityDiagram2, \
    DrawPlacementsOfEventByName2, DrawPlacementOfEventsByRegex
from tests.test_data_provider import get_example_log_path


def test_simple_pipeline():
    pipeline = Pipeline2(
        ReadLogFromXes2()
    )

    result = pipeline.execute({
        'path': StringContextValue('asdasdasdasdas')
    })

    assert not result.finalResult.HasField('success')
    assert result.finalResult.error is not None
    assert result.finalResult.error == 'Failed to read event log from asdasdasdasdas'


def test_pipeline_with_getting_context_value():
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        TracesDiversityDiagram2(),
    )

    log_path = get_example_log_path('exercise1.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_pipeline_with_getting_context_value2():
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        DrawPlacementsOfEventByName2('A'),
    )

    log_path = get_example_log_path('exercise1.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_pipeline_with_getting_context_value3():
    _do_simple_test_with_regex('A|B')


def _do_simple_test_with_regex(regex: str):
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        DrawPlacementOfEventsByRegex(regex)
    )

    log_path = get_example_log_path('exercise1.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_pipeline_with_getting_context_value4():
    _do_simple_test_with_regex('A|B|C')


def test_pipeline_with_getting_context_value5():
    _do_simple_test_with_regex('A|B|C|D')
