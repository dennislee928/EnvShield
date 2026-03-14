package api

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"strconv"
	"strings"

	"github.com/envshield/envshield/services/control-plane/internal/core"
)

type Router struct {
	service *core.Service
	mux     *http.ServeMux
}

func NewRouter(service *core.Service) *Router {
	router := &Router{
		service: service,
		mux:     http.NewServeMux(),
	}
	router.register()
	return router
}

func (r *Router) Handler() http.Handler {
	return r.mux
}

func (r *Router) register() {
	r.mux.HandleFunc("GET /healthz", func(w http.ResponseWriter, _ *http.Request) {
		writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
	})
	r.mux.HandleFunc("POST /v1/auth/github/start", r.handleStartGitHubAuth)
	r.mux.HandleFunc("POST /v1/auth/device/approve", r.handleApproveDeviceAuth)
	r.mux.HandleFunc("POST /v1/auth/device/exchange", r.handleExchangeDeviceAuth)
	r.mux.HandleFunc("POST /v1/devices", r.handleRegisterDevice)
	r.mux.HandleFunc("GET /v1/devices", r.handleListDevices)
	r.mux.HandleFunc("POST /v1/workspaces", r.handleCreateWorkspace)
	r.mux.HandleFunc("GET /v1/workspaces/{workspaceId}", r.handleGetWorkspace)
	r.mux.HandleFunc("POST /v1/snapshots", r.handleCreateSnapshot)
	r.mux.HandleFunc("GET /v1/workspaces/{workspaceId}/environments/{environment}/snapshots/latest", r.handleGetLatestSnapshot)
	r.mux.HandleFunc("GET /v1/workspaces/{workspaceId}/environments/{environment}/status", r.handleEnvironmentStatus)
}

func (r *Router) handleStartGitHubAuth(w http.ResponseWriter, req *http.Request) {
	var payload struct {
		DeviceName string `json:"deviceName"`
	}
	if err := json.NewDecoder(req.Body).Decode(&payload); err != nil {
		writeError(w, http.StatusBadRequest, err)
		return
	}
	response, err := r.service.StartGitHubAuth(payload.DeviceName)
	if err != nil {
		writeError(w, http.StatusInternalServerError, err)
		return
	}
	writeJSON(w, http.StatusOK, response)
}

