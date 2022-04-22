import math
import sys
from typing import Tuple, Optional, List

import numpy as np

SIZE_X = 17630
SIZE_Y = 9000

base_x, base_y = [int(i) for i in input().split()]
base_pos = (base_x, base_y)
opponent_pos = (abs(base_x - SIZE_X), abs(base_y - SIZE_Y))
heroes_per_player = int(input())  # Always 3


def hprinterr(hero: int, msg: str):
    print(f"{hero}: {msg}", file=sys.stderr, flush=True)


def mprinterr(monster: int, msg: str):
    print(f"{monster}: {msg}", file=sys.stderr, flush=True)


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
    pursued_by: List[Unit]
    speed: Tuple[int, int]
    will_hit_in: int

    def __init__(self, _id, position, _near_base, is_threat, _health, speed):
        super().__init__(_id, position, _health)
        self.near_base = _near_base
        self.is_threat = is_threat
        self.pursued_by = []
        self.speed = speed

    def update(self, position, _near_base, is_threat, _health, speed):
        self.position = position
        self.health = _health
        self.near_base = _near_base
        self.is_threat = is_threat
        self.speed = speed
        if self.is_threat:
            self._calculate_time_before_hitting()
            mprinterr(self._id, f"Will hit in {self.will_hit_in}")

    def _calculate_time_before_hitting(self):
        self.will_hit_in = np.linalg.norm((base_x - self.position[0], base_y - self.position[1])) / np.linalg.norm(
            self.speed)


class HeroBase(Unit):
    IDLE_POS = 2500
    MIN_DEF_RANGE = 6500
    pursuing: Optional[Monster]
    dist_to_base: float

    def __init__(self, _id, position, _health):
        super().__init__(_id, position, _health)
        self.dist_to_base = 0
        self.pursuing = None

    def update(self, position):
        self.position = position
        self.dist_to_base = self.dist_to(base_pos)

    def pursue(self, monster: Monster):
        if self.pursuing is not None:
            self.stop_pursuing()
        self.pursuing = monster
        monster.pursued_by.append(self)

    def stop_pursuing(self):
        if self.pursuing is not None:
            self.pursuing.pursued_by.remove(self)
        self.pursuing = None

    def update_threats(self, _threats: List[Monster]):
        raise NotImplementedError()

    def next_move(self) -> str:
        if self.pursuing is not None:
            return f"MOVE {self.pursuing.position[0]} {self.pursuing.position[1]}"
        else:
            return f"MOVE {abs(base_x - self.IDLE_POS)} {abs(base_y - self.IDLE_POS)}"


class Hero(HeroBase):
    def __init__(self, _id, position, _health):
        super().__init__(_id, position, _health)

    def update_threats(self, _threats: List[Monster]):
        direct_threats = [t for t in _threats if len(t.pursued_by) <= 0]

        biggest_threat = direct_threats[0] if len(direct_threats) > 0 \
            else _threats[0] if len(_threats) > 0 and _threats[0].dist_to(base_pos) < self.MIN_DEF_RANGE \
            else None

        if biggest_threat is not None:
            if self.pursuing is None:
                self.pursue(biggest_threat)
            elif self.should_stop_pursuing(biggest_threat):
                self.pursue(biggest_threat)
        else:
            if self.pursuing is not None:
                self.stop_pursuing()

    def should_stop_pursuing(self, threat: Monster) -> bool:
        dist_to_pursuing = self.dist_to(self.pursuing.position)
        dist_to_other = self.dist_to(threat.position)
        other_is_pursued = len(threat.pursued_by) > 0
        mine_is_pursued_by_others = len(threat.pursued_by) > 1
        return dist_to_pursuing > dist_to_other


class Defender(HeroBase):
    def __init__(self, _id, position, _health):
        super().__init__(_id, position, _health)

    def update_threats(self, _threats: List[Monster]):
        direct_threats = [t for t in _threats if len(t.pursued_by) <= 0 and t.dist_to(base_pos) < self.MIN_DEF_RANGE]

        biggest_threat = direct_threats[0] if len(direct_threats) > 0 \
            else _threats[0] if len(_threats) > 0 and _threats[0].dist_to(base_pos) < self.MIN_DEF_RANGE \
            else None

        if biggest_threat is not None:
            if self.pursuing is None:
                self.pursue(biggest_threat)
            elif self.should_stop_pursuing(biggest_threat):
                self.pursue(biggest_threat)
        else:
            if self.pursuing is not None:
                self.stop_pursuing()

    def should_stop_pursuing(self, threat: Monster) -> bool:
        dist_to_pursuing = self.dist_to(self.pursuing.position)
        dist_to_other = self.dist_to(threat.position)
        other_is_pursued = len(threat.pursued_by) > 0
        mine_is_pursued_by_others = len(threat.pursued_by) > 1
        return dist_to_pursuing > dist_to_other


def heuristic(threat: Monster) -> float:
    return threat.dist_to((base_x, base_y))


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
                heroes[_id] = (Hero if len(heroes) < 2 else Defender)(_id, (x, y), health)
            else:
                heroes[_id].update((x, y))
        elif _type == 0:
            if _id not in monsters:
                monsters[_id] = Monster(_id, (x, y), near_base, threat_for == 1, health, (vx, vy))
            else:
                monsters[_id].update((x, y), near_base, threat_for == 1, health, (vx, vy))

    threats = [v for v in monsters.values() if v.is_threat]
    threats.sort(key=lambda t: heuristic(t))

    for h in heroes.values():
        h.update_threats(threats)

    for i in range(heroes_per_player):
        print(list(heroes.values())[i].next_move())
