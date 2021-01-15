import numpy as np
from typing import Tuple, List, Set

DIRS = {
    "ne": np.array([0, 1, 1]),
    "e":  np.array([1, 1, 0]),
    "se": np.array([1, 0, -1]),
    "sw": np.array([0, -1, -1]),
    "w":  np.array([-1, -1, 0]),
    "nw": np.array([-1, 0, 1])
}

def split_tile_directions(tile_directions: str) -> List[str]:
    res = []
    while tile_directions:
        for d in ["nw", "ne", "sw", "se", "e", "w"]:
            if tile_directions.startswith(d):
                res.append(d)
                tile_directions = tile_directions[len(d):]
                break
    return res

def get_tile(tile_directions: List[str]) -> Tuple[int, int, int]:
    res = np.array([0, 0, 0])
    for d in tile_directions:
        res += DIRS[d]
    return tuple(res.tolist())

def get_all_adjacent(tile: Tuple[int, int, int]) -> List[Tuple[int, int, int]]:
    res = []
    for d, vec in DIRS.items():
        res.append(tuple(np.array(tile) + vec))
    return res

def get_all_tiles_to_consider(black_tiles: Set[Tuple[int, int, int]]) -> Set[Tuple[int, int, int]]:
    res = set()
    for black_tile in black_tiles:
        res.add(black_tile)
        res.update(get_all_adjacent(black_tile))
    return res

if __name__ == "__main__":
    with open("inputs.txt", "r") as input:
        black_tiles = set()
        for line in input:
            tile = get_tile(split_tile_directions(line.strip()))
            if tile in black_tiles:
                black_tiles.remove(tile)
            else:
                black_tiles.add(tile)

        print("Part 1", len(black_tiles))

        for day in range(100):
            all_tiles = get_all_tiles_to_consider(black_tiles)
            new_black_tiles = set()
            for tile in all_tiles:
                adjacent_black_tiles = len(set(get_all_adjacent(tile)) & black_tiles)
                if tile in black_tiles and adjacent_black_tiles in [1, 2]:
                    new_black_tiles.add(tile)

                if tile not in black_tiles and adjacent_black_tiles == 2:
                    new_black_tiles.add(tile)

            black_tiles = new_black_tiles
        print("Part 2", len(new_black_tiles))



