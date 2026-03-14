use std::collections::BTreeMap;

use age::{
    secrecy::ExposeSecret,
    x25519::{Identity as AgeIdentity, Recipient as AgeRecipient},
};
use anyhow::{anyhow, bail, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    Key, XChaCha20Poly1305, XNonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::{rngs::OsRng, RngCore};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    api::{KeyEnvelope, SecretItem, SnapshotResponse},
    state::{KnownDevice, StoredDevice},
};

#[derive(Debug, Clone)]
pub struct DeviceIdentity {
    pub age_identity: String,
    pub age_recipient: String,
    pub signing_secret_key: String,
    pub signing_public_key: String,
}

#[derive(Debug, Clone)]
pub struct DecryptedSnapshot {
    pub secrets: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
struct ManifestPayload<'a> {
    workspace_id: &'a str,
    environment: &'a str,
    created_by_device: &'a str,
    key_envelopes: &'a [KeyEnvelope],
    secrets: &'a [SecretItem],
}

pub fn generate_device_identity() -> DeviceIdentity {
    let age_identity = AgeIdentity::generate();
    let signing_key = SigningKey::generate(&mut OsRng);
    DeviceIdentity {
        age_identity: age_identity.to_string().expose_secret().to_owned(),
        age_recipient: age_identity.to_public().to_string(),
        signing_secret_key: BASE64.encode(signing_key.to_bytes()),
        signing_public_key: BASE64.encode(signing_key.verifying_key().to_bytes()),
    }
}

pub fn encrypt_snapshot(
    device: &StoredDevice,
    device_id: &str,
    workspace_id: &str,
    environment: &str,
    values: &BTreeMap<String, String>,
) -> Result<(Vec<KeyEnvelope>, Vec<SecretItem>, String)> {
    let mut data_key = [0_u8; 32];
    OsRng.fill_bytes(&mut data_key);

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&data_key));
    let mut secrets = Vec::new();
    for (name, value) in values {
        let mut nonce = [0_u8; 24];
        OsRng.fill_bytes(&mut nonce);
        let aad = aad_bytes(workspace_id, environment, name);
        let ciphertext = cipher
            .encrypt(
                XNonce::from_slice(&nonce),
                chacha20poly1305::aead::Payload {
                    msg: value.as_bytes(),
                    aad: &aad,
                },
            )
            .with_context(|| format!("failed to encrypt secret {name}"))?;
        secrets.push(SecretItem {
            name: name.clone(),
            ciphertext: BASE64.encode(ciphertext),
            nonce: BASE64.encode(nonce),
            aad_hash: hex::encode(Sha256::digest(&aad)),
            value_checksum: hex::encode(Sha256::digest(value.as_bytes())),
        });
    }

    let recipient: AgeRecipient = device
        .age_recipient
        .parse()
        .map_err(|error: &str| anyhow!(error))?;
    let buffer = age::encrypt(&recipient, &data_key).context("failed to encrypt data key")?;

    let key_envelopes = vec![KeyEnvelope {
        device_id: device_id.to_string(),
        recipient: device.age_recipient.clone(),
        encrypted_key: BASE64.encode(buffer),
    }];
    let signature = sign_manifest(
        &device.signing_secret_key,
        workspace_id,
        environment,
        device_id,
        &key_envelopes,
        &secrets,
    )?;
    Ok((key_envelopes, secrets, signature))
}

