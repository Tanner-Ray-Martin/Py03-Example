use pyo3::prelude::*;
use rayon::prelude::*;
use rand::Rng;

const GWIDTH: usize = 900;
const GHEIGHT: usize = 900;

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

#[pyclass]
struct ColorGridGenerator {
    grid: Vec<Vec<u8>>,
    cg: Vec<Vec<[u8; 3]>>,
}

#[pymethods]
impl ColorGridGenerator {
    #[new]
    fn new() -> Self {
        let grid = create_grids(GWIDTH, GHEIGHT);
        let cg = vec![vec![[0; 3]; GHEIGHT]; GWIDTH];
        ColorGridGenerator { grid, cg }
    }

    fn __iter__(slf: PyRef<Self>) -> Py<ColorGridIterator> {
        Py::new(slf.py(), ColorGridIterator {
            grid: slf.grid.clone(),
            cg: slf.cg.clone(),
        }).unwrap()
    }
}

#[pyclass]
struct ColorGridIterator {
    grid: Vec<Vec<u8>>,
    cg: Vec<Vec<[u8; 3]>>,
}

#[pymethods]
impl ColorGridIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Vec<Vec<[u8; 3]>>> {
        let new_grid = get_grids(&slf.grid, GWIDTH, GHEIGHT);

        // Borrow the color grid first, then update the grid
        let mut cg = std::mem::take(&mut slf.cg);
        update_color_grid(&new_grid, &mut cg, GWIDTH, GHEIGHT);
        slf.grid = new_grid;
        slf.cg = cg;

        Some(slf.cg.clone())
    }
}

#[pymodule]
fn py03_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ColorGridGenerator>()?;
    m.add_class::<ColorGridIterator>()?;
    Ok(())
}


