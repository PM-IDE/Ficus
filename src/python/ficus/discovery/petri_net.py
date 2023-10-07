from typing import Optional


class PetriNet:
    def __init__(self):
        self.places: dict[int, Place] = dict()
        self.transitions: dict[int, Transition] = dict()


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
