
pub struct Triangle {
    sides: [u16; 3]
}

impl Triangle {

    pub fn build( sides: [u16; 3] ) -> Result<Triangle,()> {
        if Triangle::sides_valid(sides) {
            Ok(Triangle { sides: sides })

        } else {
            Err(())
        }
    }

    fn sides_valid(sides: [u16; 3]) -> bool {
        return sides.iter().all(|s| s > &0) && Triangle::sides_sum(sides);
    }

    fn sides_sum(sides: [u16; 3]) -> bool {
        if sides[0] + sides[1] < sides[2] {
            return false;
        }

        if sides[0] + sides[2] < sides[1] {
            return false;
        }

        if sides[1] + sides[2] < sides[0] {
            return false;
        }

        return true;
    }

    pub fn is_equilateral(&self) -> bool {
        return Triangle::count_equal_sides(self) == 3;
    }

    pub fn is_scalene(&self) -> bool {
        return Triangle::count_equal_sides(self) == 0;
    }

    pub fn is_isosceles(&self) -> bool {
        return Triangle::count_equal_sides(self) >= 1;
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

        return count;
    }
}
