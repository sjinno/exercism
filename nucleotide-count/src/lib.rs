use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_valid_dna(dna)?;
    is_valid_nucleotide(nucleotide)?;

    let mut count = 0;
    dna.chars().into_iter().for_each(|ncl| {
        if ncl == nucleotide {
            count += 1;
        }
    });
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    is_valid_dna(dna)?;
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
fn is_valid_dna(dna: &str) -> Result<(), char> {
    for ncl in dna.chars() {
        if !VALID_NUCLEOTIDES.iter().any(|valid| valid == &ncl) {
            return Err(ncl);
        }
    }
    Ok(())
}

// Check if target nucleotide is valid.
fn is_valid_nucleotide(nucleotide: char) -> Result<(), char> {
    if !VALID_NUCLEOTIDES.iter().any(|valid| valid == &nucleotide) {
        return Err(nucleotide);
    }
    Ok(())
}

// Community solution 1
// use std::collections::HashMap;

// pub fn count(c: char, dna: &str) -> usize {
//     dna.chars().filter(|&x| x == c).count()
// }

// pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {
//     "ACGT".chars().map(|c| (c, count(c, dna))).collect()
// }

// // Community solution 2
// use std::collections::HashMap;

// pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
//     let nucleotide = dna
//         .chars()
//         .find(|c| c != &'A' && c != &'C' && c != &'G' && c != &'T')
//         .unwrap_or(nucleotide);

//     match nucleotide {
//         'A' | 'C' | 'G' | 'T' => Ok(dna.chars().filter(|c| c == &nucleotide).count()),
//         x => Err(x),
//     }
// }

// pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
//     let mut result = HashMap::new();

//     result.insert('A', count('A', dna)?);
//     result.insert('C', count('C', dna)?);
//     result.insert('G', count('G', dna)?);
//     result.insert('T', count('T', dna)?);

//     Ok(result)
// }

// // Community solution 3
// use std::collections::HashMap;

// const VALID: [char; 4] = ['A', 'C', 'G', 'T'];

// pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
//     if !VALID.contains(&nucleotide) {
//         return Err(nucleotide);
//     }

//     let mut count = 0;

//     for c in dna.chars() {
//         if !VALID.contains(&c) {
//             return Err(c);
//         } else if c == nucleotide {
//             count += 1;
//         }
//     }

//     Ok(count)
// }

// pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
//     let mut count = [0, 0, 0, 0];

//     for c in dna.chars() {
//         let index = match c {
//             'A' => 0,
//             'C' => 1,
//             'G' => 2,
//             'T' => 3,
//             _ => return Err(c),
//         };
//         count[index] += 1;
//     }

//     Ok(VALID.iter().cloned().zip(count.iter().cloned()).collect())
// }

// // CS 4
// // Not the greatest solution but sitll there's something to learng from it.
// use std::collections::HashMap;
// use std::iter::once;

// pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
//     if !dna
//         .chars()
//         .chain(once(nucleotide))
//         .all(|c| "ACGT".contains(c))
//     {
//         return Err('X');
//     } else {
//         return Ok(dna.chars().filter(|&c| c == nucleotide).count());
//     }
// }

// pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
//     return "ACGT".chars().try_fold(HashMap::new(), |mut map, c| {
//         map.insert(c, count(c, dna)?);
//         return Ok(map);
//     });
// }
