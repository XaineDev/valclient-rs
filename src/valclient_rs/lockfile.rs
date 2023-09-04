use crate::valclient_rs::errors::ValClientError;
use std::env::var_os;
use std::path::PathBuf;

pub struct LockfileData {
    pub lockfile_path: PathBuf,
    pub application: String,
    pub pid: String,
    pub port: String,
    pub password: String,
    pub protocol: String,
}

impl LockfileData {
    pub fn new() -> Result<Self, ValClientError> {
        // %LocalAppData%\Riot Games\Riot Client\Config\lockfile

        let lockfile_path: PathBuf = match var_os("LOCALAPPDATA") {
            Some(local_app_data) => {
                let mut path = PathBuf::from(local_app_data);
                path.push("Riot Games");
                path.push("Riot Client");
                path.push("Config");
                path.push("lockfile");
                match path.try_exists() {
                    Ok(_) => path,
                    Err(_) => {
                        return Err(ValClientError::new(
                            "Failed to fetch lockfile",
                            "lockfile doesnt exist",
                        ));
                    }
                }
            }
            None => {
                return Err(ValClientError::new(
                    "Failed to fetch lockfile",
                    "Could not fetch local appdata",
                ))
            }
        };

        let (application, pid, port, password, protocol) =
            Self::read_lockfile(lockfile_path.clone());

        Ok(Self {
            lockfile_path,
            application,
            pid,
            port,
            password,
            protocol,
        })
    }

    pub fn get_path_str(&self) -> &str {
        self.lockfile_path.to_str().expect("Path conversion failed")
    }

    fn read_lockfile(path: PathBuf) -> (String, String, String, String, String) {
        let lockfile_contents = std::fs::read_to_string(path).expect("Failed to read lockfile");
        let lockfile_split: Vec<&str> = lockfile_contents.split(':').collect();

        (
            lockfile_split[0].to_owned(),
            lockfile_split[1].to_owned(),
            lockfile_split[2].to_owned(),
            lockfile_split[3].to_owned(),
            lockfile_split[4].to_owned(),
        )
    }
}
