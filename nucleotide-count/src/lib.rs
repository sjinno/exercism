use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    validate_dna(dna)?;
    validate_nucleotide(nucleotide)?;

    let mut count = 0;
    dna.chars().into_iter().for_each(|ncl| {
        if ncl == nucleotide {
            count += 1;
        }
    });
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    validate_dna(dna)?;
    let mut hm = HashMap::<char, usize>::new();
    // Initialize hm with default value being 0:
    // hm = { 'A': 0, 'C': 0, 'G', 0, 'T': 0 }
    VALID_NUCLEOTIDES
        .iter()
        .for_each(|ncl| *hm.entry(*ncl).or_default() = 0);
    dna.chars()
        .into_iter()
        .for_each(|ncl| *hm.entry(ncl).or_default() += 1);
    Ok(hm)
}

// Check if dna contains all valid nucleotides.
fn validate_dna(dna: &str) -> Result<(), char> {
    for ncl in dna.chars() {
        if !VALID_NUCLEOTIDES.iter().any(|valid| valid == &ncl) {
            return Err(ncl);
        }
    }
    Ok(())
}

// Check if target nucleotide is valid.
fn validate_nucleotide(nucleotide: char) -> Result<(), char> {
    if !VALID_NUCLEOTIDES.iter().any(|valid| valid == &nucleotide) {
        return Err(nucleotide);
    }
    Ok(())
}
