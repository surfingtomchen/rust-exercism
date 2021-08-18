use std::collections::HashMap;
use std::str;

pub struct CodonsInfo<'a> {
    codon_mapping: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        self.codon_mapping
            .get(codon)
            .cloned()
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        let stop = "stop codon";
        rna.as_bytes()
           .chunks(3)
           .map(str::from_utf8)
           .map(|str| self.name_for(str.unwrap()))
           .take_while(|&c| c != Some(stop))
           .collect::<Option<Vec<&'a str>>>()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        codon_mapping: pairs.iter().cloned().collect(),
    }
}
