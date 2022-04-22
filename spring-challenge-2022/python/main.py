import math
from typing import Tuple

base_x, base_y = [int(i) for i in input().split()]
heroes_per_player = int(input())  # Always 3


class Unit:
    _id: int
    position: Tuple[int, int]

    def __init__(self, _id, position):
        self._id = _id
        self.position = position

    def dist_to(self, other: Tuple[int, int]):
        return math.dist(self.position, other)


class Monster(Unit):
    near_base: bool
    is_threat: bool

    def __init__(self, _id, position, _near_base, is_threat):
        super().__init__(_id, position)
        self.near_base = _near_base
        self.is_threat = is_threat

    def update(self, position, _near_base, is_threat):
        self.position = position
        self.near_base = _near_base
        self.is_threat = is_threat


class Hero(Unit):
    pursuing: Unit

    def __init__(self, _id, position, ):
        super().__init__(_id, position)


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
                heroes[_id] = Hero(_id, (x, y))
            else:
                heroes[_id].position = (x, y)
        elif _type == 0:
            if _id not in monsters:
                monsters[_id] = Monster(_id, (x, y), near_base, threat_for == 1)
            else:
                monsters[_id].update((x, y), near_base, threat_for == 1)

    threats = [v for v in monsters.values() if v.is_threat]
    threats.sort(key=lambda t: t.dist_to((base_x, base_y)))

    objective = "WAIT" if len(threats) <= 0 else f"MOVE {threats[0].position[0]} {threats[0].position[1]}"

    for i in range(heroes_per_player):
        # In the first league: MOVE <x> <y> | WAIT; In later leagues: | SPELL <spellParams>;
        print(objective)
