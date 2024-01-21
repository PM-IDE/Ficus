import os.path
import tempfile

from ficus.analysis.patterns.patterns_models import UndefinedActivityHandlingStrategy
from ficus.grpc_pipelines.activities_parts import DiscoverActivities2, DiscoverActivitiesInstances2, \
    CreateLogFromActivitiesInstances2, DiscoverActivitiesForSeveralLevels2, DiscoverActivitiesUntilNoMore2, \
    DiscoverActivitiesFromPatterns2, ExecuteWithEachActivityLog2, ClearActivitiesRelatedStuff2, \
    PrintNumberOfUnderlyingEvents2, SubstituteUnderlyingEvents2, ApplyClassExtractor2
from ficus.grpc_pipelines.constants import const_names_event_log
from ficus.grpc_pipelines.context_values import StringContextValue, NamesLogContextValue, ContextValue
from ficus.grpc_pipelines.data_models import PatternsKind, NarrowActivityKind, ActivityFilterKind, ActivitiesLogsSource
from ficus.grpc_pipelines.discovery_parts import DiscoverPetriNetAlpha2, SerializePetriNetToPNML2, ViewPetriNet2, \
    DiscoverPetriNetAlphaPlus2
from ficus.grpc_pipelines.drawing_parts import TracesDiversityDiagram2, DrawPlacementsOfEventByName2, \
    DrawPlacementOfEventsByRegex2
from ficus.grpc_pipelines.filtering_parts import FilterTracesByEventsCount2, FilterEventsByName2, FilterEventsByRegex2, \
    FilterLogByVariants2
from ficus.grpc_pipelines.grpc_pipelines import Pipeline2, PrintEventLogInfo2
from ficus.grpc_pipelines.mutation_parts import AddStartEndArtificialEvents2, AddStartArtificialEvents2, \
    AddEndArtificialEvents2
from ficus.grpc_pipelines.patterns_parts import FindSuperMaximalRepeats2, PatternsDiscoveryStrategy
from ficus.grpc_pipelines.util_parts import UseNamesEventLog2
from ficus.grpc_pipelines.xes_parts import ReadLogFromXes2
from .pipeline_parts_for_tests import AssertNamesLogTestPart
from ..test_data_provider import get_example_log_path, console_app_method2_log_path, petri_net_test_gold_dir


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
    _execute_test_with_exercise_log('exercise1', Pipeline2(
        ReadLogFromXes2(),
        TracesDiversityDiagram2(),
    ))


def _execute_test_with_exercise_log(log_name: str, pipeline: Pipeline2):
    result = pipeline.execute({
        'path': StringContextValue(get_example_log_path(f'{log_name}.xes'))
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def _execute_test_with_context(pipeline: Pipeline2, context: dict[str, ContextValue]):
    result = pipeline.execute(context)

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_pipeline_with_getting_context_value2():
    _execute_test_with_exercise_log('exercise1', Pipeline2(
        ReadLogFromXes2(),
        DrawPlacementsOfEventByName2('A'),
    ))


def test_pipeline_with_getting_context_value3():
    _do_simple_test_with_regex('A|B')


def _do_simple_test_with_regex(regex: str):
    _execute_test_with_exercise_log('exercise1', Pipeline2(
        ReadLogFromXes2(),
        DrawPlacementOfEventsByRegex2(regex)
    ))


def test_pipeline_with_getting_context_value4():
    _do_simple_test_with_regex('A|B|C')


def test_pipeline_with_getting_context_value5():
    _do_simple_test_with_regex('A|B|C|D')


def test_draw_short_activities_diagram():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        FindSuperMaximalRepeats2(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
        DiscoverActivities2(activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        AssertNamesLogTestPart([
            ['(a)::(b)', '(c)::(d)', 'f'],
            ['(a)::(c)', '(b)::(d)', 'f'],
            ['(a)::(c)', '(b)::(d)', 'f'],
            ['a', 'd', '(e)', 'f'],
            ['(a)::(b)', '(c)::(d)', 'f'],
            ['a', '(e)', 'd', 'f']
        ])
    ))


def test_draw_full_activities_diagram_2():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        FindSuperMaximalRepeats2(strategy=PatternsDiscoveryStrategy.FromAllTraces),
        DiscoverActivities2(activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
        CreateLogFromActivitiesInstances2(),
        AssertNamesLogTestPart([[], [], [], [], [], []])
    ))


def test_get_event_log_info():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        PrintEventLogInfo2(),
    ))


def test_filter_traces_by_events_count():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        FilterTracesByEventsCount2(min_events_in_trace=5),
        AssertNamesLogTestPart([
            ['a', 'b', 'd', 'c', 'f'],
            ['a', 'c', 'b', 'd', 'f'],
            ['a', 'c', 'd', 'b', 'f'],
            ['a', 'b', 'c', 'd', 'f']
        ])
    ))


def _execute_test_with_names_log(names_log: list[list[str]], pipeline: Pipeline2):
    result = pipeline.execute({
        const_names_event_log: NamesLogContextValue(names_log)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_apply_class_extractor():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'D', 'A.C', 'B.D', 'C', 'D'],
            ['A.D', 'B.C', 'C', 'D', 'A.A', 'B.B'],
        ],
        Pipeline2(
            UseNamesEventLog2(),
            ApplyClassExtractor2(class_extractor_regex=r'^(.*?)(?=\.)', filter_regex=r'A\..*'),
            AssertNamesLogTestPart(
                [
                    ['A', 'B.B', 'C', 'D', 'A', 'B.D', 'C', 'D'],
                    ['A', 'B.C', 'C', 'D', 'A', 'B.B'],
                ]
            )
        )
    )
