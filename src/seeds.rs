use super::grid;
use rand::Rng;

/// generates a random seed to be used to generate the first grid.
/// it does howver hardcodes the dimensions, although it is not a technical issue, only an esthetic one.
pub fn random_seed() -> Vec<Vec<grid::Cell>> {
    let dimensions_x = 50;
    let dimensions_y = 100;
    let mut seed = Vec::new();
    for x in 0..dimensions_x {
        let mut column = Vec::new();
        for y in 0..dimensions_y {
            let cell = grid::Cell::new(x, y, rand::thread_rng().gen_range(0, 2) >= 1);
            column.push(cell)
        }
        seed.push(column);
    }
    seed
}

mod test {
    use super::grid;
    #[test]
    fn non_random_random_seed() {
        // actually just duplicating the code in a non-random version.
        let dimensions = 3;
        let mut seed = Vec::new();
        for x in 0..dimensions {
            let mut column = Vec::new();
            for y in 0..dimensions {
                let cell = grid::Cell::new(x, y, false);
                column.push(cell)
            }
            seed.push(column);
        };
        assert_eq!(seed[0][0].alive, false);
        assert_eq!(seed[2][2].alive, false);
    }
} 