func (r *Router) handleApproveDeviceAuth(w http.ResponseWriter, req *http.Request) {
	var payload struct {
		DeviceCode string `json:"deviceCode"`
		ActorEmail string `json:"actorEmail"`
	}
	if err := json.NewDecoder(req.Body).Decode(&payload); err != nil {
		writeError(w, http.StatusBadRequest, err)
		return
	}
	if err := r.service.ApproveDeviceAuth(payload.DeviceCode, payload.ActorEmail); err != nil {
		writeDomainError(w, err)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

func (r *Router) handleExchangeDeviceAuth(w http.ResponseWriter, req *http.Request) {
	var payload struct {
		DeviceCode string `json:"deviceCode"`
	}
	if err := json.NewDecoder(req.Body).Decode(&payload); err != nil {
		writeError(w, http.StatusBadRequest, err)
		return
	}
	token, err := r.service.ExchangeDeviceAuth(payload.DeviceCode)
	if err != nil {
		if errors.Is(err, core.ErrPendingApproval) {
			writeError(w, http.StatusConflict, err)
			return
		}
		writeDomainError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, token)
}

func (r *Router) handleRegisterDevice(w http.ResponseWriter, req *http.Request) {
	var payload struct {
		Name                string `json:"name"`
		EncryptionPublicKey string `json:"encryptionPublicKey"`
		SigningPublicKey    string `json:"signingPublicKey"`
	}
	if err := json.NewDecoder(req.Body).Decode(&payload); err != nil {
		writeError(w, http.StatusBadRequest, err)
		return
	}
	device, err := r.service.RegisterDevice(actorEmail(req, r.service), core.Device{
		Name:                payload.Name,
		EncryptionPublicKey: payload.EncryptionPublicKey,
		SigningPublicKey:    payload.SigningPublicKey,
	})
	if err != nil {
		writeError(w, http.StatusInternalServerError, err)
		return
	}
	writeJSON(w, http.StatusCreated, device)
}

func (r *Router) handleListDevices(w http.ResponseWriter, req *http.Request) {
	devices, err := r.service.ListDevices(actorEmail(req, r.service))
	if err != nil {
		writeError(w, http.StatusInternalServerError, err)
		return
	}
	writeJSON(w, http.StatusOK, map[string]any{"devices": devices})
}

func (r *Router) handleCreateWorkspace(w http.ResponseWriter, req *http.Request) {
	var payload struct {
		Name     string `json:"name"`
		DeviceID string `json:"deviceId"`
	}
	if err := json.NewDecoder(req.Body).Decode(&payload); err != nil {
		writeError(w, http.StatusBadRequest, err)
		return
	}
	workspace, err := r.service.CreateWorkspace(actorEmail(req, r.service), payload.Name, payload.DeviceID)
	if err != nil {
		writeError(w, http.StatusInternalServerError, err)
		return
	}
	writeJSON(w, http.StatusCreated, workspace)
}

func (r *Router) handleGetWorkspace(w http.ResponseWriter, req *http.Request) {
	workspaceID := req.PathValue("workspaceId")
	if workspaceID == "" {
		http.NotFound(w, req)
		return
	}
	workspace, err := r.service.GetWorkspace(workspaceID)
	if err != nil {
		writeDomainError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, workspace)
}

func (r *Router) handleCreateSnapshot(w http.ResponseWriter, req *http.Request) {
	var payload core.Snapshot
	if err := json.NewDecoder(req.Body).Decode(&payload); err != nil {
		writeError(w, http.StatusBadRequest, err)
		return
	}
	snapshot, err := r.service.CreateSnapshot(payload)
	if err != nil {
		writeError(w, http.StatusInternalServerError, err)
		return
	}
	writeJSON(w, http.StatusCreated, snapshot)
}

func (r *Router) handleGetLatestSnapshot(w http.ResponseWriter, req *http.Request) {
	workspaceID := req.PathValue("workspaceId")
	environment := req.PathValue("environment")
	snapshot, err := r.service.GetLatestSnapshot(workspaceID, environment)
	if err != nil {
		writeDomainError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, snapshot)
}

func (r *Router) handleEnvironmentStatus(w http.ResponseWriter, req *http.Request) {
	workspaceID := req.PathValue("workspaceId")
	environment := req.PathValue("environment")
	localVersion, _ := strconv.Atoi(req.URL.Query().Get("local_version"))
	status, err := r.service.GetEnvironmentStatus(workspaceID, environment, localVersion)
	if err != nil {
		writeDomainError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, status)
}

func actorEmail(req *http.Request, service *core.Service) string {
	auth := strings.TrimPrefix(req.Header.Get("Authorization"), "Bearer ")
	if auth == "" {
		return "developer@local.envshield.dev"
	}
	if actor, ok := service.ActorForToken(auth); ok {
		return actor
	}
	return "developer@local.envshield.dev"
}

func writeJSON(w http.ResponseWriter, status int, payload any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(payload); err != nil {
		http.Error(w, fmt.Sprintf(`{"error":"%s"}`, err), http.StatusInternalServerError)
	}
}

func writeError(w http.ResponseWriter, status int, err error) {
	writeJSON(w, status, map[string]string{"error": err.Error()})
}

func writeDomainError(w http.ResponseWriter, err error) {
	if errors.Is(err, core.ErrNotFound) {
		writeError(w, http.StatusNotFound, err)
		return
	}
	writeError(w, http.StatusInternalServerError, err)
}
