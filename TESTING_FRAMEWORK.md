# IORA Comprehensive Testing Framework

## Overview

This document outlines the comprehensive testing strategy for the IORA (Intelligent Oracle Rust Assistant) system, covering both the Rust backend and TypeScript/Node.js frontend components.

## Test Categories

### 1. Unit Tests
- **Rust Backend**: Individual module and function testing
- **TypeScript Frontend**: Component and utility function testing
- **Coverage Goal**: >85% for critical paths

### 2. Integration Tests
- **Backend**: Module-to-module communication
- **Frontend**: API route integration
- **Cross-Component**: MCP server ↔ Next.js demo

### 3. End-to-End (E2E) Tests
- **Authentication Flow**: Clerk sign-up → API key creation → Dashboard access
- **API Workflow**: Price fetch → Analysis → Blockchain feed
- **Error Scenarios**: Network failures, auth errors, rate limiting

### 4. Performance Tests
- **Load Testing**: Concurrent requests, memory usage
- **Stress Testing**: High-volume scenarios
- **Benchmarking**: Response times, throughput

### 5. Security Tests
- **Authentication**: Token validation, session management
- **Authorization**: Role-based access control
- **Input Validation**: SQL injection, XSS prevention
- **API Security**: Rate limiting, HMAC validation

## Test Structure

```
iora/
├── tests/                          # Rust backend tests
│   ├── unit_tests.rs              # Core functionality
│   ├── integration_tests.rs       # Module integration
│   ├── api_tests.rs              # HTTP API testing
│   ├── auth_tests.rs             # Authentication flows
│   ├── performance_tests.rs      # Load and stress tests
│   ├── security_tests.rs         # Security validation
│   └── e2e_tests.rs              # End-to-end scenarios
│
└── iora/mcp/tests/               # TypeScript frontend tests
    ├── unit/                     # Component unit tests
    ├── integration/              # API integration tests
    ├── e2e/                      # E2E user flows
    └── performance/               # Performance tests
```

## Testing Tools & Frameworks

### Rust Backend
- **Primary**: `cargo test` (built-in)
- **Mocking**: `mockall` crate
- **HTTP Testing**: `reqwest` for API testing
- **Async Testing**: `tokio-test`
- **Coverage**: `cargo-tarpaulin`

### TypeScript Frontend
- **Primary**: `vitest` (configured in package.json)
- **React Testing**: `@testing-library/react`
- **API Testing**: `msw` (Mock Service Worker)
- **E2E**: `playwright` or `cypress`
- **Coverage**: `vitest` with `c8`

### Integration Testing
- **Backend-to-Backend**: Custom test harnesses
- **Frontend-to-Backend**: HTTP client testing
- **Database**: Test containers with PostgreSQL
- **External APIs**: Mock servers for CoinGecko, etc.

## Test Environment Setup

### Development Testing
```bash
# Rust backend
cargo test                    # Run all tests
cargo test --test unit_tests  # Run specific test file
cargo test --lib              # Run library tests only

# TypeScript frontend
cd iora/mcp && npm test       # Run all TS tests
cd demo && npm test           # Run Next.js tests
```

### CI/CD Testing
```yaml
# GitHub Actions workflow
- name: Run comprehensive test suite
  run: |
    # Rust tests
    cargo test --workspace
    cargo tarpaulin --out xml

    # TypeScript tests
    cd iora/mcp && npm ci && npm run test:coverage
    cd ../demo && npm ci && npm run test

    # Integration tests
    docker-compose -f docker-compose.test.yml up --abort-on-container-exit
```

## Test Data Management

### Mock Data Strategy
- **External APIs**: VCR.py-style request recording/playback
- **Database**: Test fixtures with cleanup
- **Blockchain**: Local testnet (Solana devnet)
- **Authentication**: Test tokens and keys

### Test Database
- **Tool**: PostgreSQL test containers
- **Migrations**: Automated setup/teardown
- **Seeding**: Realistic test data generation

## Authentication Testing

### Clerk Integration Tests
```typescript
// Test sign-up flow
describe('Authentication Flow', () => {
  it('should complete sign-up and create user session', async () => {
    // Mock Clerk API responses
    const mockUser = { id: 'user_123', email: 'test@example.com' };

    // Test sign-up process
    const result = await signUpUser(mockUser);
    expect(result).toBeSuccessful();

    // Verify session creation
    const session = await getSession();
    expect(session.userId).toBe(mockUser.id);
  });
});
```

### API Key Testing
```rust
#[test]
fn test_api_key_generation_and_validation() {
    let api_key = generate_api_key("test-key", vec!["read".to_string()]);
    assert!(validate_api_key(&api_key.key).is_ok());
    assert_eq!(validate_api_key(&api_key.key).unwrap().user_id, "test-user");
}
```

## API Integration Testing

