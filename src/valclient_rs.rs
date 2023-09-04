mod auth;
pub mod errors;
mod lockfile;
pub mod resources;

use crate::valclient_rs::auth::Auth;
use crate::valclient_rs::errors::ValClientError;
use crate::valclient_rs::lockfile::LockfileData;
use crate::valclient_rs::resources::{Region, Resources};
use std::io::BufRead;

use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking;
use reqwest::header::HeaderMap;

pub struct UserAuthentication {
    pub puuid: String,
    pub headers: HeaderMap,
    pub local_headers: HeaderMap,
}

impl UserAuthentication {
    pub fn new(puuid: String, headers: HeaderMap, local_headers: HeaderMap) -> Self {
        Self {
            puuid,
            headers,
            local_headers,
        }
    }
}
pub struct ValClient {
    pub resources: Resources,
    pub lockfile: LockfileData,

    pub user_auth: UserAuthentication,

    pub player_name: String,
    pub player_tag: String,
    pub region: String,
    pub shard: String,
    pub client_platform: String,

    pub auth: Auth,
    pub has_auth: bool,
}

impl ValClient {
    pub fn new(region: &String, auth_opt: Option<Auth>) -> Result<Self, ValClientError> {
        let lockfile_result = LockfileData::new();
        let lockfile = match lockfile_result {
            Ok(data) => data,
            Err(error) => return Err(error),
        };

        let (auth, has_auth) = match auth_opt {
            Some(auth) => (auth, true),
            _ => (Auth::none(), false),
        };

        let mut client = Self {
            resources: Resources::new(),
            lockfile,

            user_auth: UserAuthentication {
                puuid: "".to_string(),
                headers: Default::default(),
                local_headers: Default::default(),
            },
            player_name: "".to_string(),
            player_tag: "".to_string(),
            region: region.clone(),
            shard: region.clone(),
            client_platform: "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9".to_string(),

            auth,
            has_auth,
        };

        if !Region::is_valid_region(region) {
            return Err(ValClientError::new(
                "Invalid region chosen",
                "region doesnt match any valid choices",
            ));
        }

        if client
            .resources
            .region_shard_override
            .contains_key(&Region::from(region))
        {
            client.shard = client
                .resources
                .region_shard_override
                .get(&Region::from(region))
                .unwrap()
                .to_string();
        }

        if client
            .resources
            .shard_region_override
            .contains_key(&Region::from(region))
        {
            client.region = client
                .resources
                .shard_region_override
                .get(&Region::from(region))
                .unwrap()
                .to_string();
        }

        client.resources.update_endpoints(
            client.lockfile.port.clone(),
            client.region.clone(),
            client.shard.clone(),
        );

        Ok(client)
    }

    pub fn activate(&mut self) -> Result<(), ValClientError> {
        if !self.has_auth {
            self.user_auth = self.get_headers()?;
        } else {
            self.user_auth = self.auth.authenticate()?;
        }
        Ok(())
    }

    fn get_headers(&self) -> Result<UserAuthentication, ValClientError> {
        let mut local_headers = HeaderMap::new();
        let mut headers = HeaderMap::new();

        let auth = format!("riot:{}", self.lockfile.password);
        let b64_auth = general_purpose::STANDARD.encode(auth);
        local_headers.insert(
            "Authorization",
            format!("Basic {}", b64_auth).parse().unwrap(),
        );

        let client = blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .https_only(true)
            .build()?;

        let url = format!(
            "{}/entitlements/v1/token",
            self.resources.get_base_local_endpoint()
        );

        let response = client.get(url).headers(local_headers.clone()).send();
        let raw_json = response?.text()?;
        let entitlements_json = json::parse(&raw_json)?;
        let puuid = entitlements_json["subject"]
            .as_str()
            .ok_or(ValClientError::new(
                "failed to get puuid",
                "failed to get puuid string from entitlements json",
            ))?
            .to_owned();

        headers.insert(
            "Authorization",
            format!("Bearer {}", entitlements_json["accessToken"])
                .parse()
                .unwrap(),
        );
        headers.insert(
            "X-Riot-Entitlements-JWT",
            entitlements_json["token"]
                .as_str()
                .ok_or(ValClientError::new(
                    "failed to get entitlement token",
                    "could not convert token from entitlements json to string",
                ))?
                .parse()?,
        );
        headers.insert(
            "X-Riot-ClientPlatform",
            self.client_platform.parse().unwrap(),
        );
        headers.insert(
            "X-Riot-ClientVersion",
            Self::get_current_version()?.parse().unwrap(),
        );

        Ok(UserAuthentication::new(puuid, headers, local_headers))
    }

    fn get_current_version() -> Result<String, ValClientError> {
        let data = blocking::get("https://valorant-api.com/v1/version")?.text()?;
        let json = json::parse(&data)?;

        Ok(format!(
            "{}-shipping-{}-{}",
            json["data"]["branch"],
            json["data"]["buildVersion"],
            json["data"]["version"]
                .as_str()
                .ok_or(ValClientError::new(
                    "failed to get version number",
                    "could not convert version number from version json to str"
                ))
                .unwrap()
                .to_string()
                .split(".")
                .nth(3)
                .ok_or(ValClientError::new(
                    "failed to get version number",
                    "could not get 4th element of version number"
                ))?
        ))
    }
}
