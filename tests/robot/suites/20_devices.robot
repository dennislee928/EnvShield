*** Settings ***
Resource      ../resources/common.resource
Test Setup    Initialize API Session

*** Test Cases ***
Register Device Returns Created Device
    Create Unique Identity Data
    Ensure Authenticated Context
    ${body}=    Register Device    force_new=${TRUE}
    ${device_name}=    Get Context Value    device_name
    Assert Json Field    ${body}    id    __nonempty__
    Assert Json Field    ${body}    name    ${device_name}
    Assert Json Field    ${body}    encryptionPublicKey    __nonempty__
    Assert Json Field    ${body}    signingPublicKey    __nonempty__

List Devices Includes Registered Device
    Ensure Device Registered
    ${response}=    Get Json On Session    /v1/devices    authenticated=${TRUE}
    Should Be Equal As Integers    ${response.status_code}    200
    ${body}=    Get Json Response    ${response}
    ${device_ids}=    Evaluate    [item["id"] for item in $body["devices"]]
    ${device_id}=    Get Context Value    device_id
    List Should Contain Value    ${device_ids}    ${device_id}
