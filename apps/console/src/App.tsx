import { useEffect, useMemo, useState } from "react";
import { EnvShieldApiClient, type Device, type EnvironmentStatus, type Snapshot } from "@envshield/api-client";

const client = new EnvShieldApiClient(
  typeof window !== "undefined" ? window.location.origin : "http://127.0.0.1:8080",
);

function readDeviceCode() {
  if (typeof window === "undefined") {
    return "";
  }
  return new URLSearchParams(window.location.search).get("device_code") ?? "";
}

export default function App() {
  const deviceCode = useMemo(readDeviceCode, []);
  const [actorEmail, setActorEmail] = useState("developer@local.envshield.dev");
  const [approvalState, setApprovalState] = useState<string | null>(null);
  const [devices, setDevices] = useState<Device[]>([]);
  const [workspaceId, setWorkspaceId] = useState("");
  const [environment, setEnvironment] = useState("development");
  const [status, setStatus] = useState<EnvironmentStatus | null>(null);
  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);

  useEffect(() => {
    void client.listDevices().then(setDevices).catch(() => setDevices([]));
  }, []);

  async function approveDeviceFlow() {
    if (!deviceCode) {
      setApprovalState("No device flow is attached to this page.");
      return;
    }
    const response = await fetch("/v1/auth/device/approve", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify({
        deviceCode,
        actorEmail,
      }),
    });
    if (!response.ok) {
      setApprovalState("Approval failed. Check the control plane logs.");
      return;
    }
    setApprovalState(`Approved ${deviceCode} for ${actorEmail}. You can return to the CLI now.`);
  }

  async function refreshEnvironment() {
    if (!workspaceId) {
      setStatus(null);
      setSnapshot(null);
      return;
    }
    const [nextStatus, nextSnapshot] = await Promise.all([
      client.getEnvironmentStatus(workspaceId, environment, 0),
      client.getLatestSnapshot(workspaceId, environment),
    ]);
    setStatus(nextStatus);
    setSnapshot(nextSnapshot);
  }

  return (
    <main className="shell">
      <section className="hero">
        <p className="eyebrow">Zero-knowledge developer secrets</p>
        <h1>EnvShield Console</h1>
        <p className="lede">
          Approve CLI device flows, inspect registered devices, and watch encrypted snapshots move
          through your environments without ever materializing a `.env` file.
        </p>
      </section>

      <section className="grid">
        <article className="card accent">
          <p className="card-label">Device approval</p>
          <h2>Browser-assisted auth</h2>
          <p className="card-copy">
            The CLI opens this console during `shield login`. Approval is manual in development,
            and ready to be replaced by GitHub OAuth callbacks in production.
          </p>
          <div className="stack">
            <label>
              Device code
              <input value={deviceCode} readOnly placeholder="device_code query param" />
            </label>
            <label>
              Approve as
              <input value={actorEmail} onChange={(event) => setActorEmail(event.target.value)} />
            </label>
            <button onClick={() => void approveDeviceFlow()}>Approve this device flow</button>
            {approvalState ? <p className="notice">{approvalState}</p> : null}
          </div>
        </article>

        <article className="card">
          <p className="card-label">Registered devices</p>
          <h2>Known public keys</h2>
          <p className="card-copy">
            The control plane stores device metadata and public keys only. Secret values stay
            encrypted end to end.
          </p>
          <div className="device-list">
            {devices.length === 0 ? <p className="notice">No devices registered yet.</p> : null}
            {devices.map((device) => (
              <div className="device-row" key={device.id}>
                <strong>{device.name}</strong>
                <span>{device.id}</span>
                <code>{device.encryptionPublicKey}</code>
              </div>
            ))}
          </div>
        </article>

        <article className="card wide">
          <p className="card-label">Snapshot visibility</p>
          <h2>Polling-based sync status</h2>
          <p className="card-copy">
            Enter a workspace and environment to inspect the latest snapshot metadata and detect
            drift before `shield run`.
          </p>
          <div className="toolbar">
            <label>
              Workspace ID
              <input value={workspaceId} onChange={(event) => setWorkspaceId(event.target.value)} />
            </label>
            <label>
              Environment
              <input value={environment} onChange={(event) => setEnvironment(event.target.value)} />
            </label>
            <button onClick={() => void refreshEnvironment()}>Refresh snapshot state</button>
          </div>
          <div className="status-grid">
            <div className="status-panel">
              <span>Status</span>
              <strong>{status ? (status.outOfDate ? "Out of date" : "Current") : "Unknown"}</strong>
              <small>
                {status
                  ? `Local v${status.localVersion} / Latest v${status.latestVersion}`
                  : "Query the API to populate sync state."}
              </small>
            </div>
            <div className="status-panel">
              <span>Latest snapshot</span>
              <strong>{snapshot ? `v${snapshot.version}` : "Not loaded"}</strong>
              <small>{snapshot ? snapshot.createdAt : "No snapshot metadata loaded."}</small>
            </div>
          </div>
          {snapshot ? (
            <pre className="snapshot-preview">
              {JSON.stringify(
                {
                  snapshotId: snapshot.snapshotId,
                  createdByDevice: snapshot.createdByDevice,
                  secrets: snapshot.secrets.map(
                    (item: Snapshot["secrets"][number]) => item.name,
                  ),
                },
                null,
                2,
              )}
            </pre>
          ) : null}
        </article>
      </section>
    </main>
  );
}
