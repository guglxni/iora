#!/usr/bin/env python3

# Read the file
with open('tests/multi_api_integration_tests.rs', 'r') as f:
    content = f.read()

# Replace all instances of get_available_providers() with get_all_resilience_status()
content = content.replace('client.get_available_providers()', 'client.get_all_resilience_status()')

# Replace providers.contains() with all_status.contains_key()
content = content.replace('providers.contains(', 'all_status.contains_key(')

# Write back to file
with open('tests/multi_api_integration_tests.rs', 'w') as f:
    f.write(content)

print("Fixed all get_available_providers() calls in multi_api_integration_tests.rs")