pub fn decrypt_snapshot(
    device: &StoredDevice,
    known_devices: &BTreeMap<String, KnownDevice>,
    snapshot: &SnapshotResponse,
) -> Result<DecryptedSnapshot> {
    let known_device = known_devices
        .get(&snapshot.created_by_device)
        .ok_or_else(|| {
            anyhow!(
                "unknown device {} for snapshot verification",
                snapshot.created_by_device
            )
        })?;
    verify_manifest(&known_device.signing_public_key, snapshot)?;

    let envelope = snapshot
        .key_envelopes
        .iter()
        .find(|item| {
            item.device_id == device.id.clone().unwrap_or_default()
                || item.recipient == device.age_recipient
        })
        .ok_or_else(|| anyhow!("no key envelope found for current device"))?;
    let encrypted_key = BASE64
        .decode(envelope.encrypted_key.as_bytes())
        .context("failed to decode encrypted data key")?;
    let identity: AgeIdentity = device
        .age_identity
        .parse()
        .map_err(|error: &str| anyhow!(error))?;
    let data_key = age::decrypt(&identity, &encrypted_key).context("failed to unwrap data key")?;

    if data_key.len() != 32 {
        bail!("unexpected data key length");
    }

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&data_key));
    let mut values = BTreeMap::new();
    for secret in &snapshot.secrets {
        let aad = aad_bytes(&snapshot.workspace_id, &snapshot.environment, &secret.name);
        let expected_aad_hash = hex::encode(Sha256::digest(&aad));
        if expected_aad_hash != secret.aad_hash {
            bail!("aad hash mismatch for {}", secret.name);
        }
        let nonce = BASE64
            .decode(secret.nonce.as_bytes())
            .context("failed to decode secret nonce")?;
        let ciphertext = BASE64
            .decode(secret.ciphertext.as_bytes())
            .context("failed to decode secret ciphertext")?;
        let plaintext = cipher
            .decrypt(
                XNonce::from_slice(&nonce),
                chacha20poly1305::aead::Payload {
                    msg: ciphertext.as_ref(),
                    aad: &aad,
                },
            )
            .with_context(|| format!("failed to decrypt {}", secret.name))?;
        let checksum = hex::encode(Sha256::digest(&plaintext));
        if checksum != secret.value_checksum {
            bail!("value checksum mismatch for {}", secret.name);
        }
        values.insert(
            secret.name.clone(),
            String::from_utf8(plaintext).context("secret payload was not valid UTF-8")?,
        );
    }

    Ok(DecryptedSnapshot { secrets: values })
}

pub fn verify_manifest(signing_public_key: &str, snapshot: &SnapshotResponse) -> Result<()> {
    let key_bytes = BASE64
        .decode(signing_public_key.as_bytes())
        .context("failed to decode signing public key")?;
    let verifying_key = VerifyingKey::from_bytes(
        &key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("signing public key length mismatch"))?,
    )
    .context("failed to parse signing public key")?;
    let signature_bytes = BASE64
        .decode(snapshot.manifest_signature.as_bytes())
        .context("failed to decode manifest signature")?;
    let signature =
        Signature::from_slice(&signature_bytes).context("failed to parse manifest signature")?;
    let payload = manifest_payload(
        &snapshot.workspace_id,
        &snapshot.environment,
        &snapshot.created_by_device,
        &snapshot.key_envelopes,
        &snapshot.secrets,
    )?;
    verifying_key
        .verify(&payload, &signature)
        .context("manifest signature verification failed")
}

fn sign_manifest(
    signing_secret_key: &str,
    workspace_id: &str,
    environment: &str,
    created_by_device: &str,
    key_envelopes: &[KeyEnvelope],
    secrets: &[SecretItem],
) -> Result<String> {
    let key_bytes = BASE64
        .decode(signing_secret_key.as_bytes())
        .context("failed to decode signing secret key")?;
    let signing_key = SigningKey::from_bytes(
        &key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("signing secret key length mismatch"))?,
    );
    let payload = manifest_payload(
        workspace_id,
        environment,
        created_by_device,
        key_envelopes,
        secrets,
    )?;
    Ok(BASE64.encode(signing_key.sign(&payload).to_bytes()))
}

fn manifest_payload(
    workspace_id: &str,
    environment: &str,
    created_by_device: &str,
    key_envelopes: &[KeyEnvelope],
    secrets: &[SecretItem],
) -> Result<Vec<u8>> {
    serde_json::to_vec(&ManifestPayload {
        workspace_id,
        environment,
        created_by_device,
        key_envelopes,
        secrets,
    })
    .context("failed to serialize manifest payload")
}

fn aad_bytes(workspace_id: &str, environment: &str, name: &str) -> Vec<u8> {
    format!("{workspace_id}:{environment}:{name}").into_bytes()
}
