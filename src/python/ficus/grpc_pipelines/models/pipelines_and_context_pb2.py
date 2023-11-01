# -*- coding: utf-8 -*-

# Generated by the protocol buffer compiler.  DO NOT EDIT!

# source: pipelines_and_context.proto

"""Generated protocol buffer code."""

from google.protobuf import descriptor as _descriptor

from google.protobuf import descriptor_pool as _descriptor_pool

from google.protobuf import symbol_database as _symbol_database

from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)



_sym_db = _symbol_database.Default()





import ficus.grpc_pipelines.models.pm_models_pb2 as pm__models__pb2

import ficus.grpc_pipelines.models.util_pb2 as util__pb2





DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1bpipelines_and_context.proto\x12\x05\x66icus\x1a\x0fpm_models.proto\x1a\nutil.proto\"\x1e\n\x0eGrpcContextKey\x12\x0c\n\x04name\x18\x01 \x01(\t\"\xdd\x05\n\x10GrpcContextValue\x12\x10\n\x06string\x18\x01 \x01(\tH\x00\x12;\n\nhashes_log\x18\x02 \x01(\x0b\x32%.ficus.GrpcHashesEventLogContextValueH\x00\x12\x39\n\tnames_log\x18\x03 \x01(\x0b\x32$.ficus.GrpcNamesEventLogContextValueH\x00\x12\x10\n\x06uint32\x18\x04 \x01(\rH\x00\x12J\n\x11traces_sub_arrays\x18\x05 \x01(\x0b\x32-.ficus.GrpcEventLogTraceSubArraysContextValueH\x00\x12P\n\x16trace_index_sub_arrays\x18\x06 \x01(\x0b\x32..ficus.GrpcSubArraysWithTraceIndexContextValueH\x00\x12\x0e\n\x04\x62ool\x18\x07 \x01(\x08H\x00\x12=\n\rxes_event_log\x18\x08 \x01(\x0b\x32$.ficus.GrpcNamesEventLogContextValueH\x00\x12/\n\ncolors_log\x18\t \x01(\x0b\x32\x19.ficus.GrpcColorsEventLogH\x00\x12\x1f\n\x04\x65num\x18\n \x01(\x0b\x32\x0f.ficus.GrpcEnumH\x00\x12\x31\n\x0e\x65vent_log_info\x18\x0b \x01(\x0b\x32\x17.ficus.GrpcEventLogInfoH\x00\x12%\n\x07strings\x18\x0c \x01(\x0b\x32\x12.ficus.GrpcStringsH\x00\x12\'\n\x08pipeline\x18\r \x01(\x0b\x32\x13.ficus.GrpcPipelineH\x00\x12\'\n\x08petriNet\x18\x0e \x01(\x0b\x32\x13.ficus.GrpcPetriNetH\x00\x12!\n\x05graph\x18\x0f \x01(\x0b\x32\x10.ficus.GrpcGraphH\x00\x12\x0f\n\x05\x66loat\x18\x10 \x01(\x02H\x00\x42\x0e\n\x0c\x63ontextValue\"a\n\x13GrpcContextKeyValue\x12\"\n\x03key\x18\x01 \x01(\x0b\x32\x15.ficus.GrpcContextKey\x12&\n\x05value\x18\x02 \x01(\x0b\x32\x17.ficus.GrpcContextValue\"H\n\x1eGrpcHashesEventLogContextValue\x12&\n\x03log\x18\x01 \x01(\x0b\x32\x19.ficus.GrpcHashesEventLog\"F\n\x1dGrpcNamesEventLogContextValue\x12%\n\x03log\x18\x01 \x01(\x0b\x32\x18.ficus.GrpcNamesEventLog\"^\n&GrpcEventLogTraceSubArraysContextValue\x12\x34\n\x11traces_sub_arrays\x18\x01 \x03(\x0b\x32\x19.ficus.GrpcTraceSubArrays\"/\n\x11GrpcTraceSubArray\x12\r\n\x05start\x18\x01 \x01(\r\x12\x0b\n\x03\x65nd\x18\x02 \x01(\r\"B\n\x12GrpcTraceSubArrays\x12,\n\nsub_arrays\x18\x01 \x03(\x0b\x32\x18.ficus.GrpcTraceSubArray\"^\n\x1aGrpcSubArrayWithTraceIndex\x12+\n\tsub_array\x18\x01 \x01(\x0b\x32\x18.ficus.GrpcTraceSubArray\x12\x13\n\x0btrace_index\x18\x02 \x01(\r\"`\n\'GrpcSubArraysWithTraceIndexContextValue\x12\x35\n\nsub_arrays\x18\x01 \x03(\x0b\x32!.ficus.GrpcSubArrayWithTraceIndex\"<\n\x12GrpcColorsEventLog\x12&\n\x06traces\x18\x01 \x03(\x0b\x32\x16.ficus.GrpcColorsTrace\"D\n\x0fGrpcColorsTrace\x12\x31\n\x0c\x65vent_colors\x18\x02 \x03(\x0b\x32\x1b.ficus.GrpcColoredRectangle\"j\n\x14GrpcColoredRectangle\x12\x1f\n\x05\x63olor\x18\x01 \x01(\x0b\x32\x10.ficus.GrpcColor\x12\x13\n\x0bstart_index\x18\x02 \x01(\r\x12\x0e\n\x06length\x18\x03 \x01(\r\x12\x0c\n\x04name\x18\x04 \x01(\t\"+\n\x08GrpcEnum\x12\x10\n\x08\x65numType\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t\"[\n\x10GrpcEventLogInfo\x12\x14\n\x0c\x65vents_count\x18\x01 \x01(\r\x12\x14\n\x0ctraces_count\x18\x02 \x01(\r\x12\x1b\n\x13\x65vent_classes_count\x18\x03 \x01(\r\"\x1e\n\x0bGrpcStrings\x12\x0f\n\x07strings\x18\x01 \x03(\t\":\n\x0cGrpcPipeline\x12*\n\x05parts\x18\x01 \x03(\x0b\x32\x1b.ficus.GrpcPipelinePartBase\"\xab\x02\n\x14GrpcPipelinePartBase\x12.\n\x0b\x64\x65\x66\x61ultPart\x18\x01 \x01(\x0b\x32\x17.ficus.GrpcPipelinePartH\x00\x12\x37\n\x0cparallelPart\x18\x02 \x01(\x0b\x32\x1f.ficus.GrpcParallelPipelinePartH\x00\x12O\n\x18simpleContextRequestPart\x18\x03 \x01(\x0b\x32+.ficus.GrpcSimpleContextRequestPipelinePartH\x00\x12Q\n\x19\x63omplexContextRequestPart\x18\x04 \x01(\x0b\x32,.ficus.GrpcComplexContextRequestPipelinePartH\x00\x42\x06\n\x04part\"]\n\x10GrpcPipelinePart\x12\x0c\n\x04name\x18\x01 \x01(\t\x12;\n\rconfiguration\x18\x02 \x01(\x0b\x32$.ficus.GrpcPipelinePartConfiguration\"\\\n\x1dGrpcPipelinePartConfiguration\x12;\n\x17\x63onfigurationParameters\x18\x01 \x03(\x0b\x32\x1a.ficus.GrpcContextKeyValue\"N\n\x18GrpcParallelPipelinePart\x12\x32\n\rpipelineParts\x18\x01 \x03(\x0b\x32\x1b.ficus.GrpcPipelinePartBase\"N\n\x19GrpcParallelPipelineParts\x12\x31\n\x08pipeline\x18\x01 \x03(\x0b\x32\x1f.ficus.GrpcParallelPipelinePart\"u\n$GrpcSimpleContextRequestPipelinePart\x12\"\n\x03key\x18\x01 \x01(\x0b\x32\x15.ficus.GrpcContextKey\x12)\n\x10\x66rontendPartUuid\x18\x02 \x01(\x0b\x32\x0f.ficus.GrpcUuid\"\xab\x01\n%GrpcComplexContextRequestPipelinePart\x12\"\n\x03key\x18\x01 \x01(\x0b\x32\x15.ficus.GrpcContextKey\x12\x33\n\x12\x62\x65\x66orePipelinePart\x18\x02 \x01(\x0b\x32\x17.ficus.GrpcPipelinePart\x12)\n\x10\x66rontendPartUuid\x18\x03 \x01(\x0b\x32\x0f.ficus.GrpcUuid\"U\n\tGrpcGraph\x12#\n\x05nodes\x18\x01 \x03(\x0b\x32\x14.ficus.GrpcGraphNode\x12#\n\x05\x65\x64ges\x18\x02 \x03(\x0b\x32\x14.ficus.GrpcGraphEdge\")\n\rGrpcGraphNode\x12\n\n\x02id\x18\x01 \x01(\x04\x12\x0c\n\x04\x64\x61ta\x18\x02 \x01(\t\"A\n\rGrpcGraphEdge\x12\x11\n\tfrom_node\x18\x01 \x01(\x04\x12\x0f\n\x07to_node\x18\x02 \x01(\x04\x12\x0c\n\x04\x64\x61ta\x18\x03 \x01(\tb\x06proto3')



