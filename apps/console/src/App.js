import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useEffect, useMemo, useState } from "react";
import { EnvShieldApiClient } from "@envshield/api-client";
const client = new EnvShieldApiClient(window.location.origin);
function readDeviceCode() {
    return new URLSearchParams(window.location.search).get("device_code") ?? "";
}
export default function App() {
    const deviceCode = useMemo(readDeviceCode, []);
    const [actorEmail, setActorEmail] = useState("developer@local.envshield.dev");
    const [approvalState, setApprovalState] = useState(null);
    const [devices, setDevices] = useState([]);
    const [workspaceId, setWorkspaceId] = useState("");
    const [environment, setEnvironment] = useState("development");
    const [status, setStatus] = useState(null);
    const [snapshot, setSnapshot] = useState(null);
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
    return (_jsxs("main", { className: "shell", children: [_jsxs("section", { className: "hero", children: [_jsx("p", { className: "eyebrow", children: "Zero-knowledge developer secrets" }), _jsx("h1", { children: "EnvShield Console" }), _jsx("p", { className: "lede", children: "Approve CLI device flows, inspect registered devices, and watch encrypted snapshots move through your environments without ever materializing a `.env` file." })] }), _jsxs("section", { className: "grid", children: [_jsxs("article", { className: "card accent", children: [_jsx("p", { className: "card-label", children: "Device approval" }), _jsx("h2", { children: "Browser-assisted auth" }), _jsx("p", { className: "card-copy", children: "The CLI opens this console during `shield login`. Approval is manual in development, and ready to be replaced by GitHub OAuth callbacks in production." }), _jsxs("div", { className: "stack", children: [_jsxs("label", { children: ["Device code", _jsx("input", { value: deviceCode, readOnly: true, placeholder: "device_code query param" })] }), _jsxs("label", { children: ["Approve as", _jsx("input", { value: actorEmail, onChange: (event) => setActorEmail(event.target.value) })] }), _jsx("button", { onClick: () => void approveDeviceFlow(), children: "Approve this device flow" }), approvalState ? _jsx("p", { className: "notice", children: approvalState }) : null] })] }), _jsxs("article", { className: "card", children: [_jsx("p", { className: "card-label", children: "Registered devices" }), _jsx("h2", { children: "Known public keys" }), _jsx("p", { className: "card-copy", children: "The control plane stores device metadata and public keys only. Secret values stay encrypted end to end." }), _jsxs("div", { className: "device-list", children: [devices.length === 0 ? _jsx("p", { className: "notice", children: "No devices registered yet." }) : null, devices.map((device) => (_jsxs("div", { className: "device-row", children: [_jsx("strong", { children: device.name }), _jsx("span", { children: device.id }), _jsx("code", { children: device.encryptionPublicKey })] }, device.id)))] })] }), _jsxs("article", { className: "card wide", children: [_jsx("p", { className: "card-label", children: "Snapshot visibility" }), _jsx("h2", { children: "Polling-based sync status" }), _jsx("p", { className: "card-copy", children: "Enter a workspace and environment to inspect the latest snapshot metadata and detect drift before `shield run`." }), _jsxs("div", { className: "toolbar", children: [_jsxs("label", { children: ["Workspace ID", _jsx("input", { value: workspaceId, onChange: (event) => setWorkspaceId(event.target.value) })] }), _jsxs("label", { children: ["Environment", _jsx("input", { value: environment, onChange: (event) => setEnvironment(event.target.value) })] }), _jsx("button", { onClick: () => void refreshEnvironment(), children: "Refresh snapshot state" })] }), _jsxs("div", { className: "status-grid", children: [_jsxs("div", { className: "status-panel", children: [_jsx("span", { children: "Status" }), _jsx("strong", { children: status ? (status.outOfDate ? "Out of date" : "Current") : "Unknown" }), _jsx("small", { children: status
                                                    ? `Local v${status.localVersion} / Latest v${status.latestVersion}`
                                                    : "Query the API to populate sync state." })] }), _jsxs("div", { className: "status-panel", children: [_jsx("span", { children: "Latest snapshot" }), _jsx("strong", { children: snapshot ? `v${snapshot.version}` : "Not loaded" }), _jsx("small", { children: snapshot ? snapshot.createdAt : "No snapshot metadata loaded." })] })] }), snapshot ? (_jsx("pre", { className: "snapshot-preview", children: JSON.stringify({
                                    snapshotId: snapshot.snapshotId,
                                    createdByDevice: snapshot.createdByDevice,
                                    secrets: snapshot.secrets.map((item) => item.name),
                                }, null, 2) })) : null] })] })] }));
}
