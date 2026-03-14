use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
    client: Client,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>, token: Option<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            token,
        }
    }

    pub fn start_github_auth(&self, device_name: &str) -> Result<StartAuthResponse> {
        self.request(
            reqwest::Method::POST,
            "/v1/auth/github/start",
            Some(StartAuthRequest {
                device_name: device_name.to_string(),
            }),
        )
    }

    pub fn approve_device_auth(&self, device_code: &str, actor_email: &str) -> Result<()> {
        self.request::<ApproveDeviceAuthRequest, serde_json::Value>(
            reqwest::Method::POST,
            "/v1/auth/device/approve",
            Some(ApproveDeviceAuthRequest {
                device_code: device_code.to_string(),
                actor_email: actor_email.to_string(),
            }),
        )?;
        Ok(())
    }

    pub fn exchange_device_auth(&self, device_code: &str) -> Result<DeviceTokenResponse> {
        self.request(
            reqwest::Method::POST,
            "/v1/auth/device/exchange",
            Some(ExchangeDeviceAuthRequest {
                device_code: device_code.to_string(),
            }),
        )
    }

    pub fn register_device(&self, request: &RegisterDeviceRequest) -> Result<DeviceResponse> {
        self.request(reqwest::Method::POST, "/v1/devices", Some(request))
    }

    pub fn create_workspace(&self, request: &CreateWorkspaceRequest) -> Result<WorkspaceResponse> {
        self.request(reqwest::Method::POST, "/v1/workspaces", Some(request))
    }

    pub fn get_workspace(&self, workspace_id: &str) -> Result<WorkspaceResponse> {
        self.request::<(), WorkspaceResponse>(
            reqwest::Method::GET,
            &format!("/v1/workspaces/{workspace_id}"),
            None,
        )
    }

    pub fn create_snapshot(&self, request: &CreateSnapshotRequest) -> Result<SnapshotResponse> {
        self.request(reqwest::Method::POST, "/v1/snapshots", Some(request))
    }

    pub fn get_latest_snapshot(
        &self,
        workspace_id: &str,
        environment: &str,
    ) -> Result<SnapshotResponse> {
        self.request::<(), SnapshotResponse>(
            reqwest::Method::GET,
            &format!("/v1/workspaces/{workspace_id}/environments/{environment}/snapshots/latest"),
            None,
        )
    }

    pub fn get_environment_status(
        &self,
        workspace_id: &str,
        environment: &str,
        local_version: u64,
    ) -> Result<EnvironmentStatusResponse> {
        let path = format!(
            "/v1/workspaces/{workspace_id}/environments/{environment}/status?local_version={local_version}"
        );
        self.request::<(), EnvironmentStatusResponse>(reqwest::Method::GET, &path, None)
    }

    fn request<RequestBody, ResponseBody>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<RequestBody>,
    ) -> Result<ResponseBody>
    where
        RequestBody: Serialize,
        ResponseBody: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let mut request = self
            .client
            .request(method, url)
            .header("Accept", "application/json");

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().context("failed to contact control plane")?;
        let status = response.status();
        if status.is_success() {
            if status == reqwest::StatusCode::NO_CONTENT {
                return serde_json::from_value(serde_json::json!({}))
                    .context("failed to decode empty response");
            }
            return response
                .json::<ResponseBody>()
                .context("failed to decode control plane response");
        }

        let payload: serde_json::Value = response
            .json()
            .unwrap_or_else(|_| serde_json::json!({"error":"unknown error"}));
        let message = payload
            .get("error")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown error");
        Err(anyhow!(
            "control plane request failed ({status}): {message}"
        ))
    }
}

#[derive(Debug, Serialize)]
pub struct StartAuthRequest {
    #[serde(rename = "deviceName")]
    pub device_name: String,
}

#[derive(Debug, Deserialize)]
pub struct StartAuthResponse {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    #[serde(rename = "userCode")]
    pub user_code: String,
    #[serde(rename = "verificationUrl")]
    pub verification_url: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct ApproveDeviceAuthRequest {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    #[serde(rename = "actorEmail")]
    pub actor_email: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangeDeviceAuthRequest {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceTokenResponse {
    pub token: String,
    #[serde(rename = "actorEmail")]
    pub actor_email: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterDeviceRequest {
    pub name: String,
    #[serde(rename = "encryptionPublicKey")]
    pub encryption_public_key: String,
    #[serde(rename = "signingPublicKey")]
    pub signing_public_key: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DeviceResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "encryptionPublicKey")]
    pub encryption_public_key: String,
    #[serde(rename = "signingPublicKey")]
    pub signing_public_key: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WorkspaceResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub members: Vec<MemberResponse>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MemberResponse {
    pub id: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyEnvelope {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub recipient: String,
    #[serde(rename = "encryptedKey")]
    pub encrypted_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretItem {
    pub name: String,
    pub ciphertext: String,
    pub nonce: String,
    #[serde(rename = "aadHash")]
    pub aad_hash: String,
    #[serde(rename = "valueChecksum")]
    pub value_checksum: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateSnapshotRequest {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    pub environment: String,
    #[serde(rename = "createdByDevice")]
    pub created_by_device: String,
    #[serde(rename = "manifestSignature")]
    pub manifest_signature: String,
    #[serde(rename = "keyEnvelopes")]
    pub key_envelopes: Vec<KeyEnvelope>,
    pub secrets: Vec<SecretItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotResponse {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    pub environment: String,
    pub version: u64,
    #[serde(rename = "createdByDevice")]
    pub created_by_device: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "manifestSignature")]
    pub manifest_signature: String,
    #[serde(rename = "keyEnvelopes")]
    pub key_envelopes: Vec<KeyEnvelope>,
    pub secrets: Vec<SecretItem>,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentStatusResponse {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    pub environment: String,
    #[serde(rename = "latestVersion")]
    pub latest_version: u64,
    #[serde(rename = "localVersion")]
    pub local_version: u64,
    #[serde(rename = "outOfDate")]
    pub out_of_date: bool,
}
