use std::collections::HashSet;

const DNA: &[char; 4] = &['G', 'C', 'T', 'A'];
const RNA: &[char; 4] = &['C', 'G', 'A', 'U'];

#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if dna.chars().all(|n| DNA.contains(&n)) {
            return Ok(Dna {
                dna: dna.to_string(),
            });
        }

        Err(dna
            .chars()
            .filter(|n| DNA.contains(n))
            .collect::<HashSet<char>>()
            .len())
    }

    pub fn into_rna(self) -> Rna {
        self.into()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if rna.chars().all(|n| RNA.contains(&n)) {
            return Ok(Rna {
                rna: rna.to_string(),
            });
        }

        Err(rna
            .chars()
            .filter(|n| RNA.contains(n))
            .collect::<HashSet<_>>()
            .len())
    }
}

impl Into<Rna> for Dna {
    fn into(self) -> Rna {
        Rna {
            rna: self.dna.chars().fold(String::new(), |mut acc, n| {
                match n {
                    'G' => acc.push('C'),
                    'C' => acc.push('G'),
                    'T' => acc.push('A'),
                    'A' => acc.push('U'),
                    _ => (),
                }
                acc
            }),
        }
    }
}
