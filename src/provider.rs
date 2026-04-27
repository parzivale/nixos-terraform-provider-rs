use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tf_provider::{
    map,
    schema::{Attribute, AttributeConstraint, AttributeType, Block, Description, Schema},
    Diagnostics, Provider,
};

use crate::{
    datasources::system_info::SystemInfoDataSource,
    resources::nixos_configuration::NixosConfigurationResource,
};

#[derive(Debug, Default)]
pub struct NixosProvider;

#[derive(Debug, Serialize, Deserialize)]
pub struct NixosProviderConfig<'a> {
    pub user: &'a str,
    pub build_host: Option<&'a str>,
}

#[async_trait]
impl Provider for NixosProvider {
    type Config<'a> = NixosProviderConfig<'a>;
    type MetaState<'a> = ();

    fn schema(&self, _diags: &mut Diagnostics) -> Option<Schema> {
        Some(Schema {
            version: 1,
            block: Block {
                attributes: map! {
                    "user" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain(
                            "SSH user for connecting to hosts. \
                             Authentication is handled by the SSH agent (SSH_AUTH_SOCK).",
                        ),
                        constraint: AttributeConstraint::Required,
                        sensitive: false,
                        deprecated: false,
                    },
                    "build_host" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain(
                            "Optional dedicated host to offload nix builds",
                        ),
                        constraint: AttributeConstraint::Optional,
                        sensitive: false,
                        deprecated: false,
                    }
                },
                blocks: HashMap::new(),
                description: Description::plain("NixOS Terraform provider"),
                deprecated: false,
                ..Default::default()
            },
        })
    }

    fn get_resources(
        &self,
        _diags: &mut Diagnostics,
    ) -> Option<HashMap<String, Box<dyn tf_provider::DynamicResource>>> {
        Some(map! {
            "nixos_configuration" => Box::new(NixosConfigurationResource) as Box<_>
        })
    }

    fn get_data_sources(
        &self,
        _diags: &mut Diagnostics,
    ) -> Option<HashMap<String, Box<dyn tf_provider::DynamicDataSource>>> {
        Some(map! {
            "nixos_system_info" => Box::new(SystemInfoDataSource) as Box<_>
        })
    }
}
