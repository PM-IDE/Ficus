# -*- coding: utf-8 -*-

# Generated by the protocol buffer compiler.  DO NOT EDIT!

# source: pm_models.proto

"""Generated protocol buffer code."""

from google.protobuf import descriptor as _descriptor

from google.protobuf import descriptor_pool as _descriptor_pool

from google.protobuf import symbol_database as _symbol_database

from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)



_sym_db = _symbol_database.Default()





from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2





DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x0fpm_models.proto\x12\x05\x66icus\x1a\x1fgoogle/protobuf/timestamp.proto\"<\n\x12GrpcSimpleEventLog\x12&\n\x06traces\x18\x01 \x03(\x0b\x32\x16.ficus.GrpcSimpleTrace\"3\n\x0fGrpcSimpleTrace\x12 \n\x06\x65vents\x18\x01 \x03(\x0b\x32\x10.ficus.GrpcEvent\"?\n\tGrpcEvent\x12\x0c\n\x04name\x18\x01 \x01(\t\x12$\n\x05stamp\x18\x02 \x01(\x0b\x32\x15.ficus.GrpcEventStamp\"V\n\x0eGrpcEventStamp\x12*\n\x04\x64\x61te\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.TimestampH\x00\x12\x0f\n\x05order\x18\x02 \x01(\x04H\x00\x42\x07\n\x05stamp\"?\n\x12GrpcHashesEventLog\x12)\n\x06traces\x18\x02 \x03(\x0b\x32\x19.ficus.GrpcHashesLogTrace\"$\n\x12GrpcHashesLogTrace\x12\x0e\n\x06\x65vents\x18\x01 \x03(\x04\":\n\x11GrpcNamesEventLog\x12%\n\x06traces\x18\x01 \x03(\x0b\x32\x15.ficus.GrpcNamesTrace\" \n\x0eGrpcNamesTrace\x12\x0e\n\x06\x65vents\x18\x01 \x03(\t\"\x99\x01\n\x0cGrpcPetriNet\x12(\n\x06places\x18\x01 \x03(\x0b\x32\x18.ficus.GrpcPetriNetPlace\x12\x32\n\x0btransitions\x18\x02 \x03(\x0b\x32\x1d.ficus.GrpcPetriNetTransition\x12+\n\x07marking\x18\x03 \x01(\x0b\x32\x1a.ficus.GrpcPetriNetMarking\"\x1f\n\x11GrpcPetriNetPlace\x12\n\n\x02id\x18\x01 \x01(\x03\"\x82\x01\n\x16GrpcPetriNetTransition\x12,\n\x0cincomingArcs\x18\x01 \x03(\x0b\x32\x16.ficus.GrpcPetriNetArc\x12,\n\x0coutgoingArcs\x18\x02 \x03(\x0b\x32\x16.ficus.GrpcPetriNetArc\x12\x0c\n\x04\x64\x61ta\x18\x03 \x01(\t\"\"\n\x0fGrpcPetriNetArc\x12\x0f\n\x07placeId\x18\x01 \x01(\x03\"N\n\x13GrpcPetriNetMarking\x12\x37\n\x08Markings\x18\x01 \x03(\x0b\x32%.ficus.GrpcPetriNetSinglePlaceMarking\"F\n\x1eGrpcPetriNetSinglePlaceMarking\x12\x0f\n\x07placeId\x18\x01 \x01(\x03\x12\x13\n\x0btokensCount\x18\x02 \x01(\x03\x62\x06proto3')



_globals = globals()

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)

_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'pm_models_pb2', _globals)

if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None

  _globals['_GRPCSIMPLEEVENTLOG']._serialized_start=59

  _globals['_GRPCSIMPLEEVENTLOG']._serialized_end=119

  _globals['_GRPCSIMPLETRACE']._serialized_start=121

  _globals['_GRPCSIMPLETRACE']._serialized_end=172

  _globals['_GRPCEVENT']._serialized_start=174

  _globals['_GRPCEVENT']._serialized_end=237

  _globals['_GRPCEVENTSTAMP']._serialized_start=239

  _globals['_GRPCEVENTSTAMP']._serialized_end=325

  _globals['_GRPCHASHESEVENTLOG']._serialized_start=327

  _globals['_GRPCHASHESEVENTLOG']._serialized_end=390

  _globals['_GRPCHASHESLOGTRACE']._serialized_start=392

  _globals['_GRPCHASHESLOGTRACE']._serialized_end=428

  _globals['_GRPCNAMESEVENTLOG']._serialized_start=430

  _globals['_GRPCNAMESEVENTLOG']._serialized_end=488

  _globals['_GRPCNAMESTRACE']._serialized_start=490

  _globals['_GRPCNAMESTRACE']._serialized_end=522

  _globals['_GRPCPETRINET']._serialized_start=525

  _globals['_GRPCPETRINET']._serialized_end=678

  _globals['_GRPCPETRINETPLACE']._serialized_start=680

  _globals['_GRPCPETRINETPLACE']._serialized_end=711

  _globals['_GRPCPETRINETTRANSITION']._serialized_start=714

  _globals['_GRPCPETRINETTRANSITION']._serialized_end=844

  _globals['_GRPCPETRINETARC']._serialized_start=846

  _globals['_GRPCPETRINETARC']._serialized_end=880

  _globals['_GRPCPETRINETMARKING']._serialized_start=882

  _globals['_GRPCPETRINETMARKING']._serialized_end=960

  _globals['_GRPCPETRINETSINGLEPLACEMARKING']._serialized_start=962

  _globals['_GRPCPETRINETSINGLEPLACEMARKING']._serialized_end=1032

# @@protoc_insertion_point(module_scope)
