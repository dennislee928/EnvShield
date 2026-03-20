from __future__ import annotations

import json
import random
import string
import time
from typing import Any

import requests
from robot.api.deco import keyword, library


@library(scope="GLOBAL", auto_keywords=False)
class EnvShieldApiLibrary:
    def __init__(self, base_url: str = "") -> None:
        self.base_url = (base_url or "").rstrip("/")
        self.http = requests.Session()
        self.context: dict[str, Any] = {}

    def _require_base_url(self) -> None:
        if not self.base_url:
            raise AssertionError(
                "BASE_URL is required. Pass --variable BASE_URL:https://<host> to robot."
            )

    def _bool(self, value: Any) -> bool:
        if isinstance(value, bool):
            return value
        return str(value).strip().lower() in {"1", "true", "yes", "on"}

    def _random_suffix(self, length: int = 6) -> str:
        alphabet = string.ascii_lowercase + string.digits
        return "".join(random.choice(alphabet) for _ in range(length))

    def _request(
        self,
        method: str,
        path: str,
        *,
        token: str = "",
        json_body: Any | None = None,
        data: str | None = None,
        expected_status: int | None = None,
    ) -> requests.Response:
        self._require_base_url()
        headers = {"Content-Type": "application/json"}
        if token:
            headers["Authorization"] = f"Bearer {token}"
        response = self.http.request(
            method=method,
            url=f"{self.base_url}{path}",
            json=json_body,
            data=data,
            headers=headers,
            timeout=30,
        )
        if expected_status is not None and response.status_code != expected_status:
            raise AssertionError(
                f"{method} {path} returned {response.status_code}, expected {expected_status}: {response.text}"
            )
        return response

    def _response_json(self, response: requests.Response) -> Any:
        try:
            return response.json()
        except json.JSONDecodeError as exc:
            raise AssertionError(f"Expected JSON response, got: {response.text}") from exc

    def _resolve_field(self, payload: Any, field_path: str) -> Any:
        value = payload
        for part in field_path.split("."):
            if isinstance(value, dict):
                value = value[part]
                continue
            raise AssertionError(f"Cannot resolve field '{field_path}' in payload: {payload}")
        return value

    @keyword("Create Unique Identity Data")
    def create_unique_identity_data(self, force_new: bool = False) -> dict[str, Any]:
        if self.context and not self._bool(force_new):
            return dict(self.context)

        suffix = f"{int(time.time())}-{self._random_suffix()}"
        self.context = {
            "run_id": suffix,
            "device_name": f"robot-device-{suffix}",
            "actor_email": f"robot-{suffix}@example.com",
            "workspace_name": f"robot-workspace-{suffix}",
            "environment": f"robot-env-{suffix}",
            "empty_environment": f"robot-empty-{suffix}",
            "encryption_public_key": f"age1robot{suffix.replace('-', '')}",
            "signing_public_key": f"ed25519-robot-{suffix}",
            "device_code": "",
            "user_code": "",
            "verification_url": "",
            "token": "",
            "device_id": "",
            "workspace_id": "",
            "snapshot_id": "",
            "snapshot_version": 0,
        }
        return dict(self.context)

    @keyword("Get Context Value")
    def get_context_value(self, key: str, default: Any = "") -> Any:
        return self.context.get(key, default)

    @keyword("Start Device Auth")
    def start_device_auth(self, force_new: bool = False) -> dict[str, Any]:
        self.create_unique_identity_data()
        if self.context.get("device_code") and not self._bool(force_new):
            return {
                "deviceCode": self.context["device_code"],
                "userCode": self.context["user_code"],
                "verificationUrl": self.context["verification_url"],
                "expiresIn": 600,
            }

        response = self._request(
            "POST",
            "/v1/auth/github/start",
            json_body={"deviceName": self.context["device_name"]},
            expected_status=200,
        )
        payload = self._response_json(response)
        self.context["device_code"] = payload["deviceCode"]
        self.context["user_code"] = payload["userCode"]
        self.context["verification_url"] = payload["verificationUrl"]
        return payload

    @keyword("Approve Device Auth")
    def approve_device_auth(
        self,
        device_code: str = "",
        actor_email: str = "",
        expected_status: int = 204,
    ) -> int:
        self.create_unique_identity_data()
        resolved_device_code = device_code or self.context.get("device_code", "")
        resolved_actor_email = actor_email or self.context.get("actor_email", "")
        response = self._request(
            "POST",
            "/v1/auth/device/approve",
            json_body={
                "deviceCode": resolved_device_code,
                "actorEmail": resolved_actor_email,
            },
            expected_status=expected_status,
        )
        return response.status_code

    @keyword("Exchange Device Token")
    def exchange_device_token(
        self,
        device_code: str = "",
        expected_status: int = 200,
        store_token: bool = True,
    ) -> dict[str, Any]:
        self.create_unique_identity_data()
        resolved_device_code = device_code or self.context.get("device_code", "")
        response = self._request(
            "POST",
            "/v1/auth/device/exchange",
            json_body={"deviceCode": resolved_device_code},
            expected_status=expected_status,
        )
        payload = self._response_json(response)
        if self._bool(store_token) and expected_status == 200:
            self.context["token"] = payload["token"]
        return payload

    @keyword("Register Device")
    def register_device(self, force_new: bool = False) -> dict[str, Any]:
        self.create_unique_identity_data()
        if self.context.get("device_id") and not self._bool(force_new):
            return {
                "id": self.context["device_id"],
                "name": self.context["device_name"],
                "encryptionPublicKey": self.context["encryption_public_key"],
                "signingPublicKey": self.context["signing_public_key"],
            }

        response = self._request(
            "POST",
            "/v1/devices",
            token=self.context.get("token", ""),
            json_body={
                "name": self.context["device_name"],
                "encryptionPublicKey": self.context["encryption_public_key"],
                "signingPublicKey": self.context["signing_public_key"],
            },
            expected_status=201,
        )
        payload = self._response_json(response)
        self.context["device_id"] = payload["id"]
        return payload

    @keyword("Create Workspace")
    def create_workspace(self, force_new: bool = False) -> dict[str, Any]:
        self.create_unique_identity_data()
        if self.context.get("workspace_id") and not self._bool(force_new):
            return {
                "id": self.context["workspace_id"],
                "name": self.context["workspace_name"],
            }

        response = self._request(
            "POST",
            "/v1/workspaces",
            token=self.context.get("token", ""),
            json_body={
                "name": self.context["workspace_name"],
                "deviceId": self.context["device_id"],
            },
            expected_status=201,
        )
        payload = self._response_json(response)
        self.context["workspace_id"] = payload["id"]
        return payload

    @keyword("Create Snapshot")
    def create_snapshot(
        self,
        environment: str = "",
        force_new: bool = False,
    ) -> dict[str, Any]:
        self.create_unique_identity_data()
        resolved_environment = environment or self.context["environment"]
        if (
            resolved_environment == self.context["environment"]
            and self.context.get("snapshot_id")
            and not self._bool(force_new)
        ):
            return {
                "snapshotId": self.context["snapshot_id"],
                "version": self.context["snapshot_version"],
                "environment": self.context["environment"],
            }

        response = self._request(
            "POST",
            "/v1/snapshots",
            token=self.context.get("token", ""),
            json_body={
                "workspaceId": self.context["workspace_id"],
                "environment": resolved_environment,
                "createdByDevice": self.context["device_id"],
                "manifestSignature": f"robot-signature-{self.context['run_id']}",
                "keyEnvelopes": [
                    {
                        "deviceId": self.context["device_id"],
                        "recipient": self.context["encryption_public_key"],
                        "encryptedKey": f"wrapped-{self.context['run_id']}",
                    }
                ],
                "secrets": [
                    {
                        "name": "DATABASE_URL",
                        "ciphertext": f"ciphertext-{self.context['run_id']}",
                        "nonce": f"nonce-{self.context['run_id']}",
                        "aadHash": f"aad-{self.context['run_id']}",
                        "valueChecksum": f"checksum-{self.context['run_id']}",
                    }
                ],
            },
            expected_status=201,
        )
        payload = self._response_json(response)
        if resolved_environment == self.context["environment"]:
            self.context["snapshot_id"] = payload["snapshotId"]
            self.context["snapshot_version"] = payload["version"]
        return payload

    @keyword("Assert Json Field")
    def assert_json_field(self, payload: Any, field_path: str, expected: Any) -> None:
        if isinstance(payload, requests.Response):
            payload = self._response_json(payload)
        elif isinstance(payload, str):
            payload = json.loads(payload)

        actual = self._resolve_field(payload, field_path)

        if expected == "__nonempty__":
            if actual in ("", None, [], {}):
                raise AssertionError(f"Expected '{field_path}' to be non-empty, got {actual!r}")
            return

        if expected == "__integer__":
            if not isinstance(actual, int):
                raise AssertionError(f"Expected '{field_path}' to be an integer, got {actual!r}")
            return

        if isinstance(actual, int) and isinstance(expected, str) and expected.isdigit():
            expected = int(expected)

        if actual != expected:
            raise AssertionError(
                f"Unexpected value for '{field_path}': expected {expected!r}, got {actual!r}"
            )

    @keyword("Assert Error Response")
    def assert_error_response(
        self,
        response: requests.Response,
        expected_status: int,
        expected_error: str = "",
    ) -> dict[str, Any]:
        if response.status_code != int(expected_status):
            raise AssertionError(
                f"Unexpected status: expected {expected_status}, got {response.status_code}: {response.text}"
            )

        payload = self._response_json(response)
        if expected_error:
            actual_error = str(payload.get("error", ""))
            if expected_error not in actual_error:
                raise AssertionError(
                    f"Expected error containing {expected_error!r}, got {actual_error!r}"
                )
        return payload
