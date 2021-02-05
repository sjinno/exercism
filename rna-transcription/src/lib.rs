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
        let mut map = HashSet::<char>::new();
        dna.chars().into_iter().for_each(|n| {
            if DNA.contains(&n) {
                map.insert(n);
            }
        });

        let valid_count = map.len();
        if dna.chars().all(|n| DNA.contains(&n)) {
            Ok(Dna {
                dna: dna.to_string(),
            })
        } else {
            Err(valid_count)
        }
    }

    pub fn into_rna(self) -> Rna {
        let mut rna = Vec::with_capacity(self.dna.len());
        self.dna.chars().into_iter().for_each(|n| match n {
            'G' => rna.push('C'),
            'C' => rna.push('G'),
            'T' => rna.push('A'),
            'A' => rna.push('U'),
            _ => (),
        });
        Rna {
            rna: rna.iter().collect::<String>(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut map = HashSet::<char>::new();
        rna.chars().into_iter().for_each(|n| {
            if RNA.contains(&n) {
                map.insert(n);
            }
        });

        let valid_count = map.len();
        if rna.chars().all(|n| RNA.contains(&n)) {
            Ok(Rna {
                rna: rna.to_string(),
            })
        } else {
            Err(valid_count)
        }
    }
}
