# IORA Testing Guide

## Overview

This guide provides comprehensive instructions for running and maintaining the IORA testing suite. The testing framework covers unit tests, integration tests, end-to-end tests, performance tests, and security tests for both the Rust backend and TypeScript frontend.

## Test Categories

### 🧪 Unit Tests
- **Rust**: Individual module and function testing
- **TypeScript**: Component and utility function testing
- **Coverage Goal**: >85% for critical paths

### 🔗 Integration Tests
- **Backend**: Module-to-module communication
- **Frontend**: API route integration
- **Cross-Component**: MCP server ↔ Next.js demo

### 🚀 End-to-End (E2E) Tests
- **Authentication Flow**: Sign-up → API key creation → Dashboard access
- **API Workflow**: Price fetch → Analysis → Blockchain feed
- **Error Scenarios**: Network failures, auth errors, rate limiting

### ⚡ Performance Tests
- **Load Testing**: Concurrent requests, memory usage
- **Stress Testing**: High-volume scenarios
- **Benchmarking**: Response times, throughput

### 🔒 Security Tests
- **Authentication**: Token validation, session management
- **Authorization**: Role-based access control
- **Input Validation**: SQL injection, XSS prevention

## Quick Start

### Prerequisites
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Install cargo testing tools
cargo install cargo-tarpaulin cargo-audit

# Install Node.js dependencies
cd iora/mcp && npm install
cd ../demo && npm install
```

### Run All Tests
```bash
# Run all Rust tests
cargo test --workspace

# Run all TypeScript tests
cd iora/mcp && npm test

# Run all Next.js tests
cd demo && npm run test:run

# Run specific test categories
cargo test --test unit_tests        # Unit tests only
cargo test --test integration_tests # Integration tests only
cargo test --test auth_tests        # Authentication tests only
cargo test --test api_integration_tests # API tests only
```

## Detailed Test Execution

### Rust Backend Tests

#### Unit Tests
```bash
# Run all unit tests
cargo test --lib

# Run specific unit test file
cargo test --test unit_tests

# Run tests with output
cargo test -- --nocapture

# Run tests with backtrace (for debugging)
RUST_BACKTRACE=1 cargo test
```

#### Integration Tests
```bash
# Run integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test --test integration_tests -- --nocapture
```

#### Authentication Tests
```bash
# Run authentication flow tests
cargo test --test auth_tests

# Run API key management tests
cargo test --test auth_tests -- --nocapture test_api_key_creation
```

#### API Integration Tests
```bash
# Run API endpoint tests
cargo test --test api_integration_tests

# Run specific API test
cargo test --test api_integration_tests -- test_get_price_endpoint
```

#### End-to-End Tests
```bash
# Run complete authentication flow tests
cargo test --test e2e_authentication_tests

# Run load testing
cargo test --test e2e_authentication_tests -- test_sustained_load_authentication
```

### TypeScript Frontend Tests

#### MCP Server Tests
```bash
cd iora/mcp

# Run all tests
npm test

# Run tests in watch mode
npm run test:watch

# Run tests with UI
npm run test:ui

# Run tests once
npm run test:run

# Run with coverage
npm run test:coverage
```

#### Next.js Demo Tests
```bash
cd demo

# Run all tests
npm run test:run

# Run tests in watch mode
npm run test

# Run with coverage
npm run test:coverage

# Run specific test file
npm run test DashboardClient.test.tsx
```

## CI/CD Integration

### GitHub Actions Workflow
The CI/CD pipeline runs comprehensive tests on every push and PR:

```yaml
# Runs automatically on:
# - Push to main/develop branches
# - Pull requests to main/develop branches

# Test jobs:
# - rust-test: Complete Rust test suite
# - typescript-test: MCP server TypeScript tests
# - nextjs-test: Next.js demo tests
# - coverage: Code coverage analysis
# - security-audit: Dependency security scanning
# - performance: Performance benchmarks
# - docker: Container build tests
# - release: Release build validation
```

### Viewing Test Results
- **GitHub Actions**: Check the "Actions" tab for detailed test results
- **Code Coverage**: View reports in the "coverage" job artifacts
- **Performance Metrics**: Monitor in the "performance" job logs

## Test Data Management

### Mock Data Strategy
- **External APIs**: VCR.py-style request recording/playback
- **Database**: Test fixtures with cleanup
- **Blockchain**: Local testnet (Solana devnet)
- **Authentication**: Test tokens and keys

### Environment Variables for Testing
```bash
# For MCP server tests
export TEST_SERVER_URL="http://localhost:7145"
export HMAC_SECRET="test_hmac_secret_for_testing"

# For Next.js demo tests
export MCP_SERVER_URL="http://localhost:7145"
export NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY="pk_test_..."
export CLERK_SECRET_KEY="sk_test_..."
```

## Debugging Tests

### Common Issues & Solutions

#### Rust Tests
```bash
# Test not found
cargo test test_function_name

# Async test debugging
#[tokio::test]
async fn test_async_function() {
    // Use tokio::test for async tests
}

# Test isolation issues
#[test]
fn test_with_setup() {
    // Use setup functions for clean state
}
```

#### TypeScript Tests
```bash
# Component not rendering
// Check if component is properly exported
// Verify test setup in setup.ts

# API mocking issues
// Ensure MSW is properly configured
// Check fetch mock setup

# Authentication mocking
// Verify Clerk mock in test setup
// Check auth context providers
```

#### Integration Issues
```bash
# Server not responding
# Check if servers are running on correct ports
lsof -i -P | grep LISTEN | grep -E "(7070|7145|3000)"

