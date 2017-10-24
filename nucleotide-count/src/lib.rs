use std::collections::HashMap;

pub fn count( nucleotide: char, strand: &str ) -> Result<usize,()> {
    if is_valid(nucleotide) && is_valid_strand(strand) {
        Ok(strand.chars().filter(|&c| nucleotide == c ).count())
    } else {
        Err(())
    }
}

pub fn nucleotide_counts(strand: &str) -> Result<HashMap<char,usize>,()> {
    let mut counts : HashMap<char,usize> = [ ('A', 0), ('C', 0), ('G', 0), ('T', 0) ].iter().cloned().collect();

    if is_valid_strand(strand) {
        for ch in strand.chars() {
            counts.insert(ch, count(ch, strand).unwrap());
        }

        Ok(counts)
    } else {
        Err(())
    }
}

fn is_valid( nuc: char ) -> bool {
    return match nuc {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false
    }
}

fn is_valid_strand( strand: &str ) -> bool {
    strand.len() == 0 || strand.chars().all(|c| is_valid(c))
}
