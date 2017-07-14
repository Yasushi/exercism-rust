#[derive(PartialEq, Debug)]
struct Frame {
    throws: Vec<u8>,
}

impl Frame {
    fn new() -> Self {
        Self { throws: Vec::new() }
    }

    fn is_open(&self) -> bool {

    }
}

#[derive(Debug)]
pub struct BowlingGame {}

impl BowlingGame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn roll(&mut self, n: u8) -> Result<(), ()> {
        Ok(())
    }

    pub fn score(&self) -> Result<u16, ()> {
        Ok(0)
    }
}
