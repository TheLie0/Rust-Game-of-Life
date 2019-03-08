extern crate console;
extern crate rand;

use console::Term;

fn main() {
    let cutoff: f32 = 0.3;
    let term = Term::stdout();

    let mut game = GameOfLife::new(255, 255, cutoff);
    game.print_to_console(&term);
    loop {
        game.update();
        if game.print_to_console(&term) == 0 {
            game = GameOfLife::new(255, 255, cutoff);
        }
    }
}

struct GameOfLife {
    x: usize,
    y: usize,
    playing_field: Vec<Vec<u16>>,
    /*The lest significant bit of the u16 represents the current state of the cell.
    The 15 bits to the left represent the 15 past states. This is used to calculate
    whether there is still interesting movement in the visible playing field.*/
}

impl GameOfLife {
    pub fn new(x: usize, y: usize, cutoff: f32) -> Self {
        let mut playing_field = vec![vec![0; y]; x];

        for i in 0..x {
            for j in 0..y {
                playing_field[i][j] = random_one_or_zero_with_cutoff_point(cutoff)
            }
        }

        Self { x, y, playing_field }
    }

    pub fn print_to_console(&self, term: &Term) -> usize {
        let (x, y) = term.size();
        term.clear_last_lines(x as usize);
        let upper_border = (self.x - x as usize) / 2;
        let left_border = (self.y - y as usize) / 2;

        let mut movement_score: usize = 0;

        for x_iter in upper_border..(self.x - upper_border) {
            for y_iter in left_border..(self.y - left_border) {
                if self.playing_field[x_iter][y_iter] & 1 == 1 {
                    print!("#");
                } else {
                    print!(" ");
                };
                movement_score += get_movement_score(self.playing_field[x_iter][y_iter]);
            }
            print!("\n");
        }

        return movement_score;
    }

    pub fn update(&mut self) {
        let mut new_field = vec![vec![0; self.y]; self.x];

        for x_iter in 1..self.x - 1 {
            for y_iter in 1..self.y - 1 {
                let mut neighbor_count: u8 = 0;

                for n_x_iter in (x_iter - 1)..(x_iter + 2) {
                    for n_y_iter in (y_iter - 1)..(y_iter + 2) {
                        if (n_x_iter, n_y_iter) != (x_iter, y_iter)
                            && self.playing_field[n_x_iter][n_y_iter] & 1 == 1
                        {
                            neighbor_count += 1;
                        }
                    }
                }

                match neighbor_count {
                    3 => {
                        new_field[x_iter][y_iter] =
                            self.playing_field[x_iter][y_iter] << 1 | 1;
                    }
                    2 => {
                        new_field[x_iter][y_iter] =
                            self.playing_field[x_iter][y_iter] << 1 | (
                                self.playing_field[x_iter][y_iter] & 1
                            )
                    }
                    _ => {
                        new_field[x_iter][y_iter] =
                            self.playing_field[x_iter][y_iter] << 1;
                    }
                }
            }
        }

        self.playing_field = new_field
    }
}

fn random_one_or_zero_with_cutoff_point(cutoff: f32) -> u16 {
    let rand_num: f32 = rand::random();
    if rand_num < cutoff {
        return 1;
    }
    return 0;
}

fn get_movement_score(cell: u16) -> usize {
    if (cell & 0b0000000001111110) == ((cell & 0b0001111110000000) >> 6) {
        return 0;
    }
    return 1;
}
