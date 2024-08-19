import numpy as np
import pygame
from pygame.time import Clock
import Py03_Example  # This is the Rust module
import time
import random

file_type: str = "mp4v"
file_name: str = "conways_game_of_life.mp4"
fps: int = 30
rz, cz = 900, 700  # row size and column size of the grid

pygame.init()

clock = Clock()
ww, wh = rz, cz


def create_window(ww, wh):
    return pygame.display.set_mode((ww, wh))


def create_grids(rz, cz, ww, wh):
    grid = np.zeros(rz * cz, dtype=int)
    cg = np.zeros(ww * wh * 3, dtype=np.uint8)
    for i in range(int((rz * cz) * 0.95)):
        idx = random.randint(0, rz * cz - 1)
        grid[idx] = random.randint(0, 1)
        cg[idx * 3 : idx * 3 + 3] = [
            random.randint(0, 255),
            random.randint(0, 255),
            random.randint(0, 255),
        ]
    return grid, cg


def handle_events():
    for event in pygame.event.get():
        if event.type == pygame.KEYDOWN:
            return False
    return True


def update_window(window, cg):
    pygame.surfarray.blit_array(window, cg.reshape(rz, cz, 3))
    pygame.display.flip()


def delay(fps):
    clock.tick(fps)


# create the grid and the corresponding color grid
grid, cg = create_grids(rz, cz, ww, wh)
window = create_window(ww, wh)

running = True
t = time.time()
times_ran = 0
times_to_run = 200
while running:
    grid, cg = Py03_Example.get_grids(grid.tolist(), cg.tolist(), rz, cz)
    grid = np.array(grid)  # Convert back to numpy array
    cg = np.array(cg)  # Convert back to numpy array
    running = handle_events()
    update_window(window, cg)
    times_ran += 1
    if times_ran == times_to_run:
        running = False

pygame.quit()
print("Time taken: ", time.time() - t)
