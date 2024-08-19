import numpy as np
from numba import njit, prange
import time
import random
import pygame
from pygame.time import Clock

file_type: str = "mp4v"
file_name: str = "conways_game_of_life.mp4"
fps: int = 30
rz, cz = 640, 480  # row size and column size of the grid

pygame.init()


clock = Clock()
# Example usage
rz, cz = 900, 700
ww, wh = rz, cz


def create_window(ww, wh):
    return pygame.display.set_mode((ww, wh))


def create_grids(rz, cz, ww, wh):
    grid = np.zeros((rz, cz), dtype=int)
    cg = np.zeros((ww, wh, 3), dtype=np.uint8)
    # for the inital state of the game, we set a ton of the pixels to alive at random
    for i in range(int((rz * cz) * 0.95)):
        grid[random.randint(0, rz - 1), random.randint(0, cz - 1)] = random.randint(
            0, 1
        )
        cg[random.randint(0, ww - 1), random.randint(0, wh - 1)] = [
            random.randint(0, 255),
            random.randint(0, 255),
            random.randint(0, 255),
        ]
    return grid, cg


@njit
def get_neighbors(grid, r, c, rz, cz):
    r_min, r_max = max(0, r - 1), min(rz, r + 2)
    c_min, c_max = max(0, c - 1), min(cz, c + 2)
    v = grid[r, c]
    red, blue, green = 0, 0, 0
    for i in range(r_min, r_max):
        for j in range(c_min, c_max):
            red += grid[i, j]
            if (i == r_min or i == r_max - 1) and (j == c_min or j == c_max - 1):
                blue += grid[i, j]
    green = red - blue - v
    return red * 28 * v, green * 63, blue * 63, red - v, v


@njit
def get_new_value(v, ln):
    return 1 if v == 0 and ln == 3 else (0 if (v == 1 and (ln < 2 or ln > 3)) else v)


@njit(parallel=True)
def get_grids(grid, cg, rz, cz):
    new_grid = np.empty_like(grid)
    new_cg = np.empty_like(cg)
    for r in prange(rz):
        for c in prange(cz):
            red, green, blue, ln, v = get_neighbors(grid, r, c, rz, cz)
            new_v = get_new_value(v, ln)
            new_grid[r, c] = new_v
            new_cg[r, c] = [red, green, blue]

    return new_grid, new_cg


running = True


def handle_events():
    for event in pygame.event.get():
        if event.type == pygame.KEYDOWN:
            return False
    return True


def update_window(window, cg):
    pygame.surfarray.blit_array(window, cg)
    pygame.display.flip()


def delay(fps):
    clock.tick(fps)


# create the grid and the corresponding color grid
grid, cg = create_grids(rz, cz, ww, wh)
window = create_window(ww, wh)

t = time.time()
times_ran = 0
times_to_run = 200
while running:
    # not used right now, but will do in the future.
    grid, cg = get_grids(grid, cg, rz, cz)
    running = handle_events()
    update_window(window, cg)
    times_ran += 1
    if times_ran == times_to_run:
        running = False

pygame.quit()
print("Time taken: ", time.time() - t)
