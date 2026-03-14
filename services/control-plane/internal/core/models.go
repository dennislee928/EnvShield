package core

import "time"

type Member struct {
	ID    string `json:"id"`
	Email string `json:"email"`
	Role  string `json:"role"`
}

type Workspace struct {
	ID        string    `json:"id"`
	Name      string    `json:"name"`
	CreatedAt time.Time `json:"createdAt"`
	Members   []Member  `json:"members"`
}

type Device struct {
	ID                  string    `json:"id"`
	Name                string    `json:"name"`
	EncryptionPublicKey string    `json:"encryptionPublicKey"`
	SigningPublicKey    string    `json:"signingPublicKey"`
	CreatedAt           time.Time `json:"createdAt"`
}

type KeyEnvelope struct {
	DeviceID     string `json:"deviceId"`
	Recipient    string `json:"recipient"`
	EncryptedKey string `json:"encryptedKey"`
}

type SecretItem struct {
	Name          string `json:"name"`
	Ciphertext    string `json:"ciphertext"`
	Nonce         string `json:"nonce"`
	AADHash       string `json:"aadHash"`
	ValueChecksum string `json:"valueChecksum"`
}

type Snapshot struct {
	SnapshotID        string        `json:"snapshotId"`
	WorkspaceID       string        `json:"workspaceId"`
	Environment       string        `json:"environment"`
	Version           int           `json:"version"`
	CreatedByDevice   string        `json:"createdByDevice"`
	CreatedAt         time.Time     `json:"createdAt"`
	ManifestSignature string        `json:"manifestSignature"`
	KeyEnvelopes      []KeyEnvelope `json:"keyEnvelopes"`
	Secrets           []SecretItem  `json:"secrets"`
}

type EnvironmentStatus struct {
	WorkspaceID   string `json:"workspaceId"`
	Environment   string `json:"environment"`
	LatestVersion int    `json:"latestVersion"`
	LocalVersion  int    `json:"localVersion"`
	OutOfDate     bool   `json:"outOfDate"`
}

type DeviceAuthSession struct {
	DeviceCode      string
	UserCode        string
	VerificationURL string
	ExpiresAt       time.Time
	DeviceName      string
	ActorEmail      string
	Approved        bool
}

type StartDeviceAuthResponse struct {
	DeviceCode      string `json:"deviceCode"`
	UserCode        string `json:"userCode"`
	VerificationURL string `json:"verificationUrl"`
	ExpiresIn       int    `json:"expiresIn"`
}

type DeviceToken struct {
	Token      string `json:"token"`
	ActorEmail string `json:"actorEmail"`
}
