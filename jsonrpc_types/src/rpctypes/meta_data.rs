// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use util::Address;

/// Metadata of current chain.
///
/// Related system contract: scripts/contracts/system/sys_config.sol
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct MetaData {
    /// The id of current chain
    #[serde(rename = "chainId")]
    pub chain_id: u32,
    /// The name of current chain
    #[serde(rename = "chainName")]
    pub chain_name: String,
    /// The operator of current chain
    pub operator: String,
    /// Current operator's website URL
    pub website: String,
    /// Genesis block's timestamp (milliseconds)
    #[serde(rename = "genesisTimestamp")]
    pub genesis_timestamp: u64,
    /// Node address list which validate blocks
    pub validators: Vec<Address>,
    /// The interval time for creating a block (milliseconds)
    #[serde(rename = "blockInterval")]
    pub block_interval: u64,
}

#[cfg(test)]
mod tests {
    use super::{Address, MetaData};
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn metadata_serialization() {
        let value = json!({
            "chainId": 123,
            "chainName": "test-chain-name",
            "operator": "test-operator",
            "website": "https://www.google.com",
            "genesisTimestamp": 1524000000000u64,
            "validators": [
                "0xa83ca59edc87a9cc7e384afa8d218dcca71cae88",
                "0xbc1fafd5ba5485f97e937fe574f836b275e593dd",
                "0xfc788efe3fda574e21691d383e429be02c530e4c",
                "0xe9deeae8b2a43675f113d11573119b9c68e5e3d8",
            ],
            "blockInterval": 3000,
        });
        let metadata = MetaData {
            chain_id: 123,
            chain_name: "test-chain-name".to_owned(),
            operator: "test-operator".to_owned(),
            website: "https://www.google.com".to_owned(),
            genesis_timestamp: 1524000000000,
            validators: vec![
                "0xa83ca59edc87a9cc7e384afa8d218dcca71cae88",
                "0xbc1fafd5ba5485f97e937fe574f836b275e593dd",
                "0xfc788efe3fda574e21691d383e429be02c530e4c",
                "0xe9deeae8b2a43675f113d11573119b9c68e5e3d8",
            ].into_iter()
                .map(|s| Address::from_str(s).unwrap())
                .collect::<Vec<_>>(),
            block_interval: 3000,
        };
        assert_eq!(serde_json::to_value(metadata).unwrap(), value);
    }
}
