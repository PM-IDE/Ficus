from typing import Optional

from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcGraph


class Graph:
    def __init__(self):
        self.nodes = []
        self.edges = []

class GraphNode:
    def __init__(self, id: int, data: Optional[str]):
        self.id = id
        self.data = data

class GraphEdge:
    def __init__(self, from_node: int, to_node: int, data: Optional[str]):
        self.from_node = from_node
        self.to_node = to_node
        self.data = data


def from_grpc_graph(grpc_graph: GrpcGraph) -> Graph:
    graph = Graph()
    for node in grpc_graph.nodes:
        graph.nodes.append(GraphNode(id=node.id, data=node.data))

    for edge in grpc_graph.edges:
        graph.edges.append(GraphEdge(from_node=edge.from_node, to_node=edge.to_node, data=edge.data))

    return graph
