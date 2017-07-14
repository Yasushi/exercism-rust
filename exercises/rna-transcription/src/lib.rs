#[derive(PartialEq, Debug)]
pub struct RibonucleicAcid(String);

impl RibonucleicAcid {
    pub fn new(s: &str) -> RibonucleicAcid {
        RibonucleicAcid(s.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub struct DeoxyribonucleicAcid(String);

impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid(s.to_string())
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, ()> {
        self.0
            .chars()
            .map(|c| match c {
                'A' => Some('U'),
                'T' => Some('A'),
                'C' => Some('G'),
                'G' => Some('C'),
                _ => None,
            })
            .collect::<Option<String>>()
            .map(|rna| RibonucleicAcid::new(&rna))
            .ok_or(())
    }
}
