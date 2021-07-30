use crate::{LifeGame, State};

pub trait DisplayInCli {
    fn display(&self);
}

impl DisplayInCli for LifeGame {
    fn display(&self) {
        let field_as_string = (0..self.height as i32)
            .map(|y| {
                format!(
                    "{}\n",
                    (0..self.width as i32)
                        .map(|x| match self.get_cell_state(x, y) {
                            State::Alive => "■",
                            State::Dead => "□",
                        })
                        .collect::<String>()
                )
            })
            .collect::<String>();

        println!("{}", field_as_string);
    }
}
