#!/usr/bin/env python3

# Read the file
with open('tests/advanced_data_processing_tests.rs', 'r') as f:
    content = f.read()

# Replace all fn test_ with async fn test_ for functions that are called with .await
# We need to be careful not to affect functions that shouldn't be async

# First, let's find all the test functions that are called with .await in the master function
import re

# Pattern to match function calls in the master test function
called_functions = re.findall(r'(\w+)\(\)\.await', content)

# Get unique function names that are called with .await
unique_called_functions = set(called_functions)

print("Functions called with .await:", unique_called_functions)

# Now add async to all test functions that are called with .await
for func_name in unique_called_functions:
    if func_name.startswith('test_'):
        # Replace fn func_name with async fn func_name
        pattern = rf'(    )fn {func_name}\('
        replacement = rf'\1async fn {func_name}('
        content = re.sub(pattern, replacement, content)

# Write back to file
with open('tests/advanced_data_processing_tests.rs', 'w') as f:
    f.write(content)

print("Fixed async keywords for functions called with .await")

