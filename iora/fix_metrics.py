#!/usr/bin/env python3

import re

def fix_api_metrics(content):
    # Pattern to match ApiMetrics initializations missing provider and circuit_breaker_tripped
    pattern = r'metrics\.insert\(ApiProvider::(\w+), ApiMetrics \{\s*([^}]*?)\s*\}\)'

    def replacement(match):
        provider_name = match.group(1)
        existing_fields = match.group(2)

        # Check if provider field is already present
        if 'provider:' in existing_fields:
            return match.group(0)

        # Add missing fields
        provider_field = f'provider: ApiProvider::{provider_name},'
        circuit_breaker_field = 'circuit_breaker_tripped: false,'

        # Insert the missing fields after the opening brace
        lines = existing_fields.strip().split('\n')
        if lines and lines[0].strip():
            lines.insert(0, provider_field)
            lines.insert(1, circuit_breaker_field)
        else:
            lines = [provider_field, circuit_breaker_field]

        return f'metrics.insert(ApiProvider::{provider_name}, ApiMetrics {{\n                {"\n                ".join(lines)}\n            }})'

    return re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)

# Read the file
with open('tests/routing_algorithm_tests.rs', 'r') as f:
    content = f.read()

# Fix the ApiMetrics initializations
fixed_content = fix_api_metrics(content)

# Write back to file
with open('tests/routing_algorithm_tests.rs', 'w') as f:
    f.write(fixed_content)

print("Fixed ApiMetrics initializations in routing_algorithm_tests.rs")
