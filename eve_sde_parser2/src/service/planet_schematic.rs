use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PlanceSchematicService(pub HashMap<SchematicId, PlanetSchematicEntry>);

impl PlanceSchematicService {
    const PATH: &'static str = "sde/fsd/planetSchematics.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlanetSchematicEntry {
    #[serde(rename = "cycleTime")]
    pub cycle_time: u32,
    #[serde(rename = "nameID")]
    pub name:       HashMap<String, String>,
    #[serde(rename = "pins")]
    pub skills:     Vec<TypeId>,
    #[serde(rename = "types")]
    pub types:      HashMap<TypeId, SchematicType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicType {
    #[serde(rename = "isInput")]
    pub is_input: bool,
    #[serde(rename = "quantity")]
    pub quantity: u32,
}
