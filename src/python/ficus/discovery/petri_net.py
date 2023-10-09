import os
import shutil
import tempfile
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
    def __init__(self, id: int, name: str):
        self.id = id
        self.name = name


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


def draw_petri_net(net: PetriNet,
                   show_places_names: bool = False,
                   name: str = 'petri_net',
                   background_color: str = 'white',
                   engine='dot',
                   export_path: Optional[str] = None,
                   rankdir: str = 'LR'):
    tmp_save_file = tempfile.NamedTemporaryFile(suffix='.gv')

    g = graphviz.Digraph(name, engine=engine, graph_attr={
        'bgcolor': background_color,
        'rankdir': rankdir
    })

    initial_marking_places = set()
    if net.initial_marking is not None:
        for single_marking in net.initial_marking.markings:
            initial_marking_places.add(single_marking.place_id)

    final_marking_places = set()
    if net.final_marking is not None:
        for single_marking in net.final_marking.markings:
            final_marking_places.add(single_marking.place_id)

    for place in net.places.values():
        if place.id in initial_marking_places:
            g.node(str(place.id), '<&#9679;>', style='filled', border='1', shape='circle')
        elif place.id in final_marking_places:
            g.node(str(place.id), '<&#9679;>', style='filled', border='1', shape='doublecircle')
        else:
            label = place.name if show_places_names else ''
            g.node(str(place.id), label=label, style='filled', border='1', shape='circle')

    for transition in net.transitions.values():
        g.node(str(transition.id), label=transition.data, shape='box')

    for transition in net.transitions.values():
        for arc in transition.incoming_arcs:
            g.edge(str(arc.place_id), str(transition.id))

        for arc in transition.outgoing_arcs:
            g.edge(str(transition.id), str(arc.place_id))

    g.attr(overlap='false')
    g.save(tmp_save_file.name)

    if export_path is None:
        display(g)
    else:
        dir_name = os.path.dirname(export_path)
        if not os.path.exists(dir_name):
            os.makedirs(dir_name, exist_ok=True)

        _, extension = os.path.splitext(export_path)
        graphviz.render(engine, extension[1::], tmp_save_file.name)
        shutil.move(tmp_save_file.name + extension, export_path)

