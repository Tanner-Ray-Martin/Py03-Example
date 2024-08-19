use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*; // For parallelism

#[pyfunction]
fn get_grids(grid: Vec<usize>, _cg: Vec<u8>, rz: usize, cz: usize) -> (Vec<usize>, Vec<u8>) {
    // Use parallel iterator to calculate new grid and color grid values
    let results: Vec<(usize, [u8; 3])> = (0..rz * cz)
        .into_par_iter()
        .map(|idx| {
            let r = idx / cz;
            let c = idx % cz;
            let v = grid[idx];
            let mut red = 0;
            let mut blue = 0;

            let r_min = if r > 0 { r - 1 } else { 0 };
            let r_max = if r + 1 < rz { r + 2 } else { rz };
            let c_min = if c > 0 { c - 1 } else { 0 };
            let c_max = if c + 1 < cz { c + 2 } else { cz };

            for i in r_min..r_max {
                for j in c_min..c_max {
                    let neighbor_idx = i * cz + j;
                    red += grid[neighbor_idx];
                    if (i == r_min || i == r_max - 1) && (j == c_min || j == c_max - 1) {
                        blue += grid[neighbor_idx];
                    }
                }
            }
            let green = red - blue;

            let new_v = if v == 0 && (red - v) == 3 {
                1
            } else if v == 1 && ((red - v) < 2 || (red - v) > 3) {
                0
            } else {
                v
            };

            (new_v, [(red * 28 * v) as u8, (green * 63) as u8, (blue * 63) as u8])
        })
        .collect();

    // Split the results into separate grid and color grid vectors
    let new_grid: Vec<usize> = results.iter().map(|(new_v, _)| *new_v).collect();
    let new_cg: Vec<u8> = results.iter().flat_map(|(_, colors)| colors.iter().copied()).collect();

    (new_grid, new_cg)
}
#[pymodule]
fn Py03_Example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_grids, m)?)?;
    Ok(())
}

