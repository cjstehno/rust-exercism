extern crate core;

use core::cmp::PartialEq;

#[derive(Debug)]
pub struct RibonucleicAcid {
    nucleotides: String
}

impl RibonucleicAcid {
    pub fn new( nucleotides: &str ) -> RibonucleicAcid {
        return RibonucleicAcid { nucleotides: nucleotides.to_string() };
    }
}

impl PartialEq for RibonucleicAcid {
    fn eq(&self, other: &RibonucleicAcid) -> bool {
        self.nucleotides == other.nucleotides
    }
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    nucleotides: String
}

impl DeoxyribonucleicAcid {

    pub fn new( nucleotides: &str ) -> DeoxyribonucleicAcid {
        return DeoxyribonucleicAcid { nucleotides: nucleotides.to_string() };
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let transcribed: String = self.nucleotides.chars().map(|n| match n {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _   => panic!("Invalid nucleotides")
        }).into_iter().collect();

        return RibonucleicAcid::new(&transcribed);
    }
}
