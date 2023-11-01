pub struct Triangle {
    sides: [u64; 3],
}

fn is_valid(sides: &[u64; 3]) -> bool {
    for &v in sides {
        if v == 0 {
            return false;
        }
    }
    if sides[0] + sides[1] <= sides[2]
        || sides[1] + sides[2] <= sides[0]
        || sides[2] + sides[0] <= sides[1]
    {
        return false;
    }
    true
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if is_valid(&sides) {
            Some(Self { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let key = self.sides[0];
        for v in self.sides {
            if v != key {
                return false;
            }
        }
        true
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        return self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2];
    }
}
