import py03_example  # This is the Rust module
import time
import pygame
import numpy as np
from numba import njit, prange

pygame.init()

ww, wh = 900, 900
window = pygame.display.set_mode((ww, wh))
pygame.display.set_caption("Conway's Game of Life")

gen = py03_example.ColorGridGenerator()

running = True


def handle_events():
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            return False
    return True


def draw_grid(grid: list[list[int]]):
    # Lock the window surface for pixel access
    pixel_array = pygame.PixelArray(window)

    for y in range(len(grid)):
        for x in range(len(grid[0])):
            pixel_array[x, y] = tuple(grid[x][y])

    # Unlock the surface
    del pixel_array
    pygame.display.flip()


t = time.time()
times_ran = 0
times_to_run = 60
gen = py03_example.ColorGridGenerator()

for cg in gen:  # Update the grid using Numba
    draw_grid(cg)  # Convert back to list for drawing if necessary
    if times_ran == times_to_run or not handle_events():
        break
    times_ran += 1

print(time.time() - t)
pygame.quit()
