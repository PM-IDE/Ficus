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
from tests.core.gold_based_test import execute_test_with_gold
from tests.grpc_pipelines.pipeline_parts_for_tests import AssertNamesLogTestPart
from tests.test_data_provider import get_example_log_path, console_app_method2_log_path, petri_net_test_gold_dir


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


def test_class_extractors():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'A.C', 'B.D'],
            ['A.D', 'B.C', 'C', 'A.A', 'B.B'],
        ],
        Pipeline2(
            UseNamesEventLog2(),
            FindSuperMaximalRepeats2(strategy=PatternsDiscoveryStrategy.FromAllTraces, class_extractor='^(.*?)\\.'),
            DiscoverActivities2(activity_level=0),
            DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
            CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
            AssertNamesLogTestPart([
                ['(A.A)::(B.B)', 'C', '(A.A)::(B.B)'],
                ['(A.A)::(B.B)', 'C', '(A.A)::(B.B)']
            ])
        ))


def test_several_levels():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'D', 'A.C', 'B.D', 'C', 'D'],
            ['A.D', 'B.C', 'C', 'D', 'A.A', 'B.B'],
        ],
        Pipeline2(
            UseNamesEventLog2(),
            TracesDiversityDiagram2(plot_legend=True, title='InitialLog'),
            DiscoverActivitiesForSeveralLevels2(event_classes=['^(.*?)\\.', '.*'],
                                                patterns_kind=PatternsKind.MaximalRepeats),
            CreateLogFromActivitiesInstances2(),
            AssertNamesLogTestPart([
                ['(A.D)::(B.C)', '(C)::(D)', '(A.D)::(B.C)', '(C)::(D)'],
                ['(A.D)::(B.C)', '(C)::(D)', '(A.D)::(B.C)']
            ])
        )
    )


def _execute_test_with_names_log(names_log: list[list[str]], pipeline: Pipeline2):
    result = pipeline.execute({
        const_names_event_log: NamesLogContextValue(names_log)
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_discover_activities_from_patterns():
    _execute_test_with_names_log(
        [
            ['A', 'B', 'C', 'A', 'B'],
            ['A', 'B', 'C', 'A', 'B'],
        ],
        Pipeline2(
            UseNamesEventLog2(),
            DiscoverActivitiesFromPatterns2(patterns_kind=PatternsKind.MaximalRepeats),
            DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
            CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
            AssertNamesLogTestPart([['(A)::(B)', 'C', '(A)::(B)'], ['(A)::(B)', 'C', '(A)::(B)']])
        )
    )


def test_discover_activities_until_no_more():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'D', 'A.C', 'B.D', 'C', 'D'],
            ['A.D', 'B.C', 'C', 'D', 'A.A', 'B.B'],
        ],
        Pipeline2(
            UseNamesEventLog2(),
            DiscoverActivitiesUntilNoMore2(event_class=r'^(.*?)(?=\.)'),
            AssertNamesLogTestPart([['(A.A)::(B.B)::(C)::(D)'], ['(A.A)::(B.B)::(C)::(D)']])
        )
    )


def test_filter_events_by_name():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        FilterEventsByName2(event_name='a'),
        AssertNamesLogTestPart([
            ['b', 'd', 'c', 'f'],
            ['c', 'b', 'd', 'f'],
            ['c', 'd', 'b', 'f'],
            ['d', 'e', 'f'],
            ['b', 'c', 'd', 'f'],
            ['e', 'd', 'f']
        ])
    ))


def test_filter_events_by_regex():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        FilterEventsByRegex2(regex='a|b'),
        AssertNamesLogTestPart([
            ['d', 'c', 'f'],
            ['c', 'd', 'f'],
            ['c', 'd', 'f'],
            ['d', 'e', 'f'],
            ['c', 'd', 'f'],
            ['e', 'd', 'f']
        ])
    ))


def test_filter_log_by_variants():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        FilterLogByVariants2(),
        AssertNamesLogTestPart([
            ['a', 'b', 'd', 'c', 'f'],
            ['a', 'c', 'b', 'd', 'f'],
            ['a', 'c', 'd', 'b', 'f'],
            ['a', 'd', 'e', 'f'],
            ['a', 'b', 'c', 'd', 'f'],
            ['a', 'e', 'd', 'f']
        ])
    ))


def test_execute_with_each_activity_log():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        DiscoverActivitiesFromPatterns2(patterns_kind=PatternsKind.MaximalRepeats,
                                        strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
        ExecuteWithEachActivityLog2(ActivitiesLogsSource.TracesActivities, 0, Pipeline2(
            TracesDiversityDiagram2(plot_legend=True)
        ))
    ))


