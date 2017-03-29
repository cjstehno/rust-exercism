use std::collections::HashMap;

pub fn count( nucleotide: char, strand: &str ) -> Result<usize,()> {
    if is_valid(nucleotide) && is_valid_strand(strand) {
        let count : usize = strand.chars().filter(|&c| nucleotide == c ).count();
        return Ok(count);

    } else {
        return Err(());
    }
}

pub fn nucleotide_counts(strand: &str) -> Result<HashMap<char,usize>,()> {
    let mut counts = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    counts.insert('T', 0);

    if is_valid_strand(strand) {
        for ch in strand.chars() {
            counts.insert(ch, count(ch, strand).unwrap());
        }

        return Ok(counts);
    } else {
        return Err(());
    }
}

fn is_valid( nuc: char ) -> bool {
    return match nuc {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false
    }
}

fn is_valid_strand( strand: &str ) -> bool {
    return strand.len() == 0 || strand.chars().all(|c| is_valid(c));
}
