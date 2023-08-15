use super::*;

#[derive(Clone, PartialEq)]
pub struct Knight;

const WHITE_SYMBOL: &str = "♞";
const BLACK_SYMBOL: &str = "♘";

impl Eq for Knight {}

impl Piece for Knight {
    fn get_piece_symbol(color: &Color) -> &'static str {
        match color {
            Color::White => WHITE_SYMBOL,
            Color::Black => BLACK_SYMBOL,
        }
    }

    fn get_moveset(square: &Square) -> Vec<Move> {
        let mut moveset: Vec<Move> = Vec::new();
        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::North.unit_square()
                + Direction::North.unit_square()
                + Direction::West.unit_square(),
        ));
        println!("1. {}", moveset[moveset.len() - 1]);

        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::North.unit_square()
                + Direction::North.unit_square()
                + Direction::East.unit_square(),
        ));
        println!("2. {}", moveset[moveset.len() - 1]);

        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::South.unit_square()
                + Direction::South.unit_square()
                + Direction::West.unit_square(),
        ));
        println!("3. {}", moveset[moveset.len() - 1]);

        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::South.unit_square()
                + Direction::South.unit_square()
                + Direction::East.unit_square(),
        ));
        println!("4. {}", moveset[moveset.len() - 1]);

        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::East.unit_square()
                + Direction::East.unit_square()
                + Direction::North.unit_square(),
        ));
        println!("5. {}", moveset[moveset.len() - 1]);

        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::East.unit_square()
                + Direction::East.unit_square()
                + Direction::South.unit_square(),
        ));
        println!("6. {}", moveset[moveset.len() - 1]);

        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::West.unit_square()
                + Direction::West.unit_square()
                + Direction::North.unit_square(),
        ));
        println!("7. {}", moveset[moveset.len() - 1]);

        moveset.push(Move::new(
            square.clone(),
            square.clone()
                + Direction::West.unit_square()
                + Direction::West.unit_square()
                + Direction::South.unit_square(),
        ));
        println!("8. {}", moveset[moveset.len() - 1]);

        let output = moveset
            .into_iter()
            .filter(|mv| {
                mv.start.rank < 8
                    && mv.start.file < 8
                    && mv.dest.rank < 8
                    && mv.dest.file < 8
                    && mv.start != mv.dest
            })
            .collect();

        println!("Knight moves for {}:", square);
        for mv in &output {
            println!("{}", mv);
        }

        output
    }
}
