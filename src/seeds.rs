use super::grid;
use rand::Rng;

pub fn random_seed() -> Vec<Vec<grid::Cell>> {
    let dimensions = rand::thread_rng().gen_range(60, 61);
    let mut seed = Vec::new();
    for x in 0..dimensions {
        let mut column = Vec::new();
        for y in 0..dimensions {
            let cell = grid::Cell::new(x, y, rand::thread_rng().gen_range(0, 2) >= 1);
            column.push(cell)
        }
        seed.push(column);
    }
    seed
}

mod test {
    use super::*;

    #[test]
    fn non_random_random_seed() {
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