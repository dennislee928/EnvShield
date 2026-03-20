*** Settings ***
Resource      ../resources/common.resource
Test Setup    Initialize API Session

*** Test Cases ***
Start GitHub Auth Creates Device Flow
    Create Unique Identity Data
    ${body}=    Start Device Auth    force_new=${TRUE}
    Assert Json Field    ${body}    deviceCode    __nonempty__
    Assert Json Field    ${body}    userCode    __nonempty__
    Assert Json Field    ${body}    verificationUrl    __nonempty__
    ${verification_url}=    Get From Dictionary    ${body}    verificationUrl
    Should Start With    ${verification_url}    ${BASE_URL}/approve?device_code=
    ${expires_in}=    Get From Dictionary    ${body}    expiresIn
    Should Be True    ${expires_in} > 0

Exchange Before Approval Returns 409
    Create Unique Identity Data
    ${body}=    Start Device Auth    force_new=${TRUE}
    ${device_code}=    Get From Dictionary    ${body}    deviceCode
    ${payload}=    Create Dictionary    deviceCode=${device_code}
    ${response}=    Post Json On Session    /v1/auth/device/exchange    ${payload}
    Assert Error Response    ${response}    409    pending approval

Approve Unknown Device Code Returns 404
    ${payload}=    Create Dictionary    deviceCode=unknown-device-code    actorEmail=robot-missing@example.com
    ${response}=    Post Json On Session    /v1/auth/device/approve    ${payload}
    Assert Error Response    ${response}    404    not found

Exchange Unknown Device Code Returns 404
    ${payload}=    Create Dictionary    deviceCode=unknown-device-code
    ${response}=    Post Json On Session    /v1/auth/device/exchange    ${payload}
    Assert Error Response    ${response}    404    not found

Approve Device Auth Returns 204
    Create Unique Identity Data
    Start Device Auth    force_new=${TRUE}
    ${status}=    Approve Device Auth
    Should Be Equal As Integers    ${status}    204

Exchange Approved Device Returns Token
    Create Unique Identity Data
    Start Device Auth    force_new=${TRUE}
    Approve Device Auth
    ${body}=    Exchange Device Token
    Assert Json Field    ${body}    token    __nonempty__
    ${actor_email}=    Get Context Value    actor_email
    Assert Json Field    ${body}    actorEmail    ${actor_email}
