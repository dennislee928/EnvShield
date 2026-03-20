*** Settings ***
Resource      ../resources/common.resource
Test Setup    Initialize API Session

*** Test Cases ***
Latest Snapshot Returns 404 Before Create
    Ensure Workspace Created
    ${workspace_id}=    Get Context Value    workspace_id
    ${empty_environment}=    Get Context Value    empty_environment
    ${response}=    Get Json On Session    /v1/workspaces/${workspace_id}/environments/${empty_environment}/snapshots/latest    authenticated=${TRUE}
    Assert Error Response    ${response}    404    not found

Create Snapshot Returns Stored Snapshot
    Ensure Workspace Created
    ${body}=    Create Snapshot
    ${workspace_id}=    Get Context Value    workspace_id
    ${environment}=    Get Context Value    environment
    Assert Json Field    ${body}    snapshotId    __nonempty__
    Assert Json Field    ${body}    workspaceId    ${workspace_id}
    Assert Json Field    ${body}    environment    ${environment}
    ${version}=    Get From Dictionary    ${body}    version
    Should Be True    ${version} >= 1

Latest Snapshot Returns Created Snapshot
    Ensure Snapshot Created
    ${workspace_id}=    Get Context Value    workspace_id
    ${environment}=    Get Context Value    environment
    ${snapshot_id}=    Get Context Value    snapshot_id
    ${response}=    Get Json On Session    /v1/workspaces/${workspace_id}/environments/${environment}/snapshots/latest    authenticated=${TRUE}
    Should Be Equal As Integers    ${response.status_code}    200
    ${body}=    Get Json Response    ${response}
    Assert Json Field    ${body}    snapshotId    ${snapshot_id}
