package api_test

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"

	"github.com/envshield/envshield/services/control-plane/internal/api"
	"github.com/envshield/envshield/services/control-plane/internal/core"
	"github.com/envshield/envshield/services/control-plane/internal/store"
	"github.com/envshield/envshield/services/control-plane/internal/testutil"
)

func TestDeviceFlowAndSnapshotLifecycle(t *testing.T) {
	server := testutil.NewTestServer()
	defer server.Close()

	deviceAuth := requestJSON(t, server.URL+"/v1/auth/github/start", http.MethodPost, map[string]string{
		"deviceName": "Dennis Laptop",
	}, "")
	deviceCode := deviceAuth["deviceCode"].(string)

	requestStatus(t, server.URL+"/v1/auth/device/approve", http.MethodPost, map[string]string{
		"deviceCode": deviceCode,
		"actorEmail": "dennis@example.com",
	}, "", http.StatusNoContent)

	tokenResponse := requestJSON(t, server.URL+"/v1/auth/device/exchange", http.MethodPost, map[string]string{
		"deviceCode": deviceCode,
	}, "")
	token := tokenResponse["token"].(string)

	device := requestJSON(t, server.URL+"/v1/devices", http.MethodPost, map[string]string{
		"name":                "Dennis Laptop",
		"encryptionPublicKey": "age1devicekey",
		"signingPublicKey":    "ed25519-device-key",
	}, token)

	workspace := requestJSON(t, server.URL+"/v1/workspaces", http.MethodPost, map[string]string{
		"name":     "EnvShield",
		"deviceId": device["id"].(string),
	}, token)

	snapshot := requestJSON(t, server.URL+"/v1/snapshots", http.MethodPost, map[string]any{
		"workspaceId":       workspace["id"].(string),
		"environment":       "production",
		"createdByDevice":   device["id"].(string),
		"manifestSignature": "signed-manifest",
		"keyEnvelopes": []map[string]string{{
			"deviceId":     device["id"].(string),
			"recipient":    "age1devicekey",
			"encryptedKey": "wrapped-key",
		}},
		"secrets": []map[string]string{{
			"name":          "DATABASE_URL",
			"ciphertext":    "secret-ciphertext",
			"nonce":         "nonce",
			"aadHash":       "aad-hash",
			"valueChecksum": "checksum",
		}},
	}, token)

	if got := int(snapshot["version"].(float64)); got != 1 {
		t.Fatalf("expected snapshot version 1, got %d", got)
	}

	status := requestJSON(
		t,
		server.URL+"/v1/workspaces/"+workspace["id"].(string)+"/environments/production/status?local_version=0",
		http.MethodGet,
		nil,
		token,
	)
	if outOfDate := status["outOfDate"].(bool); !outOfDate {
		t.Fatal("expected local version to be out of date")
	}
}

func TestRouterServesEmbeddedConsoleAssets(t *testing.T) {
	tempDir := t.TempDir()
	if err := os.WriteFile(filepath.Join(tempDir, "index.html"), []byte("EnvShield Console"), 0o644); err != nil {
		t.Fatalf("write index: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(tempDir, "assets"), 0o755); err != nil {
		t.Fatalf("mkdir assets: %v", err)
	}
	if err := os.WriteFile(filepath.Join(tempDir, "assets", "app.js"), []byte("asset-body"), 0o644); err != nil {
		t.Fatalf("write asset: %v", err)
	}

	service := core.NewService(store.NewMemoryStore(), "http://localhost:8080")
	router := api.NewRouter(service, tempDir)
	server := httptest.NewServer(router.Handler())
	defer server.Close()

	response, err := http.Get(server.URL + "/approve?device_code=test")
	if err != nil {
		t.Fatalf("get approve page: %v", err)
	}
	defer response.Body.Close()

	body, _ := io.ReadAll(response.Body)
	if response.StatusCode != http.StatusOK {
		t.Fatalf("expected 200, got %d", response.StatusCode)
	}
	if !bytes.Contains(body, []byte("EnvShield Console")) {
		t.Fatalf("expected SPA shell, got %q", body)
	}

	response, err = http.Get(server.URL + "/assets/app.js")
	if err != nil {
		t.Fatalf("get asset: %v", err)
	}
	defer response.Body.Close()

	body, _ = io.ReadAll(response.Body)
	if response.StatusCode != http.StatusOK {
		t.Fatalf("expected asset 200, got %d", response.StatusCode)
	}
	if !bytes.Equal(body, []byte("asset-body")) {
		t.Fatalf("unexpected asset body: %q", body)
	}
}

func requestJSON(t *testing.T, url, method string, payload any, token string) map[string]any {
	t.Helper()

	body := bytes.NewBuffer(nil)
	if payload != nil {
		if err := json.NewEncoder(body).Encode(payload); err != nil {
			t.Fatalf("encode request: %v", err)
		}
	}

	req, err := http.NewRequest(method, url, body)
	if err != nil {
		t.Fatalf("new request: %v", err)
	}
	req.Header.Set("Content-Type", "application/json")
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("do request: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 400 {
		payload, _ := io.ReadAll(resp.Body)
		t.Fatalf("unexpected status %d: %s", resp.StatusCode, payload)
	}

	var result map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		t.Fatalf("decode response: %v", err)
	}
	return result
}

func requestStatus(t *testing.T, url, method string, payload any, token string, want int) {
	t.Helper()

	body := bytes.NewBuffer(nil)
	if payload != nil {
		if err := json.NewEncoder(body).Encode(payload); err != nil {
			t.Fatalf("encode request: %v", err)
		}
	}

	req, err := http.NewRequest(method, url, body)
	if err != nil {
		t.Fatalf("new request: %v", err)
	}
	req.Header.Set("Content-Type", "application/json")
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("do request: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != want {
		payload, _ := io.ReadAll(resp.Body)
		t.Fatalf("unexpected status %d: %s", resp.StatusCode, payload)
	}
}