# Environment variables not loaded
# Ensure .env files are properly configured
# Check for missing API keys

# Authentication failures
# Verify test tokens and API keys
# Check Clerk configuration in tests
```

### Debug Commands
```bash
# Run single test with verbose output
cargo test test_name -- --nocapture

# Run tests with timing information
cargo test -- --report-time

# TypeScript debugging
npm run test -- --reporter=verbose --no-coverage

# Check test environment
cargo test --test unit_tests test_cargo_toml_exists -- --nocapture
```

## Performance Testing

### Load Testing Setup
```bash
# Run sustained load test (5 minutes)
cargo test --test e2e_authentication_tests -- test_sustained_load_authentication

# Run concurrent request test (100 concurrent requests)
cargo test --test e2e_authentication_tests -- test_authentication_performance_concurrent

# Monitor memory usage during tests
cargo test --test performance_tests -- --nocapture
```

### Performance Benchmarks
- **Response Time**: < 2 seconds for API endpoints
- **Concurrent Users**: Support 100+ simultaneous requests
- **Memory Usage**: < 100MB per test process
- **Error Rate**: < 5% under normal load

## Security Testing

### Authentication Security
```bash
# Test HMAC signature validation
cargo test --test auth_tests -- test_hmac_authentication

# Test rate limiting enforcement
cargo test --test auth_tests -- test_rate_limiting

# Test session timeout handling
cargo test --test auth_tests -- test_session_management

# Test multi-user isolation
cargo test --test auth_tests -- test_multi_user_isolation
```

### API Security
```bash
# Test input validation
cargo test --test api_integration_tests -- test_invalid_input_handling

# Test XSS prevention
cargo test --test e2e_authentication_tests -- test_authentication_security

# Test rate limiting across auth methods
cargo test --test auth_tests -- test_rate_limiting_across_auth_methods
```

## Coverage Analysis

### Code Coverage Reports
```bash
# Generate Rust coverage report
cargo tarpaulin --out html --output-dir coverage/

# Generate TypeScript coverage report
cd iora/mcp && npm run test:coverage
cd ../demo && npm run test:coverage

# Open coverage reports
open coverage/tarpaulin-report.html  # Rust
open coverage/lcov-report/index.html  # TypeScript
```

### Coverage Targets
- **Rust Backend**: >90% coverage for core modules
- **TypeScript Frontend**: >85% coverage for components
- **API Endpoints**: 100% coverage for all routes
- **Critical Authentication Paths**: 100% coverage

## Best Practices

### Test Organization
1. **One assertion per test** - Clear failure identification
2. **Descriptive test names** - Self-documenting tests
3. **Setup/Teardown** - Clean state between tests
4. **Mock external dependencies** - Fast, reliable tests
5. **Test data factories** - Consistent, realistic data

### Async Testing Patterns
```rust
#[tokio::test]
async fn test_async_authentication() {
    // Use tokio::test for async tests
    let result = authenticate_user("test@example.com").await;
    assert!(result.is_ok());
}
```

### Error Testing
```rust
#[test]
fn test_error_handling() {
    let result = function_that_might_fail();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}
```

## Maintenance Tasks

### Regular Maintenance
- **Weekly**: Review test coverage reports
- **Monthly**: Update test data and mocks
- **Quarterly**: Performance regression testing
- **Before releases**: Full E2E test suite

### Test Debt Management
- **Technical debt**: Tests for deprecated features
- **Coverage gaps**: New features without tests
- **Maintenance burden**: Overly complex test setups

## Contributing Tests

### For New Features
1. **Write tests first** (TDD approach)
2. **Follow existing patterns** in similar test files
3. **Include edge cases** and error scenarios
4. **Update documentation** with new test scenarios

### Code Review Guidelines
- **Test coverage**: Ensure >80% coverage for new code
- **Test quality**: Clear, maintainable, well-documented tests
- **Performance impact**: Tests should not significantly slow CI/CD
- **Dependencies**: Minimize external test dependencies

## Troubleshooting

### Common Issues

#### "Test not found"
```bash
# Check test function name exactly
cargo test --list | grep test_name

# Run with pattern matching
cargo test pattern_matching
```

#### "Async test failed"
```bash
# Ensure proper tokio::test attribute
#[tokio::test]
async fn async_test() { /* ... */ }

// Run with verbose output
cargo test async_test -- --nocapture
```

#### "Network request failed"
```bash
# Check if test servers are running
curl http://localhost:7145/tools/health

# Check environment variables
echo $TEST_SERVER_URL

# Verify network connectivity
ping localhost
```

#### "TypeScript compilation error"
```bash
# Check TypeScript configuration
cd demo && npx tsc --noEmit

# Fix import issues
# Ensure proper mock setup in test files
```

### Getting Help
1. **Check test output** for detailed error messages
2. **Review CI/CD logs** in GitHub Actions
3. **Consult test documentation** in `TESTING_FRAMEWORK.md`
4. **Ask team members** for guidance on complex scenarios

## Test Metrics

### Success Criteria
- **Unit Tests**: >95% pass rate
- **Integration Tests**: >90% pass rate
- **E2E Tests**: >85% pass rate
- **Performance Tests**: All benchmarks within limits
- **Security Tests**: No vulnerabilities found

### Monitoring
- **CI/CD Pipeline**: All tests pass on main branch
- **Coverage Reports**: Maintain >85% overall coverage
- **Performance Trends**: Monitor for regressions
- **Security Scans**: Regular vulnerability assessments

---

**Last Updated**: October 3, 2025
**Test Coverage Goal**: 90%+ for production readiness
**CI/CD Integration**: Full automation with quality gates

