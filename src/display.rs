use crate::{CellState, LifeGame};

pub trait DisplayInCui {
    fn display(&self);
}

impl DisplayInCui for LifeGame {
    fn display(&self) {
        let field_as_string = (0..self.height as i32)
            .map(|y| {
                format!(
                    "{}\n",
                    (0..self.width as i32)
                        .map(|x| match self.get_cell_state(x, y) {
                            CellState::Alive => "■",
                            CellState::Dead => "□",
                        })
                        .collect::<String>()
                )
            })
            .collect::<String>();

        println!("{}", field_as_string);
    }
}
