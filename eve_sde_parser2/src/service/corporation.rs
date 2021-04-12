mod npc_corporation;
mod npc_corporation_division;

pub use self::npc_corporation::*;
pub use self::npc_corporation_division::*;

use crate::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CorporationService {
    npc_corporations: HashMap<CorporationId, NpcCorporationEntry>,
    npc_divisions:    HashMap<DivisionId, NpcCorporationDivisionEntry>
}

impl CorporationService {
    const PATH_NPC_CORPORATIONS: &'static str = "sde/fsd/npcCorporations.yaml";
    const PATH_NPC_DIVISIONS:    &'static str = "sde/fsd/npcCorporationDivisions.yaml";

    pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveSdeParserError> {
        Ok(Self {
            npc_corporations: service_file_gen!(zip, Self::PATH_NPC_CORPORATIONS),
            npc_divisions:    service_file_gen!(zip, Self::PATH_NPC_DIVISIONS),
        })
    }
}
