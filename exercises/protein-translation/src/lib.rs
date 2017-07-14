use std::collections::HashMap;

#[allow(dead_code)]
pub struct Info<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> Info<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&'a str, ()> {
        if let Some(prot) = self.pairs.get(codon) {
            Ok(prot)
        } else {
            Err(())
        }
    }

    pub fn of_rna(&self, rna: &str) -> Result<Vec<&'a str>, ()> {
        rna.chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|codon| {
                self.name_for(&codon.into_iter().collect::<String>())
            })
            .take_while(|prot| prot.map(|p| p != "stop codon").unwrap_or(true))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> Info {
    Info { pairs: pairs.into_iter().collect() }
}
