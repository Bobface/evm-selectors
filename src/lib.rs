#![warn(clippy::all, clippy::pedantic, clippy::style)]

#[cfg(feature = "download")]
extern crate reqwest;

#[cfg(feature = "download")]
mod download;

mod parsing;
mod selector;
pub use selector::Selector;

use anyhow::Result;
use ethers::abi::Function;
use parsing::parse_line;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone)]
pub struct EvmSelectors {
    items: HashMap<Selector, Vec<Function>>,
}

impl EvmSelectors {
    /// Creates a new instance with raw data supplied by the file at `path`.
    /// The data must follow the format as exported by the [OpenChain API].
    ///
    /// # Errors
    ///
    /// This function will return an error if reading the file fails or if it contains invalid data.
    ///
    /// [OpenChain API]: https://docs.openchain.xyz/
    pub fn new_from_file(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)?;
        Self::new_from_raw(&raw)
    }

    /// Creates a new instance with raw data supplied by the string `raw`.
    /// The data must follow the format as exported by the [OpenChain API].
    ///
    /// # Errors
    ///
    /// This function will return an error if reading the file fails or if it contains invalid data.
    ///
    /// [OpenChain API]: https://docs.openchain.xyz/
    pub fn new_from_raw(raw: &str) -> Result<Self> {
        Ok(Self {
            items: raw
                .lines()
                .map(parse_line) // Parse the lines
                .collect::<Result<Vec<_>>>()? // If one had an error, return it
                .into_iter()
                .flatten() // Remove None values
                .fold(HashMap::new(), |mut map, (selector, function)| {
                    // Collect the items
                    map.entry(selector).or_default().push(function);
                    map
                }),
        })
    }

    /// Returns all known selectors.
    #[must_use]
    pub fn items(&self) -> &HashMap<Selector, Vec<Function>> {
        &self.items
    }

    /// Returns the functions known for the given selector.
    /// Note that since (especially 4-byte selectors) can have collisions, there can be multiple items returned.
    /// It is up to the caller to decide which one to use.
    #[must_use]
    pub fn get(&self, selector: &Selector) -> Option<&Vec<Function>> {
        self.items.get(selector)
    }

    /// Adds a new item for a given selector.
    /// This method does *not* check for duplicates.
    /// The item is purely kept in memory and not persisted.
    pub fn push(&mut self, selector: Selector, function: Function) {
        self.items.entry(selector).or_default().push(function);
    }
}
