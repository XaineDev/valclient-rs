use crate::valclient_rs::resources::Region::{Ap, Br, Eu, Kr, Latam, Na, None, Pbe};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Region {
    Na,
    Eu,
    Latam,
    Br,
    Ap,
    Kr,
    Pbe,
    None,
}

impl Region {
    pub fn is_valid_region(region: &str) -> bool {
        Region::from(region) != None
    }

    pub fn from(region: &str) -> Self {
        match region {
            "na" => Na,
            "eu" => Eu,
            "latam" => Latam,
            "br" => Br,
            "ap" => Ap,
            "kr" => Kr,
            "pbe" => Pbe,
            _ => None,
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Na => "na",
            Eu => "eu",
            Latam => "latam",
            Br => "br",
            Ap => "ap",
            Kr => "kr",
            Pbe => "pbe",
            _ => "invalid",
        }
        .fmt(f)
    }
}

/* todo: reimplement
pub enum QueueType {
    Competitive,
    Custom,
    Deathmatch,
    GGTeam,
    Snowball,
    SpikeRush,
    Unrated,
    OneFa,
    Null,
}
 */

pub struct Resources {
    pub region_shard_override: HashMap<Region, Region>,
    pub shard_region_override: HashMap<Region, Region>,

    port: String,
    region: String,
    shard: String,
}

impl Resources {
    pub fn new() -> Self {
        let mut region_shard_override = HashMap::new();
        region_shard_override.insert(Latam, Na);
        region_shard_override.insert(Br, Na);

        let mut shard_region_override = HashMap::new();
        shard_region_override.insert(Pbe, Na);

        Self {
            region_shard_override,
            shard_region_override,
            port: "".to_string(),
            region: "".to_string(),
            shard: "".to_string(),
        }
    }

    pub fn update_endpoints(&mut self, port: String, region: String, shard: String) {
        self.port = port;
        self.region = region;
        self.shard = shard;
    }

    pub fn get_base_local_endpoint(&self) -> String {
        format!("https://127.0.0.1:{}", self.port)
    }

    pub fn get_base_endpoint(&self) -> String {
        format!("https://pd.{}.a.pvp.net", self.shard)
    }

    pub fn get_base_glz_endpoint(&self) -> String {
        format!("https://glz-{}-1.{}.a.pvp.net", self.region, self.shard)
    }

    pub fn get_base_shared_endpoint(&self) -> String {
        format!("https://shared.{}.a.pvp.net", self.shard)
    }
}
