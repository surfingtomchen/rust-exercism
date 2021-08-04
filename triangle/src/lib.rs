pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0 || sides[1] == 0 || sides[2] == 0 {
            return None;
        }

        if sides[0] + sides[1] > sides[2]
            && sides[1] + sides[2] > sides[0]
            && sides[0] + sides[2] > sides[1]
        {
            Some(Self { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }
}
