*** Settings ***
Resource      ../resources/common.resource
Test Setup    Initialize API Session

*** Test Cases ***
Health Endpoint Returns Ok
    ${response}=    GET On Session    ${SESSION_ALIAS}    /healthz    expected_status=anything
    Should Be Equal As Integers    ${response.status_code}    200
    ${body}=    Get Json Response    ${response}
    Assert Json Field    ${body}    status    ok
