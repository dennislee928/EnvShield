package store

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"sort"
	"sync"
	"time"

	"github.com/envshield/envshield/services/control-plane/internal/core"
	"github.com/google/uuid"
)

type MemoryStore struct {
	mu          sync.RWMutex
	auth        map[string]core.DeviceAuthSession
	tokens      map[string]string
	devices     map[string][]core.Device
	workspaces  map[string]core.Workspace
	snapshots   map[string][]core.Snapshot
	workspaceBy map[string][]string
}

func NewMemoryStore() *MemoryStore {
	return &MemoryStore{
		auth:        make(map[string]core.DeviceAuthSession),
		tokens:      make(map[string]string),
		devices:     make(map[string][]core.Device),
		workspaces:  make(map[string]core.Workspace),
		snapshots:   make(map[string][]core.Snapshot),
		workspaceBy: make(map[string][]string),
	}
}

func (s *MemoryStore) CreateDeviceAuth(deviceName, verificationURL string) (core.DeviceAuthSession, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	deviceCode := randomToken(16)
	session := core.DeviceAuthSession{
		DeviceCode:      deviceCode,
		UserCode:        randomToken(4),
		VerificationURL: verificationURL,
		DeviceName:      deviceName,
		ExpiresAt:       time.Now().Add(10 * time.Minute),
	}
	s.auth[deviceCode] = session
	return session, nil
}

func (s *MemoryStore) ApproveDeviceAuth(deviceCode, actorEmail string) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	session, ok := s.auth[deviceCode]
	if !ok {
		return core.ErrNotFound
	}
	session.ActorEmail = actorEmail
	session.Approved = true
	s.auth[deviceCode] = session
	return nil
}

func (s *MemoryStore) ExchangeDeviceAuth(deviceCode string) (core.DeviceToken, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	session, ok := s.auth[deviceCode]
	if !ok {
		return core.DeviceToken{}, core.ErrNotFound
	}
	if !session.Approved {
		return core.DeviceToken{}, core.ErrPendingApproval
	}
	token := fmt.Sprintf("esh_%s", randomToken(16))
	s.tokens[token] = session.ActorEmail
	delete(s.auth, deviceCode)
	return core.DeviceToken{
		Token:      token,
		ActorEmail: session.ActorEmail,
	}, nil
}

func (s *MemoryStore) RegisterDevice(actorEmail string, device core.Device) (core.Device, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	if actorEmail == "" {
		actorEmail = "developer@local.envshield.dev"
	}
	device.ID = uuid.NewString()
	device.CreatedAt = time.Now().UTC()
	s.devices[actorEmail] = append(s.devices[actorEmail], device)
	return device, nil
}

func (s *MemoryStore) ListDevices(actorEmail string) ([]core.Device, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	return append([]core.Device(nil), s.devices[actorEmail]...), nil
}

func (s *MemoryStore) CreateWorkspace(actorEmail, name, deviceID string) (core.Workspace, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	if actorEmail == "" {
		actorEmail = "developer@local.envshield.dev"
	}
	workspace := core.Workspace{
		ID:        uuid.NewString(),
		Name:      name,
		CreatedAt: time.Now().UTC(),
		Members: []core.Member{
			{
				ID:    uuid.NewString(),
				Email: actorEmail,
				Role:  "owner",
			},
		},
	}
	s.workspaces[workspace.ID] = workspace
	s.workspaceBy[actorEmail] = append(s.workspaceBy[actorEmail], workspace.ID)
	return workspace, nil
}

func (s *MemoryStore) GetWorkspace(workspaceID string) (core.Workspace, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	workspace, ok := s.workspaces[workspaceID]
	if !ok {
		return core.Workspace{}, core.ErrNotFound
	}
	return workspace, nil
}

func (s *MemoryStore) CreateSnapshot(snapshot core.Snapshot) (core.Snapshot, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	key := snapshotKey(snapshot.WorkspaceID, snapshot.Environment)
	existing := s.snapshots[key]
	snapshot.SnapshotID = uuid.NewString()
	snapshot.CreatedAt = time.Now().UTC()
	snapshot.Version = len(existing) + 1
	s.snapshots[key] = append(existing, snapshot)
	return snapshot, nil
}

func (s *MemoryStore) GetLatestSnapshot(workspaceID, environment string) (core.Snapshot, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	snapshots := s.snapshots[snapshotKey(workspaceID, environment)]
	if len(snapshots) == 0 {
		return core.Snapshot{}, core.ErrNotFound
	}
	return snapshots[len(snapshots)-1], nil
}

func (s *MemoryStore) GetEnvironmentStatus(workspaceID, environment string, localVersion int) (core.EnvironmentStatus, error) {
	snapshot, err := s.GetLatestSnapshot(workspaceID, environment)
	if err != nil {
		if err == core.ErrNotFound {
			return core.EnvironmentStatus{
				WorkspaceID:   workspaceID,
				Environment:   environment,
				LatestVersion: 0,
				LocalVersion:  localVersion,
				OutOfDate:     false,
			}, nil
		}
		return core.EnvironmentStatus{}, err
	}
	return core.EnvironmentStatus{
		WorkspaceID:   workspaceID,
		Environment:   environment,
		LatestVersion: snapshot.Version,
		LocalVersion:  localVersion,
		OutOfDate:     localVersion < snapshot.Version,
	}, nil
}

func (s *MemoryStore) ActorForToken(token string) (string, bool) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	actor, ok := s.tokens[token]
	return actor, ok
}

func (s *MemoryStore) SnapshotKeys() []string {
	s.mu.RLock()
	defer s.mu.RUnlock()

	keys := make([]string, 0, len(s.snapshots))
	for key := range s.snapshots {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	return keys
}

func snapshotKey(workspaceID, environment string) string {
	return fmt.Sprintf("%s:%s", workspaceID, environment)
}

func randomToken(size int) string {
	buffer := make([]byte, size)
	if _, err := rand.Read(buffer); err != nil {
		panic(err)
	}
	return hex.EncodeToString(buffer)
}