### MCP Server Endpoints
```typescript
describe('MCP Server API', () => {
  it('should return price data for valid symbol', async () => {
    const response = await request(app)
      .post('/tools/get_price')
      .set('Authorization', `Bearer ${validToken}`)
      .send({ symbol: 'BTC' });

    expect(response.status).toBe(200);
    expect(response.body.data.price).toBeGreaterThan(0);
  });

  it('should reject unauthorized requests', async () => {
    const response = await request(app)
      .post('/tools/get_price')
      .send({ symbol: 'BTC' });

    expect(response.status).toBe(401);
  });
});
```

## Performance Testing

### Load Testing Setup
```typescript
describe('Load Testing', () => {
  it('should handle 100 concurrent price requests', async () => {
    const requests = Array(100).fill().map(() =>
      request(app).post('/tools/get_price').send({ symbol: 'BTC' })
    );

    const results = await Promise.all(requests);
    const successRate = results.filter(r => r.status === 200).length;

    expect(successRate).toBeGreaterThan(95); // 95% success rate
  });
});
```

## Security Testing

### Authentication Security
```rust
#[test]
fn test_hmac_signature_validation() {
    let payload = r#"{"symbol":"BTC"}"#;
    let secret = "test-secret";
    let signature = generate_hmac_signature(payload, secret);

    // Test valid signature
    assert!(validate_hmac_signature(payload, signature, secret));

    // Test invalid signature
    assert!(!validate_hmac_signature(payload, "invalid", secret));
}
```

### Rate Limiting Tests
```typescript
describe('Rate Limiting', () => {
  it('should enforce rate limits per user', async () => {
    // Make requests up to limit
    for (let i = 0; i < 60; i++) {
      await request(app).post('/tools/get_price').send({ symbol: 'BTC' });
    }

    // Next request should be rate limited
    const response = await request(app).post('/tools/get_price').send({ symbol: 'BTC' });
    expect(response.status).toBe(429);
  });
});
```

## CI/CD Integration

### GitHub Actions Workflow
```yaml
name: Comprehensive Test Suite

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  test:
    name: Full Test Suite
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: test_password
        ports:
          - 5432:5432

    steps:
      - uses: actions/checkout@v4

      # Rust tests
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --workspace

      # TypeScript tests
      - uses: actions/setup-node@v4
      - run: cd iora/mcp && npm ci && npm run test:coverage
      - run: cd ../demo && npm ci && npm run test

      # Integration tests
      - run: docker-compose -f docker-compose.test.yml up --abort-on-container-exit

      # Performance tests
      - run: cargo test --test performance_tests

      # Security audit
      - run: cargo audit
```

## Test Reporting & Coverage

### Coverage Targets
- **Rust Backend**: >90% coverage for core modules
- **TypeScript Frontend**: >85% coverage for components
- **API Endpoints**: 100% coverage for all routes
- **Critical Paths**: 100% coverage

### Reporting Tools
- **Rust**: `cargo-tarpaulin` → Codecov integration
- **TypeScript**: `vitest` with `c8` → Codecov integration
- **E2E**: Test results → GitHub Actions artifacts

## Best Practices

### Test Organization
1. **One assertion per test** - Clear failure identification
2. **Descriptive test names** - Self-documenting tests
3. **Setup/Teardown** - Clean state between tests
4. **Mock external dependencies** - Fast, reliable tests
5. **Test data factories** - Consistent, realistic data

### Async Testing
```rust
#[tokio::test]
async fn test_async_functionality() {
    // Use tokio::test for async tests
    let result = async_function().await;
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

## Debugging Tests

### Common Issues
1. **Flaky tests** - Use deterministic mocking
2. **Slow tests** - Parallel execution where possible
3. **Memory leaks** - Monitor in performance tests
4. **Test isolation** - Proper cleanup between tests

### Debug Commands
```bash
# Run specific test
cargo test test_function_name

# Run with output
cargo test -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test

# TypeScript debugging
npm run test -- --reporter=verbose
```

## Maintenance

### Regular Tasks
1. **Weekly**: Review test coverage reports
2. **Monthly**: Update test data and mocks
3. **Quarterly**: Performance regression testing
4. **Before releases**: Full E2E test suite

### Test Debt Management
- **Technical debt**: Tests for deprecated features
- **Coverage gaps**: New features without tests
- **Maintenance burden**: Overly complex test setups

## Getting Started

### For New Contributors
1. **Run existing tests**: `cargo test && cd iora/mcp && npm test`
2. **Add tests for new features**: Follow existing patterns
3. **Update documentation**: Document new test scenarios
4. **Review coverage**: Ensure >80% coverage for new code

### For Maintainers
1. **Monitor CI/CD**: Address failing tests immediately
2. **Code review**: Ensure new code includes tests
3. **Refactor safely**: Update tests when refactoring
4. **Performance**: Monitor test execution time

---

**Last Updated**: October 3, 2025
**Test Coverage Goal**: 90%+ for production readiness
**CI/CD Integration**: Full automation with quality gates

