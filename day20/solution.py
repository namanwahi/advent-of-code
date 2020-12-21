import numpy as np
from pprint import pprint
import itertools

OPPOSITE_DIRECTIONS = {
    "b": "t",
    "t": "b",
    "l": "r",
    "r": "l",
}

def parse_data(lines):
    tid = None
    curr_img = None
    images = {}
    for line in filter(None, lines):
        if line.startswith("Tile "):
            if tid:
                images[tid] = np.array(curr_img)
            tid = int(line.replace("Tile ", "").replace(":", ""))
            curr_img = []
        else:
            curr_img.append([c for c in line])

    images[tid] = np.array(curr_img)
    return images

def transform(arr, x, y, rots):
    res = arr

    if x:
        res = np.flip(arr, 1)
    if y:
        res = np.flip(arr, 0)

    res = np.rot90(res, k=rots)

    return res


def tile_to_edges(tiles):
    res = {}
    for tid, tile in tiles.items():
        for (transformed_tile, x_flip, y_flip, rots) in get_all_transformations(tile):
            res[(tid, x_flip, y_flip, rots)] = {
                "t": transformed_tile[0, :].squeeze(),
                "b": transformed_tile[-1, :].squeeze(),
                "l": transformed_tile[:, 0].squeeze(),
                "r": transformed_tile[:, -1].squeeze(),
            }
    return res

def get_all_transformations(tile):
    res = []
    for (x_flip, y_flip) in [(False, False), (False, True), (True, False), (True, True)]:
        for rots in [0, 1, 2, 3]:
            res.append((transform(tile, x_flip, y_flip, rots), x_flip, y_flip, rots))
    return res

def adjacent_tiles(edges):
    full_connections = {}
    for tid, x_flip, y_flip, rots in edges:
        for dir in ["t", "b", "l", "r"]:
            full_connections[(tid, x_flip, y_flip, rots, dir)] = set()

    for this_tile, this_edges in edges.items():
        for that_tile, that_edges in edges.items():
            if this_tile[0] == that_tile[0]:
                continue

            for this_dir in ["t", "b", "l", "r"]:
                that_dir = OPPOSITE_DIRECTIONS[this_dir]

                if (this_edges[this_dir] == that_edges[that_dir]).all():
                    this_key = (*this_tile, this_dir)
                    full_connections[this_key].add(that_tile)

    res = {}
    for (tid, _, _, _, _), connections in full_connections.items():
        if tid not in res:
            res[tid] = set()

        for (other_tid, _, _, _) in connections:
            res[tid].add(other_tid)

    return res


def reassemble(adjacent):
    all_tids = adjacent.keys()
    num_tiles = len(all_tids)
    num_tiles_edge = int(round(num_tiles ** 0.5))
    tile_assignments = np.repeat(None, num_tiles).reshape((num_tiles_edge, num_tiles_edge))

    pos_to_coord = lambda x: (x // num_tiles_edge, x % num_tiles_edge)

    def _reassemble(pos, tile_assignments, remaining_tids):
        assert len(remaining_tids) + pos == num_tiles

        if pos == num_tiles:
            return tile_assignments

        (i, j) = pos_to_coord(pos)
        for tid in remaining_tids:
            if j > 0: # check left
                left_tile = tile_assignments[i, j - 1]
                assert left_tile is not None
                if left_tile not in adjacent[tid]:
                    continue

            if i > 0: # check top
                above_tile = tile_assignments[i - 1, j]
                assert above_tile is not None
                if above_tile not in adjacent[tid]:
                    continue

            new_tile_assingments = tile_assignments.copy()
            new_tile_assingments[i, j] = tid
            res = _reassemble(pos + 1, new_tile_assingments, remaining_tids - {tid})
            if res is not None:
                return res

        return None

    return _reassemble(0, tile_assignments, all_tids)

def stich(tiles, tile_positions):
    all_tids = tiles.keys()
    num_tiles = len(all_tids)
    num_tiles_edge = int(round(num_tiles ** 0.5))
    transformed_tiles = np.repeat(None, num_tiles).reshape((num_tiles_edge, num_tiles_edge))

    pos_to_coord = lambda x: (x // num_tiles_edge, x % num_tiles_edge)

    def _stich(pos, transformed_tiles):
        if pos == num_tiles:
            return transformed_tiles

        (i, j) = pos_to_coord(pos)
        for tile, _, _, _ in get_all_transformations(tiles[tile_positions[i][j]]):
            if j > 0: # check left
                left_tile = transformed_tiles[i, j - 1]
                assert left_tile is not None
                if not (left_tile[:, -1] == tile[:, 0]).all():
                    continue

            if i > 0: # check top
                above_tile = transformed_tiles[i - 1, j]
                assert above_tile is not None
                if not (above_tile[-1, :] == tile[0, :]).all():
                    continue

            new_transformed_tiles = transformed_tiles.copy()
            new_transformed_tiles[i, j] = tile
            res = _stich(pos + 1, new_transformed_tiles)
            if res is not None:
                return res

        return None

    return np.array(_stich(0, transformed_tiles).tolist())

def show_tile(tile):
    for row in tile:
        print("".join(row))


if __name__ == "__main__":
    with open("inputs.txt", "r") as f:
        lines = [l.strip() for l in f]

    tiles = parse_data(lines)

    edges = tile_to_edges(tiles)

    adjacent = adjacent_tiles(edges)

    tile_positions = reassemble(adjacent)

    print("Part 1:" tile_positions[0][0] * tile_positions[0][-1] * tile_positions[-1][0] * tile_positions[-1][-1])

    stiched_image = stich(tiles, tile_positions)

    tile_width = stiched_image.shape[-1]

    stiched_image = np.concatenate(np.concatenate(stiched_image[:,:,1:-1,1:-1], axis=1), axis=1)

    sea_monster = np.array([
        list("                  # "),
        list("#    ##    ##    ###"),
        list(" #  #  #  #  #  #   "),
    ])

    for stiched_image, _, _, _ in get_all_transformations(stiched_image):
        for i in range(stiched_image.shape[0]):
            for j in range(stiched_image.shape[1]):
                search_region = stiched_image[i:i+sea_monster.shape[0], j:j+sea_monster.shape[1]]
                if sea_monster.shape == search_region.shape:
                    monster_mask = sea_monster == "#"
                    region_mask = np.logical_or(search_region == "#", search_region == "O")

                    if np.logical_and(monster_mask, region_mask).sum() == monster_mask.sum():
                        search_region[monster_mask] = "O"

        print("Part 2:",np.logical_or(stiched_image == "O", stiched_image == "#").sum() - (stiched_image == "O").sum())