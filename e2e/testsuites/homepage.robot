*** Settings ***
Library    Browser

*** Variables ***
${BROWSER}            chromium
${URL}                http://127.0.0.1:8080/
${EXPECTED_TITLE}     Kev's Quiz Web App

*** Test Cases ***
Homepage Should Load Correctly
  New Browser    browser=${BROWSER}    headless=False
  New Page    ${URL}
  Wait For Load State    domcontentloaded    timeout=3s
  Get Title    ==    ${EXPECTED_TITLE}
  Close Browser

