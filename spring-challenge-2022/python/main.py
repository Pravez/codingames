import math
import sys
from typing import Tuple, Optional, List

base_x, base_y = [int(i) for i in input().split()]
base_pos = (base_x, base_y)
heroes_per_player = int(input())  # Always 3


def printerr(hero: int, msg: str):
    print(f"{hero}: {msg}", file=sys.stderr, flush=True)


class Unit:
    _id: int
    position: Tuple[int, int]
    health: int

    def __init__(self, _id, position, _health):
        self._id = _id
        self.position = position
        self.health = _health

    def dist_to(self, other: Tuple[int, int]):
        return math.dist(self.position, other)


class Monster(Unit):
    near_base: bool
    is_threat: bool
    pursued_by: Optional[List[Unit]]

    def __init__(self, _id, position, _near_base, is_threat, _health):
        super().__init__(_id, position, _health)
        self.near_base = _near_base
        self.is_threat = is_threat
        self.pursued_by = None

    def update(self, position, _near_base, is_threat, _health):
        self.position = position
        self.health = _health
        self.near_base = _near_base
        self.is_threat = is_threat


class Hero(Unit):
    MAX_DIST_TO_BASE = 1500
    pursuing: Optional[Monster]
    dist_to_base: float

    def __init__(self, _id, position, _health):
        super().__init__(_id, position, _health)
        self.dist_to_base = 0
        self.pursuing = None

    def update(self, position):
        self.position = position
        self.dist_to_base = self.dist_to(base_pos)

    def update_threats(self, _threats: List[Monster]):
        direct_threats = [t for t in _threats if t.pursued_by is None or (t.near_base and t.health >= 9)]
        if len(direct_threats) > 0:
            self.pursue(direct_threats[0])
        else:
            self.stop_pursuing()
            if len(_threats) > 0:
                self.pursue(_threats[0])

    def pursue(self, monster: Monster):
        self.pursuing = monster
        monster.pursued_by = [self] if monster.pursued_by is None else monster.pursued_by + [self]

    def stop_pursuing(self):
        if self.pursuing is not None:
            self.pursuing.pursued_by = None
        self.pursuing = None

    def next_move(self) -> str:
        if self.pursuing is not None:
            return f"MOVE {self.pursuing.position[0]} {self.pursuing.position[1]}"
        elif self.dist_to_base > self.MAX_DIST_TO_BASE:
            return f"MOVE {base_x} {base_y}"
        else:
            return "WAIT"


def heuristic(threat: Monster) -> float:
    return threat.dist_to((base_x, base_y)) * 100 - threat.health


# game loop
while True:

    heroes = {}
    monsters = {}

    for i in range(2):
        health, mana = [int(j) for j in input().split()]
    entity_count = int(input())
    for i in range(entity_count):
        _id, _type, x, y, shield_life, is_controlled, health, vx, vy, near_base, threat_for = [int(j) for j in
                                                                                               input().split()]
        if _type == 1:
            if _id not in heroes:
                heroes[_id] = Hero(_id, (x, y), health)
            else:
                heroes[_id].update((x, y))
        elif _type == 0:
            if _id not in monsters:
                monsters[_id] = Monster(_id, (x, y), near_base, threat_for == 1, health)
            else:
                monsters[_id].update((x, y), near_base, threat_for == 1, health)

    threats = [v for v in monsters.values() if v.is_threat]
    threats.sort(key=lambda t: heuristic(t))

    for h in heroes.values():
        h.update_threats(threats)

    for i in range(heroes_per_player):
        print(list(heroes.values())[i].next_move())