_globals = globals()

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)

_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'pipelines_and_context_pb2', _globals)

if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None

  _globals['_GRPCCONTEXTKEY']._serialized_start=67

  _globals['_GRPCCONTEXTKEY']._serialized_end=97

  _globals['_GRPCCONTEXTVALUE']._serialized_start=100

  _globals['_GRPCCONTEXTVALUE']._serialized_end=833

  _globals['_GRPCCONTEXTKEYVALUE']._serialized_start=835

  _globals['_GRPCCONTEXTKEYVALUE']._serialized_end=932

  _globals['_GRPCHASHESEVENTLOGCONTEXTVALUE']._serialized_start=934

  _globals['_GRPCHASHESEVENTLOGCONTEXTVALUE']._serialized_end=1006

  _globals['_GRPCNAMESEVENTLOGCONTEXTVALUE']._serialized_start=1008

  _globals['_GRPCNAMESEVENTLOGCONTEXTVALUE']._serialized_end=1078

  _globals['_GRPCEVENTLOGTRACESUBARRAYSCONTEXTVALUE']._serialized_start=1080

  _globals['_GRPCEVENTLOGTRACESUBARRAYSCONTEXTVALUE']._serialized_end=1174

  _globals['_GRPCTRACESUBARRAY']._serialized_start=1176

  _globals['_GRPCTRACESUBARRAY']._serialized_end=1223

  _globals['_GRPCTRACESUBARRAYS']._serialized_start=1225

  _globals['_GRPCTRACESUBARRAYS']._serialized_end=1291

  _globals['_GRPCSUBARRAYWITHTRACEINDEX']._serialized_start=1293

  _globals['_GRPCSUBARRAYWITHTRACEINDEX']._serialized_end=1387

  _globals['_GRPCSUBARRAYSWITHTRACEINDEXCONTEXTVALUE']._serialized_start=1389

  _globals['_GRPCSUBARRAYSWITHTRACEINDEXCONTEXTVALUE']._serialized_end=1485

  _globals['_GRPCCOLORSEVENTLOG']._serialized_start=1487

  _globals['_GRPCCOLORSEVENTLOG']._serialized_end=1547

  _globals['_GRPCCOLORSTRACE']._serialized_start=1549

  _globals['_GRPCCOLORSTRACE']._serialized_end=1617

  _globals['_GRPCCOLOREDRECTANGLE']._serialized_start=1619

  _globals['_GRPCCOLOREDRECTANGLE']._serialized_end=1725

  _globals['_GRPCENUM']._serialized_start=1727

  _globals['_GRPCENUM']._serialized_end=1770

  _globals['_GRPCEVENTLOGINFO']._serialized_start=1772

  _globals['_GRPCEVENTLOGINFO']._serialized_end=1863

  _globals['_GRPCSTRINGS']._serialized_start=1865

  _globals['_GRPCSTRINGS']._serialized_end=1895

  _globals['_GRPCPIPELINE']._serialized_start=1897

  _globals['_GRPCPIPELINE']._serialized_end=1955

  _globals['_GRPCPIPELINEPARTBASE']._serialized_start=1958

  _globals['_GRPCPIPELINEPARTBASE']._serialized_end=2257

  _globals['_GRPCPIPELINEPART']._serialized_start=2259

  _globals['_GRPCPIPELINEPART']._serialized_end=2352

  _globals['_GRPCPIPELINEPARTCONFIGURATION']._serialized_start=2354

  _globals['_GRPCPIPELINEPARTCONFIGURATION']._serialized_end=2446

  _globals['_GRPCPARALLELPIPELINEPART']._serialized_start=2448

  _globals['_GRPCPARALLELPIPELINEPART']._serialized_end=2526

  _globals['_GRPCPARALLELPIPELINEPARTS']._serialized_start=2528

  _globals['_GRPCPARALLELPIPELINEPARTS']._serialized_end=2606

  _globals['_GRPCSIMPLECONTEXTREQUESTPIPELINEPART']._serialized_start=2608

  _globals['_GRPCSIMPLECONTEXTREQUESTPIPELINEPART']._serialized_end=2725

  _globals['_GRPCCOMPLEXCONTEXTREQUESTPIPELINEPART']._serialized_start=2728

  _globals['_GRPCCOMPLEXCONTEXTREQUESTPIPELINEPART']._serialized_end=2899

  _globals['_GRPCGRAPH']._serialized_start=2901

  _globals['_GRPCGRAPH']._serialized_end=2986

  _globals['_GRPCGRAPHNODE']._serialized_start=2988

  _globals['_GRPCGRAPHNODE']._serialized_end=3029

  _globals['_GRPCGRAPHEDGE']._serialized_start=3031

  _globals['_GRPCGRAPHEDGE']._serialized_end=3096

# @@protoc_insertion_point(module_scope)
