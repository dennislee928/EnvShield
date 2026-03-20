*** Settings ***
Resource      ../resources/common.resource
Test Setup    Initialize API Session

*** Test Cases ***
Create Workspace Returns Created Workspace
    Create Unique Identity Data
    Ensure Device Registered
    ${body}=    Create Workspace    force_new=${TRUE}
    ${workspace_name}=    Get Context Value    workspace_name
    Assert Json Field    ${body}    id    __nonempty__
    Assert Json Field    ${body}    name    ${workspace_name}
    ${members}=    Get From Dictionary    ${body}    members
    Should Not Be Empty    ${members}

Get Workspace Returns Existing Workspace
    Ensure Workspace Created
    ${workspace_id}=    Get Context Value    workspace_id
    ${response}=    Get Json On Session    /v1/workspaces/${workspace_id}    authenticated=${TRUE}
    Should Be Equal As Integers    ${response.status_code}    200
    ${body}=    Get Json Response    ${response}
    Assert Json Field    ${body}    id    ${workspace_id}

Unknown Workspace Returns 404
    ${response}=    Get Json On Session    /v1/workspaces/robot-missing-workspace    authenticated=${TRUE}
    Assert Error Response    ${response}    404    not found
