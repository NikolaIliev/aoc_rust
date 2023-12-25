from dataclasses import dataclass
import numpy as np


@dataclass
class Point:
    x: int
    y: int
    z: int


@dataclass
class Vector:
    position: Point
    velocity: Point


vectors = []

for line in open("./inputs/year2023_day24.txt").read().strip().split("\n"):
    position, velocity = line.replace("  ", " ").split(" @ ")
    vectors.append(
        Vector(
            Point(*map(int, position.split(", "))),
            Point(*map(int, velocity.split(", "))),
        )
    )

# We only need three vectors that are linearly independent and then
# can solve it using linear algebra.
v0 = vectors[0].velocity
v1 = vectors[1].velocity
v2 = vectors[2].velocity
p0 = vectors[0].position
p1 = vectors[1].position
p2 = vectors[2].position

# Solving A * x = b
A = np.array(
    [
        [v1.y - v0.y, v0.x - v1.x, 0.0, p0.y - p1.y, p1.x - p0.x, 0.0],
        [v2.y - v0.y, v0.x - v2.x, 0.0, p0.y - p2.y, p2.x - p0.x, 0.0],
        [v1.z - v0.z, 0.0, v0.x - v1.x, p0.z - p1.z, 0.0, p1.x - p0.x],
        [v2.z - v0.z, 0.0, v0.x - v2.x, p0.z - p2.z, 0.0, p2.x - p0.x],
        [0.0, v1.z - v0.z, v0.y - v1.y, 0.0, p0.z - p1.z, p1.y - p0.y],
        [0.0, v2.z - v0.z, v0.y - v2.y, 0.0, p0.z - p2.z, p2.y - p0.y],
    ]
)

b = [
    (p0.y * v0.x - p1.y * v1.x) - (p0.x * v0.y - p1.x * v1.y),
    (p0.y * v0.x - p2.y * v2.x) - (p0.x * v0.y - p2.x * v2.y),
    (p0.z * v0.x - p1.z * v1.x) - (p0.x * v0.z - p1.x * v1.z),
    (p0.z * v0.x - p2.z * v2.x) - (p0.x * v0.z - p2.x * v2.z),
    (p0.z * v0.y - p1.z * v1.y) - (p0.y * v0.z - p1.y * v1.z),
    (p0.z * v0.y - p2.z * v2.y) - (p0.y * v0.z - p2.y * v2.z),
]

rock = np.linalg.solve(A, b)
print(round(rock[0] + rock[1] + rock[2]))
