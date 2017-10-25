
pub struct Triangle {
    sides: [u16; 3]
}

impl Triangle {

    pub fn build( sides: [u16; 3] ) -> Result<Triangle,()> {
        if Triangle::sides_non_zero(sides) && Triangle::sides_sum(sides) {
            Ok(Triangle { sides: sides })

        } else {
            Err(())
        }
    }

    fn sides_non_zero(sides: [u16; 3]) -> bool {
        sides[0] > 0 && sides[1] > 0 && sides[2] > 0
    }

    fn sides_sum(sides: [u16; 3]) -> bool {
        sides[0] + sides[1] >= sides[2] && sides[0] + sides[2] >= sides[1] && sides[1] + sides[2] >= sides[0]
    }

    pub fn is_equilateral(&self) -> bool {
        Triangle::count_equal_sides(self) == 3
    }

    pub fn is_scalene(&self) -> bool {
        Triangle::count_equal_sides(self) == 0
    }

    pub fn is_isosceles(&self) -> bool {
        Triangle::count_equal_sides(self) >= 1
    }

    fn count_equal_sides(&self) -> u16 {
        let mut count: u16 = 0;

        if self.sides[0] == self.sides[1] {
            count += 1;
        }

        if self.sides[0] == self.sides[2] {
            count += 1;
        }

        if self.sides[1] == self.sides[2] {
            count += 1;
        }

        count
    }
}
