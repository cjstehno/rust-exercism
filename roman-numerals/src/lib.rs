
pub struct Roman {
    value: u16
}

impl Roman {
    pub fn from( number: u16 ) -> Roman {
        return Roman { value: number }
    }

    pub fn to_string(&self) -> String {
        let val_str = self.value.to_string();
        let mut rev_chars = val_str.chars().rev();

        let ones = match rev_chars.next() {
            Some(ch) => match ch {
                '0' => "",
                '1' => "I",
                '2' => "II",
                '3' => "III",
                '4' => "IV",
                '5' => "V",
                '6' => "VI",
                '7' => "VII",
                '8' => "VIII",
                '9' => "IX",
                _   => panic!("Unexpected ones value!")
            },
            None => panic!("Unexpected ones value!")
        };

        let tens = match rev_chars.next() {
            Some(ch) => match ch {
                '0' => "",
                '1' => "X",
                '2' => "XX",
                '3' => "XXX",
                '4' => "XL",
                '5' => "L",
                '6' => "LX",
                '7' => "LXX",
                '8' => "LXXX",
                '9' => "XC",
                _   => ""
            },
            None     => ""
        };

        let hundreds = match rev_chars.next() {
            Some(ch) => match ch {
                '0' => "",
                '1' => "C",
                '2' => "CC",
                '3' => "CCC",
                '4' => "CD",
                '5' => "D",
                '6' => "DC",
                '7' => "DCC",
                '8' => "DCCC",
                '9' => "CM",
                _   => ""
            },
            None     => ""
        };

        let thousands = match rev_chars.next() {
            Some(ch) => match ch {
                '0' => "",
                '1' => "M",
                '2' => "MM",
                '3' => "MMM",
                // we dont support more than 3k
                _   => ""
            },
            None     => ""
        };

        return format!("{}{}{}{}", thousands, hundreds, tens, ones);
    }
}
