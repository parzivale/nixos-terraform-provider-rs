use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tf_provider::{
    map,
    schema::{Attribute, AttributeConstraint, AttributeType, Block, Description, Schema},
    DataSource, Diagnostics,
};

pub struct SystemInfoDataSource;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfoState<'a> {
    /// Target host to query
    pub host: &'a str,
    /// NixOS version string (computed)
    pub nixos_version: Option<&'a str>,
    /// Kernel version string (computed)
    pub kernel_version: Option<&'a str>,
    /// System architecture (computed, e.g. "x86_64-linux")
    pub system: Option<&'a str>,
}

#[async_trait]
impl DataSource for SystemInfoDataSource {
    type State<'a> = SystemInfoState<'a>;
    type ProviderMetaState<'a> = ();

    fn schema(&self, _diags: &mut Diagnostics) -> Option<Schema> {
        Some(Schema {
            version: 1,
            block: Block {
                attributes: map! {
                    "host" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain("Target NixOS host to query"),
                        constraint: AttributeConstraint::Required,
                        sensitive: false,
                        deprecated: false,
                    },
                    "nixos_version" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain("NixOS version string"),
                        constraint: AttributeConstraint::Computed,
                        sensitive: false,
                        deprecated: false,
                    },
                    "kernel_version" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain("Kernel version string"),
                        constraint: AttributeConstraint::Computed,
                        sensitive: false,
                        deprecated: false,
                    },
                    "system" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain(
                            "System architecture (e.g. \"x86_64-linux\")",
                        ),
                        constraint: AttributeConstraint::Computed,
                        sensitive: false,
                        deprecated: false,
                    }
                },
                blocks: HashMap::new(),
                description: Description::plain(
                    "Reads runtime system information from a NixOS host",
                ),
                deprecated: false,
                ..Default::default()
            },
        })
    }

    async fn read<'a>(
        &self,
        _diags: &mut Diagnostics,
        config: Self::State<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<Self::State<'a>> {
        // TODO: SSH into config.host and run:
        //   nixos-version, uname -r, uname -m
        // Populate the computed fields from the output
        Some(config)
    }
}
