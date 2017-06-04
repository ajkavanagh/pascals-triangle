extern crate time;

use time::get_time;


pub struct PascalsTriangle1 {
    rows: u32,
}

impl PascalsTriangle1 {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle1 { rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rs: Vec<Vec<u32>> = Vec::new();
        let mut current_row: Vec<u32>;

        if self.rows == 0 { return rs; }
        rs.push(vec![1]);
        if self.rows == 1 { return rs; }
        current_row = vec![1];
        for _ in 1..self.rows {
            current_row = make_next_row(&current_row);
            rs.push(current_row.clone());
        }
        rs
    }
}

pub struct PascalsTriangle2 {
    rows: u32,
}

impl PascalsTriangle2 {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle2 { rows: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.rows {
            0 => Vec::new(),
            1 => { vec![vec![1]] },
            _ => {
                (1..self.rows).fold(
                    vec![vec![1]],
                    |v, _| {
                        let x = vec![make_next_row(&v.last().unwrap())];
                        [v, x].concat()
                    })
            },
        }
    }
}

// taking Rn, make Rn+1 from it
fn make_next_row(v: &Vec<u32>) -> Vec<u32> {
    let zero = [0];
    let i1 = zero.iter().chain(v.iter());
    let i2 = v.iter().chain(zero.iter());

    // sum the two lists together.
    i1.zip(i2).map(|(a, b)| a + b).collect()
}


macro_rules! timeit {
    ($loops:expr, $code:block) => ({
        let n = $loops;
        let start = get_time();
        for _ in 0..n {
            $code
        }
        let end = get_time();
        // return the seconds
        ((end.sec - start.sec) as f64 +
         (end.nsec - start.nsec) as f64 / 1_000_000_000.0)
    })
}

fn main() {
    let sec1 = timeit!(1_000_000, {
        let p1 = PascalsTriangle1::new(10);
        p1.rows();
    });
    println!("PascalsTriangle1: {}", sec1);
    let sec2 = timeit!(1_000_000, {
        let p2 = PascalsTriangle2::new(10);
        p2.rows();
    });
    println!("PascalsTriangle2: {}", sec2);
}
