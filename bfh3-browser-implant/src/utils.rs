use tracing::warn;

#[derive(Debug)]
pub struct MultiError(pub Vec<anyhow::Error>);

impl std::fmt::Display for MultiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.len() {
            0 => {
                warn!("MultiError has no inner errors");
                writeln!(f, "No errors found")?;
            }
            1 => {
                self.0[0].fmt(f)?;
            }
            _ => {
                writeln!(f, "Multiple errors occurred:")?;
                for (i, err) in self.0.iter().enumerate() {
                    if f.alternate() {
                        writeln!(f, "Error {}: {:#}", i + 1, err)?;
                    } else {
                        writeln!(f, "Error {}: {}", i + 1, err)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl From<anyhow::Error> for MultiError {
    fn from(value: anyhow::Error) -> Self {
        Self(vec![value])
    }
}
