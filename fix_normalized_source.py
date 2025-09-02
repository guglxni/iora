#!/usr/bin/env python3

# Read the file
with open('tests/advanced_data_processing_tests.rs', 'r') as f:
    content = f.read()

# Replace all instances of crate::modules::processor::NormalizedSource with NormalizedSource
fixed_content = content.replace('crate::modules::processor::NormalizedSource', 'NormalizedSource')

# Write back to file
with open('tests/advanced_data_processing_tests.rs', 'w') as f:
    f.write(fixed_content)

print("Fixed NormalizedSource references in advanced_data_processing_tests.rs")
