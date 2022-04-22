import math
import sys
from typing import Tuple, Optional, List

SIZE_X = 17630
SIZE_Y = 9000

base_x, base_y = [int(i) for i in input().split()]
base_pos = (base_x, base_y)
opponent_pos = (abs(base_x - SIZE_X), abs(base_y - SIZE_Y))
heroes_per_player = int(input())  # Always 3

PATROL_POINTS = [((abs(base_x - 6500), abs(base_y - 3000)),
                  (abs(base_x - 7000), abs(base_y - 1000))),
                 ((abs(base_x - 4500), abs(base_y - 5000)),
                  (abs(base_x - 3000), abs(base_y - 6500)))]


def hprinterr(hero: int, msg: str):
    print(f"{hero}: {msg}", file=sys.stderr, flush=True)


def mprinterr(monster: int, msg: str):
    print(f"{monster}: {msg}", file=sys.stderr, flush=True)


class Facts:
    monsters_inside_security: int = 0


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
    current_health: int
    lasting_health_percent: float
    will_be_controlled_by: Optional[Unit]
    is_controlled: bool

    def __init__(self, _id, position, _near_base, is_threat, _health, speed, _is_controlled):
        super().__init__(_id, position, _health)
        self.near_base = _near_base
        self.is_threat = is_threat
        self.pursued_by = []
        self.speed = speed
        self.lasting_health_percent = 1
        self.current_health = self.health
        self.will_be_controlled_by = None
        self.is_controlled = _is_controlled

    def update(self, position, _near_base, is_threat, _health, speed, _is_controlled):
        self.position = position
        self.current_health = _health
        self.lasting_health_percent = self.current_health / self.health
        self.near_base = _near_base
        self.is_threat = is_threat
        self.speed = speed
        self.will_be_controlled_by = None
        self.is_controlled = _is_controlled
        if self.health <= 0:
            self.dead()

    def dead(self):
        for hero in self.pursued_by:
            hero.pursuing = None


class HeroBase(Unit):
    IDLE_POS = 2500
    MIN_DEF_RANGE = 6500
    MIN_WIND_DIST = 3800.
    MIN_CUMULATED_HEALTH_TO_CONTROL = 20
    MAX_MONSTERS_INSIDE_SECURITY = 3
    MAX_DIST_TO_BASE_TO_CONTROL = 10000
    MIN_WIND_DIST_AROUND_HERO = 1300
    MIN_DIST_TO_CONTROL = 2200
    pursuing: Optional[Monster]
    dist_to_base: float
    current_threats: List[Monster]

    def __init__(self, _id, position, _health):
        super().__init__(_id, position, _health)
        self.dist_to_base = 0
        self.pursuing = None
        self.current_threats = []

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
        self.current_threats = _threats

    def should_stop_pursuing(self, threat: Monster) -> bool:
        dist_to_pursuing = self.dist_to(self.pursuing.position)
        dist_to_other = self.dist_to(threat.position)
        return abs(dist_to_pursuing - dist_to_other) > 500

    def define_biggest_threat(self, direct_threats: List[Monster], _threats: List[Monster]) -> Optional[Monster]:
        direct_threats.sort(key=lambda k: k.dist_to(self.position))
        return direct_threats[0] if len(direct_threats) > 0 \
            else _threats[0] if len(_threats) > 0 and _threats[0].dist_to(base_pos) < self.MIN_DEF_RANGE \
            else None

    def should_wind(self):
        if mana < 10 or self.dist_to(
                self.pursuing.position) > self.MIN_WIND_DIST_AROUND_HERO or self.pursuing.is_controlled: return False
        dist_to_base = self.pursuing.dist_to(base_pos)
        if dist_to_base > self.MIN_WIND_DIST: return False
        percent_near = 1 - (dist_to_base / self.MIN_WIND_DIST)
        hprinterr(self._id, f"Near at {percent_near:.2f}%")
        max_health = (1 - (0.75 * percent_near))
        hprinterr(self._id, f"Will wind at {max_health:.2f}% (currently {self.pursuing.lasting_health_percent})")
        return self.pursuing.lasting_health_percent > max_health and mana >= 10

    def should_control(self) -> Optional[Monster]:
        if Facts.monsters_inside_security >= self.MAX_MONSTERS_INSIDE_SECURITY or mana < 10: return None
        concerned = [t for t in self.current_threats if
                     not t.near_base and t.dist_to(base_pos) < self.MAX_DIST_TO_BASE_TO_CONTROL and self.dist_to(
                         t.position) < self.MIN_DIST_TO_CONTROL and not t.is_controlled]
        if len(concerned) <= 0 or sum(
                map(lambda t: t.health, concerned)) < self.MIN_CUMULATED_HEALTH_TO_CONTROL: return None
        concerned.sort(key=lambda k: k.current_health, reverse=True)
        mob = concerned[0]
        mob.will_be_controlled_by = self
        return mob

    def next_move(self) -> str:
        global mana
        if self.pursuing is not None:
            if self.should_wind():
                mana -= 10
                return f"SPELL WIND {opponent_pos[0]} {opponent_pos[1]}"
            return f"MOVE {self.pursuing.position[0]} {self.pursuing.position[1]}"
        else:
            return f"MOVE {abs(base_x - self.IDLE_POS)} {abs(base_y - self.IDLE_POS)}"


