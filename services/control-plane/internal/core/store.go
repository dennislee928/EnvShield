package core

type Store interface {
	CreateDeviceAuth(deviceName, verificationURL string) (DeviceAuthSession, error)
	ApproveDeviceAuth(deviceCode, actorEmail string) error
	ExchangeDeviceAuth(deviceCode string) (DeviceToken, error)

	RegisterDevice(actorEmail string, device Device) (Device, error)
	ListDevices(actorEmail string) ([]Device, error)

	CreateWorkspace(actorEmail, name, deviceID string) (Workspace, error)
	GetWorkspace(workspaceID string) (Workspace, error)

	CreateSnapshot(snapshot Snapshot) (Snapshot, error)
	GetLatestSnapshot(workspaceID, environment string) (Snapshot, error)
	GetEnvironmentStatus(workspaceID, environment string, localVersion int) (EnvironmentStatus, error)

	ActorForToken(token string) (string, bool)
}
