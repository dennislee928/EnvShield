package core

import (
	"errors"
	"fmt"
	"time"
)

var (
	ErrNotFound        = errors.New("not found")
	ErrPendingApproval = errors.New("device flow pending approval")
)

type Service struct {
	store      Store
	publicURL  string
	authTTL    time.Duration
	defaultOrg string
}

func NewService(store Store, publicURL string) *Service {
	return &Service{
		store:      store,
		publicURL:  publicURL,
		authTTL:    10 * time.Minute,
		defaultOrg: "personal",
	}
}

func (s *Service) StartGitHubAuth(deviceName string) (StartDeviceAuthResponse, error) {
	verificationURL := fmt.Sprintf("%s/approve", s.publicURL)
	session, err := s.store.CreateDeviceAuth(deviceName, verificationURL)
	if err != nil {
		return StartDeviceAuthResponse{}, err
	}
	return StartDeviceAuthResponse{
		DeviceCode:      session.DeviceCode,
		UserCode:        session.UserCode,
		VerificationURL: fmt.Sprintf("%s?device_code=%s", session.VerificationURL, session.DeviceCode),
		ExpiresIn:       int(s.authTTL.Seconds()),
	}, nil
}

func (s *Service) ApproveDeviceAuth(deviceCode, actorEmail string) error {
	return s.store.ApproveDeviceAuth(deviceCode, actorEmail)
}

func (s *Service) ExchangeDeviceAuth(deviceCode string) (DeviceToken, error) {
	return s.store.ExchangeDeviceAuth(deviceCode)
}

func (s *Service) RegisterDevice(actorEmail string, device Device) (Device, error) {
	return s.store.RegisterDevice(actorEmail, device)
}

func (s *Service) ListDevices(actorEmail string) ([]Device, error) {
	return s.store.ListDevices(actorEmail)
}

func (s *Service) CreateWorkspace(actorEmail, name, deviceID string) (Workspace, error) {
	return s.store.CreateWorkspace(actorEmail, name, deviceID)
}

func (s *Service) GetWorkspace(workspaceID string) (Workspace, error) {
	return s.store.GetWorkspace(workspaceID)
}

func (s *Service) CreateSnapshot(snapshot Snapshot) (Snapshot, error) {
	return s.store.CreateSnapshot(snapshot)
}

func (s *Service) GetLatestSnapshot(workspaceID, environment string) (Snapshot, error) {
	return s.store.GetLatestSnapshot(workspaceID, environment)
}

func (s *Service) GetEnvironmentStatus(workspaceID, environment string, localVersion int) (EnvironmentStatus, error) {
	return s.store.GetEnvironmentStatus(workspaceID, environment, localVersion)
}

func (s *Service) ActorForToken(token string) (string, bool) {
	return s.store.ActorForToken(token)
}
