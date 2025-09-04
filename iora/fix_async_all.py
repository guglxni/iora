#!/usr/bin/env python3

# Read the file
with open('tests/advanced_data_processing_tests.rs', 'r') as f:
    content = f.read()

# Replace all async fn test_ with fn test_
content = content.replace('    async fn test_', '    fn test_')

# Write back to file
with open('tests/advanced_data_processing_tests.rs', 'w') as f:
    f.write(content)

print("Fixed all async fn test_ to fn test_ in advanced_data_processing_tests.rs")

