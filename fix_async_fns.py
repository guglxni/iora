#!/usr/bin/env python3

# Read the file
with open('tests/advanced_data_processing_tests.rs', 'r') as f:
    content = f.read()

# Replace all async fn test_ with fn test_
import re
fixed_content = re.sub(r'(\s+)async fn test_', r'\1fn test_', content)

# Write back to file
with open('tests/advanced_data_processing_tests.rs', 'w') as f:
    f.write(fixed_content)

print("Fixed all async fn test_ declarations to fn test_ in advanced_data_processing_tests.rs")

