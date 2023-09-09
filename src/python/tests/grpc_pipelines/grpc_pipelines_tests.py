from ficus.grpc_pipelines.context_values import StringContextValue
from ficus.grpc_pipelines.grpc_pipelines import Pipeline2, ReadLogFromXes2, TracesDiversityDiagram2, \
    DrawPlacementsOfEventByName2, FindSuperMaximalRepeats2, DiscoverActivities2, \
    DiscoverActivitiesInstances2, DrawFullActivitiesDiagram2, DrawPlacementOfEventsByRegex2, \
    DrawShortActivitiesDiagram2, PatternsDiscoveryStrategy, PrintEventLogInfo2, FilterTracesByEventsCount2
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
        DrawPlacementOfEventsByRegex2(regex)
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


def test_draw_full_activities_diagram():
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        TracesDiversityDiagram2(),
        FindSuperMaximalRepeats2(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
        DiscoverActivities2(activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=True),
        DrawFullActivitiesDiagram2()
    )

    log_path = get_example_log_path('exercise4.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_draw_short_activities_diagram():
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        TracesDiversityDiagram2(),
        FindSuperMaximalRepeats2(strategy=PatternsDiscoveryStrategy.FromAllTraces),
        DiscoverActivities2(activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=True),
        DrawShortActivitiesDiagram2()
    )

    log_path = get_example_log_path('exercise4.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_draw_full_activities_diagram_2():
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        TracesDiversityDiagram2(plot_legend=True, title='InitialLog', width_scale=10, height_scale=5),
        FindSuperMaximalRepeats2(strategy=PatternsDiscoveryStrategy.FromAllTraces),
        DiscoverActivities2(activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=True),
        DrawFullActivitiesDiagram2()
    )

    log_path = get_example_log_path('exercise4.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_get_event_log_info():
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        PrintEventLogInfo2(),
    )

    log_path = get_example_log_path('exercise4.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_filter_traces_by_events_count():
    pipeline = Pipeline2(
        ReadLogFromXes2(),
        TracesDiversityDiagram2(),
        FilterTracesByEventsCount2(min_events_in_trace=4),
        TracesDiversityDiagram2(),
    )

    log_path = get_example_log_path('exercise4.xes')
    result = pipeline.execute({
        'path': StringContextValue(log_path)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')
