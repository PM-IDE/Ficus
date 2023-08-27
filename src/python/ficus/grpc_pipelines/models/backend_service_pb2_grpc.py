# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!

"""Client and server classes corresponding to protobuf-defined services."""

import grpc



import ficus.grpc_pipelines.models.backend_service_pb2 as backend__service__pb2





class GrpcBackendServiceStub(object):

    """Missing associated documentation comment in .proto file."""



    def __init__(self, channel):

        """Constructor.



        Args:

            channel: A grpc.Channel.

        """

        self.ExecutePipeline = channel.unary_unary(

                '/ficus.GrpcBackendService/ExecutePipeline',

                request_serializer=backend__service__pb2.GrpcPipelineExecutionRequest.SerializeToString,

                response_deserializer=backend__service__pb2.GrpcPipelineExecutionResult.FromString,

                )

        self.GetContextValue = channel.unary_unary(

                '/ficus.GrpcBackendService/GetContextValue',

                request_serializer=backend__service__pb2.GrpcGetContextValueRequest.SerializeToString,

                response_deserializer=backend__service__pb2.GrpcGetContextValueResult.FromString,

                )





class GrpcBackendServiceServicer(object):

    """Missing associated documentation comment in .proto file."""



    def ExecutePipeline(self, request, context):

        """Missing associated documentation comment in .proto file."""

        context.set_code(grpc.StatusCode.UNIMPLEMENTED)

        context.set_details('Method not implemented!')

        raise NotImplementedError('Method not implemented!')



    def GetContextValue(self, request, context):

        """Missing associated documentation comment in .proto file."""

        context.set_code(grpc.StatusCode.UNIMPLEMENTED)

        context.set_details('Method not implemented!')

        raise NotImplementedError('Method not implemented!')





def add_GrpcBackendServiceServicer_to_server(servicer, server):

    rpc_method_handlers = {

            'ExecutePipeline': grpc.unary_unary_rpc_method_handler(

                    servicer.ExecutePipeline,

                    request_deserializer=backend__service__pb2.GrpcPipelineExecutionRequest.FromString,

                    response_serializer=backend__service__pb2.GrpcPipelineExecutionResult.SerializeToString,

            ),

            'GetContextValue': grpc.unary_unary_rpc_method_handler(

                    servicer.GetContextValue,

                    request_deserializer=backend__service__pb2.GrpcGetContextValueRequest.FromString,

                    response_serializer=backend__service__pb2.GrpcGetContextValueResult.SerializeToString,

            ),

    }

    generic_handler = grpc.method_handlers_generic_handler(

            'ficus.GrpcBackendService', rpc_method_handlers)

    server.add_generic_rpc_handlers((generic_handler,))





 # This class is part of an EXPERIMENTAL API.

class GrpcBackendService(object):

    """Missing associated documentation comment in .proto file."""



    @staticmethod

    def ExecutePipeline(request,

            target,

            options=(),

            channel_credentials=None,

            call_credentials=None,

            insecure=False,

            compression=None,

            wait_for_ready=None,

            timeout=None,

            metadata=None):

        return grpc.experimental.unary_unary(request, target, '/ficus.GrpcBackendService/ExecutePipeline',

            backend__service__pb2.GrpcPipelineExecutionRequest.SerializeToString,

            backend__service__pb2.GrpcPipelineExecutionResult.FromString,

            options, channel_credentials,

            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)



    @staticmethod

    def GetContextValue(request,

            target,

            options=(),

            channel_credentials=None,

            call_credentials=None,

            insecure=False,

            compression=None,

            wait_for_ready=None,

            timeout=None,

            metadata=None):

        return grpc.experimental.unary_unary(request, target, '/ficus.GrpcBackendService/GetContextValue',

            backend__service__pb2.GrpcGetContextValueRequest.SerializeToString,

            backend__service__pb2.GrpcGetContextValueResult.FromString,

            options, channel_credentials,

            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)