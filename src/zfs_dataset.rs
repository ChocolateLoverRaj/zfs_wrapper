use std::fmt::Display;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ZfsDataset {
    pub zpool: String,
    pub dataset: String,
}

impl Display for ZfsDataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { zpool, dataset } = self;
        write!(f, "{zpool}/{dataset}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            ZfsDataset {
                zpool: "my-zpool".into(),
                dataset: "my-dataset".into(),
            }
            .to_string(),
            "my-zpool/my-dataset"
        );
    }
}
