# -*- coding: utf-8 -*-

# Generated by the protocol buffer compiler.  DO NOT EDIT!

# source: pm_models.proto

"""Generated protocol buffer code."""

from google.protobuf import descriptor as _descriptor

from google.protobuf import descriptor_pool as _descriptor_pool

from google.protobuf import message as _message

from google.protobuf import reflection as _reflection

from google.protobuf import symbol_database as _symbol_database

# @@protoc_insertion_point(imports)



_sym_db = _symbol_database.Default()





from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2





DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0fpm_models.proto\x12\x05\x66icus\x1a\x1fgoogle/protobuf/timestamp.proto\"<\n\x12GrpcSimpleEventLog\x12&\n\x06traces\x18\x01 \x03(\x0b\x32\x16.ficus.GrpcSimpleTrace\"3\n\x0fGrpcSimpleTrace\x12 \n\x06\x65vents\x18\x01 \x03(\x0b\x32\x10.ficus.GrpcEvent\"?\n\tGrpcEvent\x12\x0c\n\x04name\x18\x01 \x01(\t\x12$\n\x05stamp\x18\x02 \x01(\x0b\x32\x15.ficus.GrpcEventStamp\"V\n\x0eGrpcEventStamp\x12*\n\x04\x64\x61te\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampH\x00\x12\x0f\n\x05order\x18\x02 \x01(\x04H\x00\x42\x07\n\x05stamp\"?\n\x12GrpcHashesEventLog\x12)\n\x06traces\x18\x02 \x03(\x0b\x32\x19.ficus.GrpcHashesLogTrace\"$\n\x12GrpcHashesLogTrace\x12\x0e\n\x06\x65vents\x18\x01 \x03(\x04\":\n\x11GrpcNamesEventLog\x12%\n\x06traces\x18\x01 \x03(\x0b\x32\x15.ficus.GrpcNamesTrace\" \n\x0eGrpcNamesTrace\x12\x0e\n\x06\x65vents\x18\x01 \x03(\tb\x06proto3')







_GRPCSIMPLEEVENTLOG = DESCRIPTOR.message_types_by_name['GrpcSimpleEventLog']

_GRPCSIMPLETRACE = DESCRIPTOR.message_types_by_name['GrpcSimpleTrace']

_GRPCEVENT = DESCRIPTOR.message_types_by_name['GrpcEvent']

_GRPCEVENTSTAMP = DESCRIPTOR.message_types_by_name['GrpcEventStamp']

_GRPCHASHESEVENTLOG = DESCRIPTOR.message_types_by_name['GrpcHashesEventLog']

_GRPCHASHESLOGTRACE = DESCRIPTOR.message_types_by_name['GrpcHashesLogTrace']

_GRPCNAMESEVENTLOG = DESCRIPTOR.message_types_by_name['GrpcNamesEventLog']

_GRPCNAMESTRACE = DESCRIPTOR.message_types_by_name['GrpcNamesTrace']

GrpcSimpleEventLog = _reflection.GeneratedProtocolMessageType('GrpcSimpleEventLog', (_message.Message,), {

  'DESCRIPTOR' : _GRPCSIMPLEEVENTLOG,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcSimpleEventLog)

  })

_sym_db.RegisterMessage(GrpcSimpleEventLog)



GrpcSimpleTrace = _reflection.GeneratedProtocolMessageType('GrpcSimpleTrace', (_message.Message,), {

  'DESCRIPTOR' : _GRPCSIMPLETRACE,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcSimpleTrace)

  })

_sym_db.RegisterMessage(GrpcSimpleTrace)



GrpcEvent = _reflection.GeneratedProtocolMessageType('GrpcEvent', (_message.Message,), {

  'DESCRIPTOR' : _GRPCEVENT,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcEvent)

  })

_sym_db.RegisterMessage(GrpcEvent)



GrpcEventStamp = _reflection.GeneratedProtocolMessageType('GrpcEventStamp', (_message.Message,), {

  'DESCRIPTOR' : _GRPCEVENTSTAMP,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcEventStamp)

  })

_sym_db.RegisterMessage(GrpcEventStamp)



GrpcHashesEventLog = _reflection.GeneratedProtocolMessageType('GrpcHashesEventLog', (_message.Message,), {

  'DESCRIPTOR' : _GRPCHASHESEVENTLOG,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcHashesEventLog)

  })

_sym_db.RegisterMessage(GrpcHashesEventLog)



GrpcHashesLogTrace = _reflection.GeneratedProtocolMessageType('GrpcHashesLogTrace', (_message.Message,), {

  'DESCRIPTOR' : _GRPCHASHESLOGTRACE,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcHashesLogTrace)

  })

_sym_db.RegisterMessage(GrpcHashesLogTrace)



GrpcNamesEventLog = _reflection.GeneratedProtocolMessageType('GrpcNamesEventLog', (_message.Message,), {

  'DESCRIPTOR' : _GRPCNAMESEVENTLOG,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcNamesEventLog)

  })

_sym_db.RegisterMessage(GrpcNamesEventLog)



GrpcNamesTrace = _reflection.GeneratedProtocolMessageType('GrpcNamesTrace', (_message.Message,), {

  'DESCRIPTOR' : _GRPCNAMESTRACE,

  '__module__' : 'pm_models_pb2'

  # @@protoc_insertion_point(class_scope:ficus.GrpcNamesTrace)

  })

_sym_db.RegisterMessage(GrpcNamesTrace)



if _descriptor._USE_C_DESCRIPTORS == False:



  DESCRIPTOR._options = None

  _GRPCSIMPLEEVENTLOG._serialized_start=59

  _GRPCSIMPLEEVENTLOG._serialized_end=119

  _GRPCSIMPLETRACE._serialized_start=121

  _GRPCSIMPLETRACE._serialized_end=172

  _GRPCEVENT._serialized_start=174

  _GRPCEVENT._serialized_end=237

  _GRPCEVENTSTAMP._serialized_start=239

  _GRPCEVENTSTAMP._serialized_end=325

  _GRPCHASHESEVENTLOG._serialized_start=327

  _GRPCHASHESEVENTLOG._serialized_end=390

  _GRPCHASHESLOGTRACE._serialized_start=392

  _GRPCHASHESLOGTRACE._serialized_end=428

  _GRPCNAMESEVENTLOG._serialized_start=430

  _GRPCNAMESEVENTLOG._serialized_end=488

  _GRPCNAMESTRACE._serialized_start=490

  _GRPCNAMESTRACE._serialized_end=522

# @@protoc_insertion_point(module_scope)
