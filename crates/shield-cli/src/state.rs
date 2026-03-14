use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    Key, XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use crate::api::SnapshotResponse;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppState {
    pub api_base_url: String,
    pub actor_email: Option<String>,
    pub token: Option<String>,
    pub current_workspace_id: Option<String>,
    pub local_cache_key: Option<String>,
    pub device: Option<StoredDevice>,
    pub known_devices: BTreeMap<String, KnownDevice>,
    pub workspaces: BTreeMap<String, WorkspaceState>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredDevice {
    pub id: Option<String>,
    pub name: String,
    pub age_identity: String,
    pub age_recipient: String,
    pub signing_secret_key: String,
    pub signing_public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnownDevice {
    pub id: String,
    pub name: String,
    pub encryption_public_key: String,
    pub signing_public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceState {
    pub id: String,
    pub name: String,
    pub environments: BTreeMap<String, EnvironmentState>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EnvironmentState {
    pub version: u64,
    pub dirty: bool,
    pub cache: Option<EncryptedCache>,
    pub last_snapshot: Option<SnapshotResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedCache {
    pub nonce: String,
    pub ciphertext: String,
}

pub struct StateStore {
    root: PathBuf,
}

impl StateStore {
    pub fn new() -> Result<Self> {
        if let Ok(path) = std::env::var("ENVSHIELD_HOME") {
            return Ok(Self {
                root: PathBuf::from(path),
            });
        }
        let home = dirs::home_dir().context("failed to find home directory")?;
        Ok(Self {
            root: home.join(".envshield"),
        })
    }

    pub fn from_root(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn load(&self) -> Result<AppState> {
        let path = self.root.join("state.json");
        if !path.exists() {
            return Ok(AppState {
                api_base_url: std::env::var("ENVSHIELD_API_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
                ..AppState::default()
            });
        }
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_json::from_str(&content).context("failed to decode local EnvShield state")
    }

    pub fn save(&self, state: &AppState) -> Result<()> {
        fs::create_dir_all(&self.root)
            .with_context(|| format!("failed to create {}", self.root.display()))?;
        let serialized =
            serde_json::to_string_pretty(state).context("failed to serialize local state")?;
        fs::write(self.root.join("state.json"), serialized).context("failed to persist local state")
    }
}

impl AppState {
    pub fn resolve_workspace_id(&self, workspace: Option<&str>) -> Result<String> {
        if let Some(workspace) = workspace {
            if self.workspaces.contains_key(workspace) {
                return Ok(workspace.to_string());
            }
            if let Some((workspace_id, _)) = self
                .workspaces
                .iter()
                .find(|(_, item)| item.name == workspace)
            {
                return Ok(workspace_id.clone());
            }
            return Err(anyhow!("workspace {workspace} not found"));
        }

        self.current_workspace_id
            .clone()
            .ok_or_else(|| anyhow!("no current workspace configured; run `shield init` first"))
    }

    pub fn device(&self) -> Result<&StoredDevice> {
        self.device
            .as_ref()
            .ok_or_else(|| anyhow!("device not initialized; run `shield login` first"))
    }

    pub fn device_mut(&mut self) -> Result<&mut StoredDevice> {
        self.device
            .as_mut()
            .ok_or_else(|| anyhow!("device not initialized; run `shield login` first"))
    }

    pub fn local_cache_key(&mut self) -> Result<[u8; 32]> {
        if self.local_cache_key.is_none() {
            let mut key = [0_u8; 32];
            OsRng.fill_bytes(&mut key);
            self.local_cache_key = Some(BASE64.encode(key));
        }
        let encoded = self
            .local_cache_key
            .clone()
            .ok_or_else(|| anyhow!("failed to initialize local cache key"))?;
        let decoded = BASE64
            .decode(encoded.as_bytes())
            .context("failed to decode local cache key")?;
        decoded
            .try_into()
            .map_err(|_| anyhow!("local cache key length mismatch"))
    }

    pub fn upsert_workspace(&mut self, workspace_id: &str, name: &str) {
        self.workspaces
            .entry(workspace_id.to_string())
            .and_modify(|workspace| workspace.name = name.to_string())
            .or_insert_with(|| WorkspaceState {
                id: workspace_id.to_string(),
                name: name.to_string(),
                environments: BTreeMap::new(),
            });
        self.current_workspace_id = Some(workspace_id.to_string());
    }

    pub fn environment_state_mut(
        &mut self,
        workspace_id: &str,
        environment: &str,
    ) -> Result<&mut EnvironmentState> {
        let workspace = self
            .workspaces
            .get_mut(workspace_id)
            .ok_or_else(|| anyhow!("workspace {workspace_id} not found in local state"))?;
        Ok(workspace
            .environments
            .entry(environment.to_string())
            .or_insert_with(EnvironmentState::default))
    }
}

pub fn decrypt_local_cache(
    key_bytes: &[u8; 32],
    cache: &EncryptedCache,
) -> Result<BTreeMap<String, String>> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let nonce_bytes = BASE64
        .decode(cache.nonce.as_bytes())
        .context("failed to decode local cache nonce")?;
    let ciphertext = BASE64
        .decode(cache.ciphertext.as_bytes())
        .context("failed to decode local cache ciphertext")?;
    let plaintext = cipher
        .decrypt(XNonce::from_slice(&nonce_bytes), ciphertext.as_ref())
        .context("failed to decrypt local cache")?;
    let values =
        serde_json::from_slice(&plaintext).context("failed to decode local cache payload")?;
    Ok(values)
}

pub fn encrypt_local_cache(
    key_bytes: &[u8; 32],
    values: &BTreeMap<String, String>,
) -> Result<EncryptedCache> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let plaintext = serde_json::to_vec(values).context("failed to encode local cache payload")?;
    let mut nonce = [0_u8; 24];
    OsRng.fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(&nonce), plaintext.as_ref())
        .context("failed to encrypt local cache")?;
    Ok(EncryptedCache {
        nonce: BASE64.encode(nonce),
        ciphertext: BASE64.encode(ciphertext),
    })
}

pub struct DecryptedSecrets {
    pub values: BTreeMap<String, String>,
}

impl DecryptedSecrets {
    pub fn new(values: BTreeMap<String, String>) -> Self {
        Self { values }
    }
}

impl Drop for DecryptedSecrets {
    fn drop(&mut self) {
        for value in self.values.values_mut() {
            value.zeroize();
        }
    }
}

pub fn state_file_exists(root: &Path) -> bool {
    root.join("state.json").exists()
}
