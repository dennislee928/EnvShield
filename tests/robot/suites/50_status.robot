*** Settings ***
Resource      ../resources/common.resource
Test Setup    Initialize API Session

*** Test Cases ***
Status Without Snapshot Returns Zero Version
    Ensure Workspace Created
    ${workspace_id}=    Get Context Value    workspace_id
    ${empty_environment}=    Get Context Value    empty_environment
    ${response}=    Get Json On Session    /v1/workspaces/${workspace_id}/environments/${empty_environment}/status?local_version=0    authenticated=${TRUE}
    Should Be Equal As Integers    ${response.status_code}    200
    ${body}=    Get Json Response    ${response}
    Assert Json Field    ${body}    latestVersion    0
    Assert Json Field    ${body}    localVersion    0
    Assert Json Field    ${body}    outOfDate    ${FALSE}

Status After Snapshot Marks Client Out Of Date
    Ensure Snapshot Created
    ${workspace_id}=    Get Context Value    workspace_id
    ${environment}=    Get Context Value    environment
    ${snapshot_version}=    Get Context Value    snapshot_version
    ${response}=    Get Json On Session    /v1/workspaces/${workspace_id}/environments/${environment}/status?local_version=0    authenticated=${TRUE}
    Should Be Equal As Integers    ${response.status_code}    200
    ${body}=    Get Json Response    ${response}
    Assert Json Field    ${body}    latestVersion    ${snapshot_version}
    Assert Json Field    ${body}    outOfDate    ${TRUE}
