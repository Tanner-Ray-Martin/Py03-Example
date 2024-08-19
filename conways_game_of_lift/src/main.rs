use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;
use std::time::Instant;
use rand::Rng;
const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const GWIDTH: usize = 800;
const GHEIGHT: usize = 800;

fn create_grids(rz: usize, cz: usize) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![0; cz]; rz];
    let mut rng = rand::thread_rng();

    for r in 0..rz {
        for c in 0..cz {
            grid[r][c] = rng.gen_range(0..=1);
        }

    }

    grid
}

fn get_neighbors(grid: &Vec<Vec<u8>>, r: usize, c: usize, rz: usize, cz: usize) -> (u8, u8) {
    let mut count = 0;
    let v = grid[r][c];

    let r_min = r.saturating_sub(1);
    let r_max = (r + 2).min(rz);
    let c_min = c.saturating_sub(1);
    let c_max = (c + 2).min(cz);

    for i in r_min..r_max {
        for j in c_min..c_max {
            if i != r || j != c {
                count += grid[i][j];
            }
        }
    }

    (count, v)
}

fn get_new_value(v: u8, ln: u8) -> u8 {
    if v == 0 && ln == 3 {
        1
    } else if v == 1 && (ln < 2 || ln > 3) {
        0
    } else {
        v
    }
}

fn get_grids(grid: &Vec<Vec<u8>>, rz: usize, cz: usize) -> Vec<Vec<u8>> {
    let new_grid: Vec<Vec<u8>> = (0..rz)
        .into_par_iter()
        .map(|r| {
            let mut row = vec![0; cz];
            for c in 0..cz {
                let (ln, v) = get_neighbors(grid, r, c, rz, cz);
                let new_v = get_new_value(v, ln);
                row[c] = new_v;
            }
            row
        })
        .collect();

    new_grid
}

fn update_color_grid(grid: &Vec<Vec<u8>>, cg: &mut Vec<Vec<[u8; 3]>>, rz: usize, cz: usize) {
    for r in 0..rz {
        for c in 0..cz {
            if grid[r][c] == 1 {
                cg[r][c] = [255, 255, 255]; // White color for live cells
            } else {
                cg[r][c] = [0, 0, 0]; // Black color for dead cells
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut grid = create_grids(GWIDTH, GHEIGHT);
    let mut cg = vec![vec![[0; 3]; GHEIGHT]; GWIDTH];
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let start = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        grid = get_grids(&grid, GWIDTH, GHEIGHT);
        update_color_grid(&grid, &mut cg, GWIDTH, GHEIGHT);

        for y in 0..GHEIGHT {
            for x in 0..GWIDTH {
                let index = y * WIDTH + x;

                if index < buffer.len() {
                    let color = cg[y][x];
                    buffer[index] = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | (color[2] as u32);
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // std::thread::sleep(std::time::Duration::from_millis(1000 / FPS));
    }

    println!("Time taken: {:?}", start.elapsed());
}
