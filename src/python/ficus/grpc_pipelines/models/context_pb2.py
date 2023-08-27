# -*- coding: utf-8 -*-

# Generated by the protocol buffer compiler.  DO NOT EDIT!

# source: context.proto

"""Generated protocol buffer code."""

from google.protobuf import descriptor as _descriptor

from google.protobuf import descriptor_pool as _descriptor_pool

from google.protobuf import message as _message

from google.protobuf import reflection as _reflection

from google.protobuf import symbol_database as _symbol_database

# @@protoc_insertion_point(imports)



_sym_db = _symbol_database.Default()





import ficus.grpc_pipelines.models.pm_models_pb2 as pm__models__pb2





DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\rcontext.proto\x12\x05\x66icus\x1a\x0fpm_models.proto\"\x1e\n\x0eGrpcContextKey\x12\x0c\n\x04name\x18\x01 \x01(\t\"\xab\x03\n\x10GrpcContextValue\x12\x10\n\x06string\x18\x01 \x01(\tH\x00\x12;\n\nhashes_log\x18\x02 \x01(\x0b\x32%.ficus.GrpcHashesEventLogContextValueH\x00\x12\x39\n\tnames_log\x18\x03 \x01(\x0b\x32$.ficus.GrpcNamesEventLogContextValueH\x00\x12\x10\n\x06uint32\x18\x04 \x01(\rH\x00\x12J\n\x11traces_sub_arrays\x18\x05 \x01(\x0b\x32-.ficus.GrpcEventLogTraceSubArraysContextValueH\x00\x12P\n\x16trace_index_sub_arrays\x18\x06 \x01(\x0b\x32..ficus.GrpcSubArraysWithTraceIndexContextValueH\x00\x12\x0e\n\x04\x62ool\x18\x07 \x01(\x08H\x00\x12=\n\rxes_event_log\x18\x08 \x01(\x0b\x32$.ficus.GrpcNamesEventLogContextValueH\x00\x42\x0e\n\x0c\x63ontextValue\"a\n\x13GrpcContextKeyValue\x12\"\n\x03key\x18\x01 \x01(\x0b\x32\x15.ficus.GrpcContextKey\x12&\n\x05value\x18\x02 \x01(\x0b\x32\x17.ficus.GrpcContextValue\"H\n\x1eGrpcHashesEventLogContextValue\x12&\n\x03log\x18\x01 \x01(\x0b\x32\x19.ficus.GrpcHashesEventLog\"F\n\x1dGrpcNamesEventLogContextValue\x12%\n\x03log\x18\x01 \x01(\x0b\x32\x18.ficus.GrpcNamesEventLog\"^\n&GrpcEventLogTraceSubArraysContextValue\x12\x34\n\x11traces_sub_arrays\x18\x01 \x03(\x0b\x32\x19.ficus.GrpcTraceSubArrays\"/\n\x11GrpcTraceSubArray\x12\r\n\x05start\x18\x01 \x01(\r\x12\x0b\n\x03\x65nd\x18\x02 \x01(\r\"B\n\x12GrpcTraceSubArrays\x12,\n\nsub_arrays\x18\x01 \x03(\x0b\x32\x18.ficus.GrpcTraceSubArray\"^\n\x1aGrpcSubArrayWithTraceIndex\x12+\n\tsub_array\x18\x01 \x01(\x0b\x32\x18.ficus.GrpcTraceSubArray\x12\x13\n\x0btrace_index\x18\x02 \x01(\r\"`\n\'GrpcSubArraysWithTraceIndexContextValue\x12\x35\n\nsub_arrays\x18\x01 \x03(\x0b\x32!.ficus.GrpcSubArrayWithTraceIndexb\x06proto3')







_GRPCCONTEXTKEY = DESCRIPTOR.message_types_by_name['GrpcContextKey']

_GRPCCONTEXTVALUE = DESCRIPTOR.message_types_by_name['GrpcContextValue']

_GRPCCONTEXTKEYVALUE = DESCRIPTOR.message_types_by_name['GrpcContextKeyValue']

_GRPCHASHESEVENTLOGCONTEXTVALUE = DESCRIPTOR.message_types_by_name['GrpcHashesEventLogContextValue']

_GRPCNAMESEVENTLOGCONTEXTVALUE = DESCRIPTOR.message_types_by_name['GrpcNamesEventLogContextValue']

_GRPCEVENTLOGTRACESUBARRAYSCONTEXTVALUE = DESCRIPTOR.message_types_by_name['GrpcEventLogTraceSubArraysContextValue']

_GRPCTRACESUBARRAY = DESCRIPTOR.message_types_by_name['GrpcTraceSubArray']

_GRPCTRACESUBARRAYS = DESCRIPTOR.message_types_by_name['GrpcTraceSubArrays']

_GRPCSUBARRAYWITHTRACEINDEX = DESCRIPTOR.message_types_by_name['GrpcSubArrayWithTraceIndex']

_GRPCSUBARRAYSWITHTRACEINDEXCONTEXTVALUE = DESCRIPTOR.message_types_by_name['GrpcSubArraysWithTraceIndexContextValue']

GrpcContextKey = _reflection.GeneratedProtocolMessageType('GrpcContextKey', (_message.Message,), {

  'DESCRIPTOR' : _GRPCCONTEXTKEY,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcContextKey)

  })

