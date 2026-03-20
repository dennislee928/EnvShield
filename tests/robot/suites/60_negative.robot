*** Settings ***
Resource         ../resources/common.resource
Test Setup       Initialize API Session
Test Template    Post Malformed Json Should Return 400

*** Test Cases ***
Start Auth Rejects Malformed Json             /v1/auth/github/start
Approve Device Rejects Malformed Json         /v1/auth/device/approve
Exchange Device Rejects Malformed Json        /v1/auth/device/exchange
Register Device Rejects Malformed Json        /v1/devices
Create Workspace Rejects Malformed Json       /v1/workspaces
Create Snapshot Rejects Malformed Json        /v1/snapshots
