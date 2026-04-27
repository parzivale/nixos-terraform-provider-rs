use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tf_provider::{
    map,
    schema::{
        Attribute, AttributeConstraint, AttributeType, Block, Description, NestedBlock, Schema,
    },
    AttributePath, Diagnostics, Resource,
};

pub struct NixosConfigurationResource;

/// A secret file to be deployed to the target host before building
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretFile<'a> {
    /// File contents
    pub content: &'a str,
    /// Owner user
    pub owner: &'a str,
    /// Owner group
    pub group: &'a str,
    /// File mode (e.g. "0600")
    pub mode: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NixosConfigurationState<'a> {
    /// Target NixOS host
    pub host: &'a str,
    /// Flake reference to build (e.g. ".#hostname")
    pub flake: &'a str,
    /// Optional dedicated build host — overrides the provider-level build_host
    pub build_host: Option<&'a str>,
    /// Secret files to deploy before building, keyed by destination path
    pub files: HashMap<String, SecretFile<'a>>,
}

#[async_trait]
impl Resource for NixosConfigurationResource {
    type State<'a> = NixosConfigurationState<'a>;
    type PrivateState<'a> = ();
    type ProviderMetaState<'a> = ();

    fn schema(&self, _diags: &mut Diagnostics) -> Option<Schema> {
        Some(Schema {
            version: 1,
            block: Block {
                attributes: map! {
                    "host" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain("Target NixOS host"),
                        constraint: AttributeConstraint::Required,
                        sensitive: false,
                        deprecated: false,
                    },
                    "flake" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain(
                            "Flake reference to build and switch (e.g. \".#hostname\")",
                        ),
                        constraint: AttributeConstraint::Required,
                        sensitive: false,
                        deprecated: false,
                    },
                    "build_host" => Attribute {
                        attr_type: AttributeType::String,
                        description: Description::plain(
                            "Optional dedicated build host, overrides the provider-level build_host",
                        ),
                        constraint: AttributeConstraint::Optional,
                        sensitive: false,
                        deprecated: false,
                    }
                },
                blocks: map! {
                    "files" => NestedBlock::Map(Block {
                        attributes: map! {
                            "content" => Attribute {
                                attr_type: AttributeType::String,
                                description: Description::plain("File contents"),
                                constraint: AttributeConstraint::Required,
                                sensitive: true,
                                deprecated: false,
                            },
                            "owner" => Attribute {
                                attr_type: AttributeType::String,
                                description: Description::plain("Owner user"),
                                constraint: AttributeConstraint::Optional,
                                sensitive: false,
                                deprecated: false,
                            },
                            "group" => Attribute {
                                attr_type: AttributeType::String,
                                description: Description::plain("Owner group"),
                                constraint: AttributeConstraint::Optional,
                                sensitive: false,
                                deprecated: false,
                            },
                            "mode" => Attribute {
                                attr_type: AttributeType::String,
                                description: Description::plain("File mode (e.g. \"0600\")"),
                                constraint: AttributeConstraint::Optional,
                                sensitive: false,
                                deprecated: false,
                            }
                        },
                        description: Description::plain(
                            "Secret files to deploy to the target before building",
                        ),
                        ..Default::default()
                    })
                },
                description: Description::plain("Manages a NixOS configuration on a remote host"),
                deprecated: false,
                ..Default::default()
            },
        })
    }

    async fn read<'a>(
        &self,
        _diags: &mut Diagnostics,
        state: Self::State<'a>,
        _private_state: Self::PrivateState<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<(Self::State<'a>, Self::PrivateState<'a>)> {
        // NixOS configurations are applied imperatively; just return current state
        Some((state, ()))
    }

    async fn plan_create<'a>(
        &self,
        _diags: &mut Diagnostics,
        proposed_state: Self::State<'a>,
        _config_state: Self::State<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<(Self::State<'a>, Self::PrivateState<'a>)> {
        Some((proposed_state, ()))
    }

    async fn plan_update<'a>(
        &self,
        _diags: &mut Diagnostics,
        _prior_state: Self::State<'a>,
        proposed_state: Self::State<'a>,
        _config_state: Self::State<'a>,
        _prior_private_state: Self::PrivateState<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<(Self::State<'a>, Self::PrivateState<'a>, Vec<AttributePath>)> {
        Some((proposed_state, (), vec![]))
    }

    async fn plan_destroy<'a>(
        &self,
        _diags: &mut Diagnostics,
        _prior_state: Self::State<'a>,
        _prior_private_state: Self::PrivateState<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<Self::PrivateState<'a>> {
        Some(())
    }

    async fn create<'a>(
        &self,
        diags: &mut Diagnostics,
        planned_state: Self::State<'a>,
        _config_state: Self::State<'a>,
        _planned_private_state: Self::PrivateState<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<(Self::State<'a>, Self::PrivateState<'a>)> {
        self.apply(diags, &planned_state).await?;
        Some((planned_state, ()))
    }

    async fn update<'a>(
        &self,
        diags: &mut Diagnostics,
        _prior_state: Self::State<'a>,
        planned_state: Self::State<'a>,
        _config_state: Self::State<'a>,
        _planned_private_state: Self::PrivateState<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<(Self::State<'a>, Self::PrivateState<'a>)> {
        self.apply(diags, &planned_state).await?;
        Some((planned_state, ()))
    }

    async fn destroy<'a>(
        &self,
        _diags: &mut Diagnostics,
        _prior_state: Self::State<'a>,
        _planned_private_state: Self::PrivateState<'a>,
        _provider_meta_state: Self::ProviderMetaState<'a>,
    ) -> Option<()> {
        // NixOS configurations are not "destroyed" — this is a no-op
        Some(())
    }
}

impl NixosConfigurationResource {
    /// Deploy secret files then run nixos-rebuild switch on the target host.
    async fn apply<'a>(
        &self,
        diags: &mut Diagnostics,
        state: &NixosConfigurationState<'a>,
    ) -> Option<()> {
        // TODO: open SSH session via ssh-agent (SSH_AUTH_SOCK) using openssh::SessionBuilder
        //   let session = openssh::SessionBuilder::default()
        //       .connect(format!("ssh://{}@{}", user, state.host)).await?;
        // TODO: deploy state.files to the target over the session
        // TODO: run `nixos-rebuild switch --flake <state.flake>` on build_host or target,
        //       streaming stdout/stderr through tf_provider logs
        let _ = (diags, state);
        Some(())
    }
}