_sym_db.RegisterMessage(GrpcContextKey)



GrpcContextValue = _reflection.GeneratedProtocolMessageType('GrpcContextValue', (_message.Message,), {

  'DESCRIPTOR' : _GRPCCONTEXTVALUE,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcContextValue)

  })

_sym_db.RegisterMessage(GrpcContextValue)



GrpcContextKeyValue = _reflection.GeneratedProtocolMessageType('GrpcContextKeyValue', (_message.Message,), {

  'DESCRIPTOR' : _GRPCCONTEXTKEYVALUE,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcContextKeyValue)

  })

_sym_db.RegisterMessage(GrpcContextKeyValue)



GrpcHashesEventLogContextValue = _reflection.GeneratedProtocolMessageType('GrpcHashesEventLogContextValue', (_message.Message,), {

  'DESCRIPTOR' : _GRPCHASHESEVENTLOGCONTEXTVALUE,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcHashesEventLogContextValue)

  })

_sym_db.RegisterMessage(GrpcHashesEventLogContextValue)



GrpcNamesEventLogContextValue = _reflection.GeneratedProtocolMessageType('GrpcNamesEventLogContextValue', (_message.Message,), {

  'DESCRIPTOR' : _GRPCNAMESEVENTLOGCONTEXTVALUE,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcNamesEventLogContextValue)

  })

_sym_db.RegisterMessage(GrpcNamesEventLogContextValue)



GrpcEventLogTraceSubArraysContextValue = _reflection.GeneratedProtocolMessageType('GrpcEventLogTraceSubArraysContextValue', (_message.Message,), {

  'DESCRIPTOR' : _GRPCEVENTLOGTRACESUBARRAYSCONTEXTVALUE,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcEventLogTraceSubArraysContextValue)

  })

_sym_db.RegisterMessage(GrpcEventLogTraceSubArraysContextValue)



GrpcTraceSubArray = _reflection.GeneratedProtocolMessageType('GrpcTraceSubArray', (_message.Message,), {

  'DESCRIPTOR' : _GRPCTRACESUBARRAY,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcTraceSubArray)

  })

_sym_db.RegisterMessage(GrpcTraceSubArray)



GrpcTraceSubArrays = _reflection.GeneratedProtocolMessageType('GrpcTraceSubArrays', (_message.Message,), {

  'DESCRIPTOR' : _GRPCTRACESUBARRAYS,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcTraceSubArrays)

  })

_sym_db.RegisterMessage(GrpcTraceSubArrays)



GrpcSubArrayWithTraceIndex = _reflection.GeneratedProtocolMessageType('GrpcSubArrayWithTraceIndex', (_message.Message,), {

  'DESCRIPTOR' : _GRPCSUBARRAYWITHTRACEINDEX,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcSubArrayWithTraceIndex)

  })

_sym_db.RegisterMessage(GrpcSubArrayWithTraceIndex)



GrpcSubArraysWithTraceIndexContextValue = _reflection.GeneratedProtocolMessageType('GrpcSubArraysWithTraceIndexContextValue', (_message.Message,), {

  'DESCRIPTOR' : _GRPCSUBARRAYSWITHTRACEINDEXCONTEXTVALUE,

  '__module__' : 'context_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcSubArraysWithTraceIndexContextValue)

  })

_sym_db.RegisterMessage(GrpcSubArraysWithTraceIndexContextValue)



if _descriptor._USE_C_DESCRIPTORS == False:



  DESCRIPTOR._options = None

  _GRPCCONTEXTKEY._serialized_start=41

  _GRPCCONTEXTKEY._serialized_end=71

  _GRPCCONTEXTVALUE._serialized_start=74

  _GRPCCONTEXTVALUE._serialized_end=501

  _GRPCCONTEXTKEYVALUE._serialized_start=503

  _GRPCCONTEXTKEYVALUE._serialized_end=600

  _GRPCHASHESEVENTLOGCONTEXTVALUE._serialized_start=602

  _GRPCHASHESEVENTLOGCONTEXTVALUE._serialized_end=674

  _GRPCNAMESEVENTLOGCONTEXTVALUE._serialized_start=676

  _GRPCNAMESEVENTLOGCONTEXTVALUE._serialized_end=746

  _GRPCEVENTLOGTRACESUBARRAYSCONTEXTVALUE._serialized_start=748

  _GRPCEVENTLOGTRACESUBARRAYSCONTEXTVALUE._serialized_end=842

  _GRPCTRACESUBARRAY._serialized_start=844

  _GRPCTRACESUBARRAY._serialized_end=891

  _GRPCTRACESUBARRAYS._serialized_start=893

  _GRPCTRACESUBARRAYS._serialized_end=959

  _GRPCSUBARRAYWITHTRACEINDEX._serialized_start=961

  _GRPCSUBARRAYWITHTRACEINDEX._serialized_end=1055

  _GRPCSUBARRAYSWITHTRACEINDEXCONTEXTVALUE._serialized_start=1057

  _GRPCSUBARRAYSWITHTRACEINDEXCONTEXTVALUE._serialized_end=1153

# @@protoc_insertion_point(module_scope)