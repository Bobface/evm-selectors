use anyhow::Result;

/// Describes a selector, which can either be:
///     - 4 bytes for functions, errors, etc.
///     - 32 bytes for events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Selector {
    Four([u8; 4]),
    ThirtyTwo([u8; 32]),
}

impl From<[u8; 4]> for Selector {
    fn from(bytes: [u8; 4]) -> Self {
        Self::Four(bytes)
    }
}

impl From<[u8; 32]> for Selector {
    fn from(bytes: [u8; 32]) -> Self {
        Self::ThirtyTwo(bytes)
    }
}

impl TryFrom<&[u8]> for Selector {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Ok(match bytes.len() {
            4 => Selector::Four(bytes.try_into().unwrap()),
            32 => Selector::ThirtyTwo(bytes.try_into().unwrap()),
            _ => {
                return Err(anyhow::anyhow!(
                    "Selector has invalid byte length: {}",
                    bytes.len()
                ));
            }
        })
    }
}
