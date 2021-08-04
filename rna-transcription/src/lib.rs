const DNA: [char; 4] = ['A', 'T', 'C', 'G'];
const RNA: [char; 4] = ['U', 'A', 'G', 'C'];

#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: String,
}

fn is_valid(s: &str, standard: &[char; 4]) -> Result<(), usize> {
    for (i, c) in s.chars().enumerate() {
        if !standard.contains(&c) {
            return Err(i);
        }
    }

    Ok(())
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        is_valid(dna, &DNA)?;
        Ok(Self {
            nucleotides: dna.to_string()
        })
    }

    pub fn into_rna(self) -> Rna {
        let s = self.nucleotides.chars().map(|c| {
            let pos = DNA.iter().position(|dc| *dc == c).unwrap();
            RNA[pos]
        }).collect::<String>();
        Rna::new(&s).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        is_valid(rna, &RNA)?;
        Ok(Self {
            nucleotides: rna.to_string()
        })
    }
}
