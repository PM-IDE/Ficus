from typing import Optional

import graphviz
from IPython.core.display_functions import display


class PetriNet:
    def __init__(self):
        self.places: dict[int, Place] = dict()
        self.transitions: dict[int, Transition] = dict()
        self.initial_marking: Optional[Marking] = None
        self.final_marking: Optional[Marking] = None


class Place:
    def __init__(self, id: int):
        self.id = id


class Transition:
    def __init__(self, id: int):
        self.id = id
        self.incoming_arcs: list[Arc] = []
        self.outgoing_arcs: list[Arc] = []
        self.data: Optional[str] = None


class Arc:
    def __init__(self, place_id: int):
        self.place_id = place_id


class Marking:
    def __init__(self, markings: list['SinglePlaceMarking']):
        self.markings = markings


class SinglePlaceMarking:
    def __init__(self, place_id: int, tokens_count: int):
        self.place_id = place_id
        self.tokens_count = tokens_count


def draw_petri_net(net: PetriNet, name: str = 'petri_net'):
    g = graphviz.Digraph(name, engine='dot')
    g.graph_attr['rankdir'] = 'LR'

    initial_marking_places = set()
    if net.initial_marking is not None:
        for single_marking in net.initial_marking.markings:
            initial_marking_places.add(single_marking.place_id)

    final_marking_places = set()
    if net.final_marking is not None:
        for single_marking in net.final_marking.markings:
            final_marking_places.add(single_marking.place_id)

    print(initial_marking_places, final_marking_places)

    for place in net.places.values():
        if place.id in initial_marking_places:
            g.node(str(place.id), '<&#9679;>', style='filled', border='1', shape='circle')
        elif place.id in final_marking_places:
            g.node(str(place.id), '<&#9632;>', style='filled', border='1', shape='doublecircle')
        else:
            g.node(str(place.id), '', style='filled', border='1', shape='circle')

    for transition in net.transitions.values():
        g.node(str(transition.id), label=transition.data, shape='box')

    for transition in net.transitions.values():
        for arc in transition.incoming_arcs:
            g.edge(str(arc.place_id), str(transition.id))

        for arc in transition.outgoing_arcs:
            g.edge(str(transition.id), str(arc.place_id))

    g.attr(overlap='false')

    display(g)
