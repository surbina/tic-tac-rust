#[derive(PartialEq, Copy, Clone)]
pub enum CellValue {
    Cross,
    Circle,
}

pub struct TicTacToe {
    state: [[Option<CellValue>; 3]; 3],
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            state: [[None; 3]; 3],
        }
    }

    pub fn get_state(&self) -> [[Option<CellValue>; 3]; 3] {
        self.state
    }

    pub fn set_value(&mut self, value: CellValue, x: usize, y: usize) {
        // TODO: handle index out of range!
        if let None = self.get_winner() {
            self.state[x][y] = Some(value);
        }
    }

    pub fn get_winner(&self) -> Option<CellValue> {
        for i in 0usize..2usize {
            if let Some(value) = self.check_row(i) {
                return Some(value);
            }

            if let Some(value) = self.check_column(i) {
                return Some(value);
            }
        }

        if let Some(value) = self.check_diagonals() {
            return Some(value);
        }

        None
    }

    fn check_row(&self, row: usize) -> Option<CellValue> {
        // If the three elements in the row are the same, return the element
        if self.state[row][0] == self.state[row][1] && self.state[row][1] == self.state[row][2] {
            return self.state[row][0];
        }

        // Otherwise return None
        None
    }

    fn check_column(&self, column: usize) -> Option<CellValue> {
        // If the three elements in the column are the same, return the element
        if self.state[0][column] == self.state[1][column]
            && self.state[1][column] == self.state[2][column]
        {
            return self.state[0][column];
        }

        // Otherwise return None
        None
    }

    fn check_diagonals(&self) -> Option<CellValue> {
        // If the three elements in the diagonal are the same, return the element
        if self.state[0][0] == self.state[1][1]
            && self.state[1][1] == self.state[2][2]
            && self.state[1][1] != None
        {
            return self.state[1][1];
        }

        // If the three elements in the diagonal are the same, return the element
        if self.state[0][2] == self.state[1][1]
            && self.state[1][1] == self.state[2][0]
            && self.state[1][1] != None
        {
            return self.state[1][1];
        }

        // Otherwise return None
        None
    }
}
