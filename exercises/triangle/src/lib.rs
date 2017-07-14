#[derive(PartialEq, Eq)]
pub enum Shape {
    Equilateral,
    Isosceles,
    Scalene,
}

pub struct Triangle;

impl Triangle {
    pub fn build(sides: [u32; 3]) -> Result<Shape, ()> {
        let a = sides[0];
        let b = sides[1];
        let c = sides[2];
        if a + b > c && b + c > a && c + a > b {
            if a == b && b == c {
                Ok(Shape::Equilateral)
            } else if a == b || b == c || c == a {
                Ok(Shape::Isosceles)
            } else {
                Ok(Shape::Scalene)
            }
        } else {
            Err(())
        }
    }
}

impl Shape {
    pub fn is_equilateral(&self) -> bool {
        self == &Shape::Equilateral
    }

    pub fn is_isosceles(&self) -> bool {
        self == &Shape::Isosceles
    }

    pub fn is_scalene(&self) -> bool {
        self == &Shape::Scalene
    }
}
