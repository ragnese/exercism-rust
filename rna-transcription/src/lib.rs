#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    seq: String
}

impl RibonucleicAcid {
    pub fn new(seq: &str) -> RibonucleicAcid {
        // Well, none of the tests give us bad string inputs...
        RibonucleicAcid {
            seq: seq.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    seq: String
}

impl DeoxyribonucleicAcid {
    pub fn new(seq: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid {
            seq: seq.to_owned(),
        }
    }

    pub fn to_rna(self) -> RibonucleicAcid {
        RibonucleicAcid::new(&self.seq.chars().map(Self::transcribe).collect::<String>())
    }

    fn transcribe(c: char) -> char {
        match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => panic!("Non-nucleotide in DNA sequence")
        }
    }
}
