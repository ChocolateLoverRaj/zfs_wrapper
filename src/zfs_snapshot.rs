use std::{borrow::Cow, fmt::Display};

use crate::ZfsDataset;

#[derive(Debug, Clone)]
pub struct ZfsSnapshot<'a> {
    pub dataset: ZfsDataset<'a>,
    pub snapshot_name: Cow<'a, str>,
}

impl Display for ZfsSnapshot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            dataset,
            snapshot_name,
        } = self;
        write!(f, "{dataset}@{snapshot_name}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            ZfsSnapshot {
                dataset: ZfsDataset {
                    zpool: "my-zpool".into(),
                    dataset: "my-snapshot".into()
                },
                snapshot_name: "snap0".into()
            }
            .to_string(),
            "my-zpool/my-dataset@snap0"
        );
    }
}
