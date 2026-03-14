import type { paths } from "./generated";

type FetchLike = typeof fetch;

export type ApiPaths = paths;
export type StartGitHubAuthResponse =
  paths["/v1/auth/github/start"]["post"]["responses"]["200"]["content"]["application/json"];
export type Device =
  paths["/v1/devices"]["get"]["responses"]["200"]["content"]["application/json"]["devices"][number];
export type Workspace =
  paths["/v1/workspaces/{workspaceId}"]["get"]["responses"]["200"]["content"]["application/json"];
export type Snapshot =
  paths["/v1/workspaces/{workspaceId}/environments/{environment}/snapshots/latest"]["get"]["responses"]["200"]["content"]["application/json"];
export type EnvironmentStatus =
  paths["/v1/workspaces/{workspaceId}/environments/{environment}/status"]["get"]["responses"]["200"]["content"]["application/json"];

export class EnvShieldApiClient {
  constructor(
    private readonly baseUrl: string,
    private readonly fetchImpl: FetchLike = fetch,
  ) {}

  async startGitHubAuth(deviceName: string): Promise<StartGitHubAuthResponse> {
    return this.request("/v1/auth/github/start", {
      method: "POST",
      body: JSON.stringify({ deviceName }),
    });
  }

  async listDevices(): Promise<Device[]> {
    const result = await this.request("/v1/devices");
    return result.devices;
  }

  async getWorkspace(workspaceId: string): Promise<Workspace> {
    return this.request(`/v1/workspaces/${workspaceId}`);
  }

  async getLatestSnapshot(workspaceId: string, environment: string): Promise<Snapshot> {
    return this.request(
      `/v1/workspaces/${workspaceId}/environments/${environment}/snapshots/latest`,
    );
  }

  async getEnvironmentStatus(
    workspaceId: string,
    environment: string,
    localVersion: number,
  ): Promise<EnvironmentStatus> {
    const url = new URL(
      `/v1/workspaces/${workspaceId}/environments/${environment}/status`,
      this.baseUrl,
    );
    url.searchParams.set("local_version", String(localVersion));
    const response = await this.fetchImpl(url, {
      headers: { Accept: "application/json" },
    });
    if (!response.ok) {
      throw new Error(`API request failed: ${response.status}`);
    }
    return response.json();
  }

  private async request(path: string, init?: RequestInit) {
    const response = await this.fetchImpl(new URL(path, this.baseUrl), {
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        ...(init?.headers ?? {}),
      },
      ...init,
    });
    if (!response.ok) {
      throw new Error(`API request failed: ${response.status}`);
    }
    return response.json();
  }
}
