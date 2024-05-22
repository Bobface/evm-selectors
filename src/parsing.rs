use anyhow::Result;
use ethers::abi::{AbiParser, Function};

use crate::Selector;

pub(super) fn parse_line(line: &str) -> Result<Option<(Selector, Function)>> {
    if line.is_empty() {
        return Ok(None);
    }

    let parts: Vec<&str> = line.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Could not split line by first comma: {}",
            line
        ));
    }

    let selector = parse_selector(parts[0])?;

    // Parsing the function is more complex, so we use the AbiParser.
    // Unfortunately, this seems to fail for some valid inputs currently.
    // Therefore, we just filter out invalid ones for now.
    // Once this is fixed, we would return the error here.
    let Ok(function) = AbiParser::default().parse_function(parts[1]) else {
        return Ok(None);
    };

    Ok(Some((selector, function)))
}

fn parse_selector(raw_selector: &str) -> Result<Selector> {
    let bytes = parse_hex_string_to_bytes(raw_selector)?;

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

fn parse_hex_string_to_bytes(hex: &str) -> Result<Vec<u8>> {
    if !hex.starts_with("0x") {
        return Err(anyhow::anyhow!("Selector does not start with 0x: {}", hex));
    }

    let hex = &hex[2..];
    if hex.len() % 2 != 0 {
        return Err(anyhow::anyhow!(
            "Selector has odd number of characters: {}",
            hex
        ));
    }

    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
            anyhow::anyhow!("Could not parse selector {} at index {}: {}", hex, i, e)
        })?;
        bytes.push(byte);
    }

    Ok(bytes)
}
