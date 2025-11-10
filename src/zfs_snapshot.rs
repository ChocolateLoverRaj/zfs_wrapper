use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct ZfsSnapshot<'a> {
    pub zpool: &'a str,
    pub dataset: &'a str,
    pub snapshot_name: &'a str,
}

impl Display for ZfsSnapshot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            zpool,
            dataset,
            snapshot_name,
        } = self;
        write!(f, "{zpool}/{dataset}@{snapshot_name}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            ZfsSnapshot {
                zpool: "my-zpool",
                dataset: "my-dataset",
                snapshot_name: "snap0"
            }
            .to_string(),
            "my-zpool/my-dataset@snap0"
        );
    }
}