class Patroller(HeroBase):
    patrol_points: Tuple[Tuple[int, int], Tuple[int, int]]
    current_point: int

    def __init__(self, _id, position, _health, patrol_points):
        super().__init__(_id, position, _health)
        self.patrol_points = patrol_points
        self.current_point = 0

    def update_threats(self, _threats: List[Monster]):
        super(Patroller, self).update_threats(_threats)
        direct_threats = [t for t in _threats if len(t.pursued_by) <= 0]
        biggest_threat = self.define_biggest_threat(direct_threats, _threats)

        if biggest_threat is not None:
            if self.pursuing is None:
                self.pursue(biggest_threat)
            elif self.should_stop_pursuing(biggest_threat):
                self.pursue(biggest_threat)
        else:
            if self.pursuing is not None:
                self.stop_pursuing()

    def next_move(self) -> str:
        global mana
        should_control = self.should_control()
        if should_control is not None:
            mana -= 10
            return f"SPELL CONTROL {should_control._id} {opponent_pos[0]} {opponent_pos[1]}"

        if self.pursuing is not None:
            if self.should_wind():
                mana -= 10
                return f"SPELL WIND {opponent_pos[0]} {opponent_pos[1]}"
            return f"MOVE {self.pursuing.position[0]} {self.pursuing.position[1]}"
        else:
            if self.dist_to(self.patrol_points[self.current_point]) < 100:
                self.current_point = (self.current_point + 1) % 2
            direction = self.patrol_points[self.current_point]
            return f"MOVE {direction[0]} {direction[1]}"


class Defender(HeroBase):
    def __init__(self, _id, position, _health):
        super().__init__(_id, position, _health)

    def update_threats(self, _threats: List[Monster]):
        super(Defender, self).update_threats(_threats)
        direct_threats = [t for t in _threats if len(t.pursued_by) <= 0 and t.dist_to(base_pos) < self.MIN_DEF_RANGE]
        biggest_threat = self.define_biggest_threat(direct_threats, _threats)

        if biggest_threat is not None:
            if self.pursuing is None:
                self.pursue(biggest_threat)
            elif self.should_stop_pursuing(biggest_threat):
                self.pursue(biggest_threat)
        else:
            if self.pursuing is not None:
                self.stop_pursuing()


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
                if len(heroes) >= 2:
                    heroes[_id] = Defender(_id, (x, y), health)
                else:
                    heroes[_id] = Patroller(_id, (x, y), health, PATROL_POINTS[len(heroes)])
            else:
                heroes[_id].update((x, y))
        elif _type == 0:
            if _id not in monsters:
                monsters[_id] = Monster(_id, (x, y), near_base, threat_for == 1, health, (vx, vy), is_controlled)
            else:
                monsters[_id].update((x, y), near_base, threat_for == 1, health, (vx, vy), is_controlled)

    threats = [v for v in monsters.values() if v.is_threat]
    threats.sort(key=lambda t: heuristic(t))

    Facts.monsters_inside_security = sum(map(lambda t: t.near_base, threats))

    for h in heroes.values():
        h.update_threats(threats)

    for i in range(heroes_per_player):
        print(list(heroes.values())[i].next_move())
