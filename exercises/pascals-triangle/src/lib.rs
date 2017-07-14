pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.0 == 0 {
            return Vec::new();
        }
        let mut result = vec![vec![1]];
        for _ in 1..self.0 {
            let mut prev = result.last().unwrap().clone();
            prev.push(0);
            let mut rev = prev.clone();
            rev.reverse();

            result.push(prev.iter().zip(rev.iter()).map(|(a, b)| a + b).collect());
        }
        result
    }
}
