*** Settings ***
Library           Collections
Library           RequestsLibrary
Library           ../libraries/EnvShieldApiLibrary.py    ${BASE_URL}

*** Variables ***
${SESSION_ALIAS}    envshield

*** Keywords ***
Initialize API Session
    Should Not Be Empty    ${BASE_URL}    msg=Pass --variable BASE_URL:https://<host> to robot.
    Delete All Sessions
    Create Session    ${SESSION_ALIAS}    ${BASE_URL}    verify=${TRUE}

Ensure Authenticated Context
    Create Unique Identity Data
    ${token}=    Get Context Value    token
    IF    not $token
        Start Device Auth
        Approve Device Auth
        Exchange Device Token
    END

Ensure Device Registered
    Ensure Authenticated Context
    ${device_id}=    Get Context Value    device_id
    IF    not $device_id
        Register Device
    END

Ensure Workspace Created
    Ensure Device Registered
    ${workspace_id}=    Get Context Value    workspace_id
    IF    not $workspace_id
        Create Workspace
    END

Ensure Snapshot Created
    Ensure Workspace Created
    ${snapshot_id}=    Get Context Value    snapshot_id
    IF    not $snapshot_id
        Create Snapshot
    END

Get Auth Headers
    [Arguments]    ${authenticated}=${FALSE}
    ${headers}=    Create Dictionary    Content-Type=application/json
    IF    $authenticated
        ${token}=    Get Context Value    token
        Should Not Be Empty    ${token}    msg=Expected a stored token for authenticated request.
        Set To Dictionary    ${headers}    Authorization=Bearer ${token}
    END
    RETURN    ${headers}

Get Json Response
    [Arguments]    ${response}
    ${payload}=    Evaluate    $response.json()
    RETURN    ${payload}

Get Json On Session
    [Arguments]    ${path}    ${authenticated}=${FALSE}
    ${headers}=    Get Auth Headers    ${authenticated}
    ${response}=    GET On Session    ${SESSION_ALIAS}    ${path}    headers=${headers}    expected_status=anything
    RETURN    ${response}

Post Json On Session
    [Arguments]    ${path}    ${payload}    ${authenticated}=${FALSE}
    ${headers}=    Get Auth Headers    ${authenticated}
    ${response}=    POST On Session    ${SESSION_ALIAS}    ${path}    json=${payload}    headers=${headers}    expected_status=anything
    RETURN    ${response}

Post Malformed Json Should Return 400
    [Arguments]    ${path}    ${authenticated}=${FALSE}
    ${headers}=    Get Auth Headers    ${authenticated}
    ${response}=    POST On Session    ${SESSION_ALIAS}    ${path}    data={bad json    headers=${headers}    expected_status=anything
    Assert Error Response    ${response}    400    invalid character
