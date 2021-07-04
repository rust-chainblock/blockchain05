use chrono::{SecondsFormat, Utc};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: String,
    pub proof: i32,
    pub previous_hash: String,
}

impl Block {
    pub fn new(index: usize, proof: Option<i32>, previous_hash: Option<String>) -> Self {
        let proof = match proof {
            Some(p) => p,
            None => 1,
        };

        let previous_hash = match previous_hash {
            Some(ph) => ph,
            None => String::from("0"),
        };

        Self {
            index: index,
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false),
            proof: proof,
            previous_hash: previous_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_genesis_block() {
        let genesis = Block::new(1, None, None);

        assert_eq!(genesis.index, 1);
    }

    #[test]
    fn test_proof_genesis_block() {
        let genesis = Block::new(1, None, None);

        assert_eq!(genesis.proof, 1);
    }

    #[test]
    fn test_previous_hash_genesis_block() {
        let genesis = Block::new(1, None, None);

        assert_eq!(genesis.previous_hash, String::from("0"));
    }
}