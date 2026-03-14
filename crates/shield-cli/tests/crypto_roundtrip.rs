use std::collections::BTreeMap;

use shield_cli::{
    api::{KeyEnvelope, SecretItem, SnapshotResponse},
    crypto::{decrypt_snapshot, encrypt_snapshot, generate_device_identity},
    state::{KnownDevice, StoredDevice},
};

fn fixture_device() -> (StoredDevice, KnownDevice) {
    let identity = generate_device_identity();
    let stored = StoredDevice {
        id: Some("device-1".to_string()),
        name: "Laptop".to_string(),
        age_identity: identity.age_identity,
        age_recipient: identity.age_recipient,
        signing_secret_key: identity.signing_secret_key,
        signing_public_key: identity.signing_public_key.clone(),
    };
    let known = KnownDevice {
        id: "device-1".to_string(),
        name: "Laptop".to_string(),
        encryption_public_key: stored.age_recipient.clone(),
        signing_public_key: identity.signing_public_key,
    };
    (stored, known)
}

#[test]
fn roundtrip_encrypts_and_decrypts_snapshot() {
    let (device, known_device) = fixture_device();
    let mut values = BTreeMap::new();
    values.insert("DATABASE_URL".to_string(), "postgres://local".to_string());

    let (key_envelopes, secrets, signature) =
        encrypt_snapshot(&device, "device-1", "workspace-1", "development", &values).unwrap();
    let snapshot = SnapshotResponse {
        snapshot_id: "snapshot-1".to_string(),
        workspace_id: "workspace-1".to_string(),
        environment: "development".to_string(),
        version: 1,
        created_by_device: "device-1".to_string(),
        created_at: "2026-03-14T00:00:00Z".to_string(),
        manifest_signature: signature,
        key_envelopes,
        secrets,
    };
    let known_devices = BTreeMap::from([(known_device.id.clone(), known_device)]);
    let decrypted = decrypt_snapshot(&device, &known_devices, &snapshot).unwrap();
    assert_eq!(
        decrypted.secrets.get("DATABASE_URL"),
        Some(&"postgres://local".to_string())
    );
}

#[test]
fn wrong_device_key_cannot_decrypt_snapshot() {
    let (device, known_device) = fixture_device();
    let (other_device, _) = fixture_device();
    let mut values = BTreeMap::new();
    values.insert("DATABASE_URL".to_string(), "postgres://local".to_string());

    let (key_envelopes, secrets, signature) =
        encrypt_snapshot(&device, "device-1", "workspace-1", "development", &values).unwrap();
    let snapshot = SnapshotResponse {
        snapshot_id: "snapshot-1".to_string(),
        workspace_id: "workspace-1".to_string(),
        environment: "development".to_string(),
        version: 1,
        created_by_device: "device-1".to_string(),
        created_at: "2026-03-14T00:00:00Z".to_string(),
        manifest_signature: signature,
        key_envelopes,
        secrets,
    };
    let known_devices = BTreeMap::from([(known_device.id.clone(), known_device)]);
    let error = decrypt_snapshot(&other_device, &known_devices, &snapshot).unwrap_err();
    assert!(error.to_string().contains("no key envelope") || error.to_string().contains("unwrap"));
}

#[test]
fn tampered_manifest_fails_verification() {
    let (device, known_device) = fixture_device();
    let mut values = BTreeMap::new();
    values.insert("DATABASE_URL".to_string(), "postgres://local".to_string());

    let (key_envelopes, mut secrets, signature) =
        encrypt_snapshot(&device, "device-1", "workspace-1", "development", &values).unwrap();
    secrets[0] = SecretItem {
        name: "DATABASE_URL".to_string(),
        ciphertext: "tampered".to_string(),
        nonce: secrets[0].nonce.clone(),
        aad_hash: secrets[0].aad_hash.clone(),
        value_checksum: secrets[0].value_checksum.clone(),
    };
    let snapshot = SnapshotResponse {
        snapshot_id: "snapshot-1".to_string(),
        workspace_id: "workspace-1".to_string(),
        environment: "development".to_string(),
        version: 1,
        created_by_device: "device-1".to_string(),
        created_at: "2026-03-14T00:00:00Z".to_string(),
        manifest_signature: signature,
        key_envelopes: key_envelopes
            .into_iter()
            .map(|item| KeyEnvelope {
                device_id: item.device_id,
                recipient: item.recipient,
                encrypted_key: item.encrypted_key,
            })
            .collect(),
        secrets,
    };
    let known_devices = BTreeMap::from([(known_device.id.clone(), known_device)]);
    let error = decrypt_snapshot(&device, &known_devices, &snapshot).unwrap_err();
    assert!(error
        .to_string()
        .contains("manifest signature verification failed"));
}
