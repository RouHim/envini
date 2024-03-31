#!/usr/bin/env bash
#
# Description:
#   This is an end to end test script, that tests the actual executable.
#
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

# First create an ini file that should be tested,
# It contains two entries, one with section, one without section:
cat > test.ini <<EOF
encoding = utf-8
[User]
name=test
EOF

# Then create the test_envini_mapping.ini that contains the mapping between the env var ant the ini properties to set:
# /// # Example ini file
  #/// ```ini
  #/// [KF2_WEB_ADMIN_PASSWORD]
  #/// ini_file = ~/test.ini
  #/// ini_section = Engine.AccessControl
  #/// ini_key = AdminPassword
  #/// ```
cat > test_envini_mapping.ini <<EOF
[TEST_ENCODING]
ini_file = test.ini
ini_key = encoding
ini_section =

[TEST_USER_NAME]
ini_file = test.ini
ini_key = name
ini_section = User
EOF

# Then set the env var vars to some new values:
export TEST_ENCODING=ascii
export TEST_USER_NAME=John

# Then run the executable with the test_envini_mapping.ini file:
cargo run test_envini_mapping.ini

# Then check the test.ini file, it should have the new values:
INI_CONTENT=$(cat test.ini)
EXPECTED_INI_CONTENT="encoding=ascii

[User]
name=John"
if [ "$INI_CONTENT" != "$EXPECTED_INI_CONTENT" ]; then
  echo "Test failed, ini file content is not as expected"
  echo ""
  echo "Expected:"
  echo "==========="
  echo "$EXPECTED_INI_CONTENT"
  echo "==========="
  echo ""
  echo ""
  echo ""
  echo "Actual:"
  echo "==========="
  echo "$INI_CONTENT"
  echo "==========="
  exit 1
fi

# Clean up
rm test.ini
rm test_envini_mapping.ini