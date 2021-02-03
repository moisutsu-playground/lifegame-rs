#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct LifeGame {
    cell_field: Vec<Vec<State>>,
}

impl LifeGame {
    pub fn new(height: usize, width: usize) -> Self {
        LifeGame {
            cell_field: vec![vec![State::Dead; width]; height],
        }
    }

    pub fn next(&mut self) {
        let mut new_cell_field = self.cell_field.clone();
        for x in 0..self.cell_field[0].len() {
            for y in 0..self.cell_field.len() {
                let (x, y) = (x as i32, y as i32);
                match self.get_cell_state(x, y) {
                    State::Alive => match self.surrounding_alive_cells(x, y) {
                        2 | 3 => (),
                        _ => new_cell_field[y as usize][x as usize] = State::Dead,
                    },
                    State::Dead => match self.surrounding_alive_cells(x, y) {
                        3 => new_cell_field[y as usize][x as usize] = State::Alive,
                        _ => (),
                    },
                }
            }
        }
        self.cell_field = new_cell_field;
    }

    pub fn get_cell_state(&self, x: i32, y: i32) -> State {
        let (x, y) = self.get_real_point(x, y);
        self.cell_field[y][x]
    }

    pub fn set_cell_state(&mut self, state: State, x: i32, y: i32) {
        let (x, y) = self.get_real_point(x, y);
        self.cell_field[y][x] = state;
    }

    pub fn surrounding_alive_cells(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for move_x in -1..=1 {
            for move_y in -1..=1 {
                if move_x == 0 && move_y == 0 {
                    continue;
                }
                if self.get_cell_state(x + move_x, y + move_y) == State::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_real_point(&self, x: i32, y: i32) -> (usize, usize) {
        let x = if x >= 0 {
            x as usize % self.cell_field[0].len()
        } else {
            self.cell_field[0].len() - -x as usize
        };
        let y = if y >= 0 {
            y as usize % self.cell_field.len()
        } else {
            self.cell_field.len() - -y as usize
        };
        (x, y)
    }

    pub fn display(&self) {
        let field_as_string = self
            .cell_field
            .iter()
            .map(|line| {
                format!(
                    "{}\n",
                    line.iter()
                        .map(|&cell| match cell {
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