def test_console_app1_log():
    _execute_test_with_context(Pipeline2(
        ReadLogFromXes2(),
        FilterEventsByRegex2('Procfiler.*'),
        FilterEventsByRegex2(r'GC/SampledObjectAllocation_\{System\.Int32\[\]\}'),
        FilterEventsByRegex2(r'.*SuspendEE.*'),
        FilterEventsByRegex2(r'.*RestartEE.*'),
        FilterLogByVariants2(),
        DiscoverActivitiesFromPatterns2(PatternsKind.PrimitiveTandemArrays,
                                        activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown, min_events_in_activity=2),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesForSeveralLevels2(['.*'],
                                            PatternsKind.MaximalRepeats,
                                            min_events_in_activity_count=2,
                                            strategy=PatternsDiscoveryStrategy.FromAllTraces),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.DontInsert),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesUntilNoMore2(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace,
                                       undef_strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        PrintNumberOfUnderlyingEvents2()
    ), {
        'path': StringContextValue(console_app_method2_log_path())
    })


def test_console_app1_two_levels_of_abstraction():
    _execute_test_with_context(Pipeline2(
        ReadLogFromXes2(),
        FilterEventsByRegex2('Procfiler.*'),
        FilterEventsByRegex2(r'GC/SampledObjectAllocation_\{System\.Int32\[\]\}'),
        FilterEventsByRegex2(r'.*SuspendEE.*'),
        FilterEventsByRegex2(r'.*RestartEE.*'),
        FilterLogByVariants2(),
        DiscoverActivitiesFromPatterns2(PatternsKind.PrimitiveTandemArrays,
                                        activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown, min_events_in_activity=2),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesForSeveralLevels2([r'^(.*?)_\{', '.*'],
                                            PatternsKind.MaximalRepeats,
                                            activity_filter_kind=ActivityFilterKind.NoFilter),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.DontInsert),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesUntilNoMore2(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace,
                                       activity_filter_kind=ActivityFilterKind.NoFilter,
                                       undef_strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ExecuteWithEachActivityLog2(ActivitiesLogsSource.Log, activity_level=2, activity_log_pipeline=Pipeline2(
            SubstituteUnderlyingEvents2(),
        ))
    ), {
        'path': StringContextValue(console_app_method2_log_path())
    })


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


def test_discover_petri_net_alpha():
    _execute_discovery_test(
        'test_discover_petri_net_alpha',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'B', 'E', 'D']
        ],
        DiscoverPetriNetAlpha2(),
    )


def _execute_discovery_test(test_name: str, names_log: list[list[str]], discovery_part):
    temp_file = tempfile.NamedTemporaryFile()
    temp_file.close()

    gold_folder = petri_net_test_gold_dir(test_name)
    if not os.path.exists(gold_folder):
        os.makedirs(gold_folder, exist_ok=True)

    _execute_test_with_names_log(names_log, Pipeline2(
        UseNamesEventLog2(),
        AddStartEndArtificialEvents2(),
        discovery_part,
        SerializePetriNetToPNML2(save_path=temp_file.name, use_names_as_ids=True),
        ViewPetriNet2(show_places_names=True, export_path=os.path.join(gold_folder, '.nets', 'petri_net.png'))
    ))

    with open(temp_file.name) as fin:
        execute_test_with_gold(os.path.join(gold_folder, 'petri_net.gold'), fin.read())


def test_discover_petri_net_alpha2():
    _execute_discovery_test(
        'test_discover_petri_net_alpha2',
        [
            ['A', 'C'],
            ['A', 'B', 'C'],
            ['A', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'B', 'C'],
        ],
        DiscoverPetriNetAlpha2(),
    )


def test_discover_petri_net_alpha3():
    _execute_discovery_test(
        'test_discover_petri_net_alpha3',
        [
            ['A', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
        ],
        DiscoverPetriNetAlpha2(),
    )


def test_discover_petri_net_alpha4():
    _execute_discovery_test(
        'test_discover_petri_net_alpha4',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'C', 'B', 'D'],
            ['E', 'F'],
        ],
        DiscoverPetriNetAlpha2()
    )


def test_discover_petri_net_alpha5():
    _execute_discovery_test(
        'test_discover_petri_net_alpha5',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'C', 'B', 'D'],
            ['A', 'E', 'D'],
        ],
        DiscoverPetriNetAlpha2()
    )


def test_discover_petri_net_alpha_plus():
    _execute_discovery_test(
        'test_discover_petri_net_alpha_plus',
        [
            ['A', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
        ],
        DiscoverPetriNetAlphaPlus2()
    )


def test_discover_petri_net_alpha_plus2():
    _execute_discovery_test(
        'test_discover_petri_net_alpha_plus2',
        [
            ['A', 'C'],
            ['A', 'B', 'C'],
            ['A', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'B', 'C'],
        ],
        DiscoverPetriNetAlphaPlus2()
    )


def test_discover_petri_net_alpha_plus4():
    _execute_discovery_test(
        'test_discover_petri_net_alpha_plus4',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'C', 'B', 'D'],
            ['E', 'F'],
        ],
        DiscoverPetriNetAlphaPlus2()
    )


def test_discover_petri_net_alpha_plus5():
    _execute_discovery_test(
        'test_discover_petri_net_alpha_plus5',
        [
            ['A', 'B', 'C', 'D'],
            ['A', 'C', 'B', 'D'],
            ['A', 'E', 'D'],
        ],
        DiscoverPetriNetAlphaPlus2()
    )