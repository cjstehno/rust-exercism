pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        return PascalsTriangle { row_count: row_count };
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        return match *&self.row_count {
            0 => vec![],
            _ => (0..*&self.row_count).map(|r| PascalsTriangle::row(r)).collect::<Vec<Vec<u32>>>()
        }
    }

    fn row(row_num: u32) -> Vec<u32> {
        let mut row : Vec<u32> = vec![1];

        for i in 0..row_num {
            let item : u32 = row[i as usize];
            row.push( item * (row_num - i) / (i + 1) );
        }

        return row;
    }
}
