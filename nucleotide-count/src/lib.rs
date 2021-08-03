use std::collections::HashMap;

const NUCLEOTIDE: [char; 4] = ['A', 'T', 'C', 'G'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?.get(&nucleotide).map(|c| *c).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    Ok(dna.chars().fold(Ok(NUCLEOTIDE.iter().map(|c| (*c, 0)).collect::<HashMap<char, usize>>()),
                        |hashmap_result, c| {
                            hashmap_result.and_then(|mut hashmap| {
                                if NUCLEOTIDE.contains(&c) {
                                    *hashmap.entry(c).or_default() += 1;
                                    Ok(hashmap)
                                } else {
                                    Err(c)
                                }
                            })
                        })?)
}