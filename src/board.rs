pub struct Board {
    data: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize
}

impl Board {
    pub fn new(lines: &Vec<String>) -> Board {
        let data: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
        let width = lines[0].len();
        let height = lines.len();
        return Board {
            data, width, height
        }
    }

    pub fn get(&self, x: i64, y: i64) -> Option<char> {
        if x < 0 || x >= self.width as i64 || y < 0 || y >= self.height as i64 {
            None
        } else {
            Some(self.data[y as usize][x as usize])
        }
    }

    pub fn set(&mut self, x: i64, y: i64, c: char) {
        self.data[y as usize][x as usize] = c;
    }
}
