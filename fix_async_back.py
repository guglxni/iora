#!/usr/bin/env python3

# Read the file
with open('tests/advanced_data_processing_tests.rs', 'r') as f:
    content = f.read()

# Replace all fn test_ with async fn test_
content = content.replace('    fn test_', '    async fn test_')

# Also make the master test function async
content = content.replace('    fn run_comprehensive_advanced_data_processing_test_suite() {', '    async fn run_comprehensive_advanced_data_processing_test_suite() {')

# Write back to file
with open('tests/advanced_data_processing_tests.rs', 'w') as f:
    f.write(content)

print("Made all test functions async again in advanced_data_processing_tests.rs")

