# Contributing to I.O.R.A.

Thank you for your interest in contributing to **I.O.R.A. (Intelligent Oracle Rust Assistant)**! This document provides guidelines and information for contributors.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ (latest stable recommended)
- Node.js 16+ (for Anchor if developing Solana components)
- Docker and Docker Compose
- Git

### Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/guglxni/iora.git
   cd iora
   ```

2. **Set up the development environment:**
   ```bash
   # Run complete setup (installs all tools)
   ./scripts/dev-workflow.sh setup

   # Or run individual components
   ./scripts/install-all-tools.sh
   ```

3. **Verify setup:**
   ```bash
   # Check development environment
   ./scripts/dev-workflow.sh status

   # Run tests
   ./scripts/dev-workflow.sh test

   # Start development watch mode
   ./scripts/dev-workflow.sh watch
   ```

## 📋 Development Workflow

### 1. Choose an Issue
- Check [GitHub Issues](https://github.com/guglxni/iora/issues) for open tasks
- Look for issues labeled `good first issue` or `help wanted`
- Comment on the issue to indicate you're working on it

### 2. Create a Branch
```bash
# Create and switch to a new branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 3. Make Changes
- Follow the [Code Style Guidelines](#code-style-guidelines)
- Write tests for new functionality
- Update documentation as needed
- Ensure all tests pass locally

### 4. Test Your Changes
```bash
# Run the full test suite
./scripts/dev-workflow.sh ci

# Run specific test categories
cargo test config     # Configuration tests
cargo test unit_tests # Unit tests
cargo test integration_tests # Integration tests

# Check code quality
./scripts/dev-workflow.sh lint
./scripts/dev-workflow.sh fmt
```

### 5. Commit Your Changes
```bash
# Stage your changes
git add .

# Commit with a clear message
git commit -m "feat: add new feature description

- What was changed
- Why it was changed
- Any breaking changes"
```

### 6. Push and Create Pull Request
```bash
# Push your branch
git push origin feature/your-feature-name

# Create a Pull Request on GitHub
# - Use the PR template
# - Reference any related issues
# - Provide a clear description
```

## 🛠️ Development Tools

### VS Code Setup
The project includes recommended VS Code settings and extensions:

- **rust-analyzer**: Advanced Rust language support
- **roo-cline**: AI-assisted development
- **Prettier**: Code formatting
- **GitLens**: Enhanced Git integration

### Command Line Tools
```bash
# Development workflow
./scripts/dev-workflow.sh build      # Build project
./scripts/dev-workflow.sh run        # Run application
./scripts/dev-workflow.sh test       # Run tests
./scripts/dev-workflow.sh watch      # Development watch mode

# Code quality
./scripts/dev-workflow.sh fmt        # Format code
./scripts/dev-workflow.sh lint       # Run linter
./scripts/dev-workflow.sh fix        # Auto-fix issues

# Services
./scripts/dev-workflow.sh docker-up  # Start services
./scripts/dev-workflow.sh solana-status  # Check Solana setup
./scripts/dev-workflow.sh typesense-status # Check Typesense
```

## 📝 Code Style Guidelines

### Rust Code Style
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `rustfmt` for automatic formatting
- Follow `clippy` linting suggestions
- Use meaningful variable and function names
- Add documentation comments for public APIs

### Commit Message Format
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Testing
- `chore`: Maintenance

Examples:
```
feat: add Gemini API integration
fix: resolve wallet connection timeout
docs: update installation guide
test: add unit tests for config module
```

### Documentation
- Update README.md for new features
- Add doc comments to public functions
- Update API documentation
- Include examples in documentation

## 🧪 Testing

### Test Categories
- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test component interactions
- **Configuration Tests**: Test environment setup
- **E2E Tests**: Test complete workflows

### Running Tests
```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Coverage
./scripts/dev-workflow.sh coverage
```

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functionality() {
        // Arrange
        let input = "test input";

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_output);
    }
}
```

## 🔧 Pull Request Process

### Before Submitting
- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] Linting passes (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages follow guidelines

### PR Template
Please use this template when creating pull requests:

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Tests pass
- [ ] Ready for review

## Related Issues
Closes #issue_number
```

### Review Process
1. Automated checks (CI/CD) must pass
2. At least one reviewer approval required
3. All conversations resolved
4. Squash and merge when approved

## 🏗️ Architecture Guidelines

### Project Structure
```
iora/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library interface
│   └── modules/             # Feature modules
│       ├── config.rs        # Configuration management
│       ├── fetcher.rs       # Data fetching
│       ├── analyzer.rs      # AI analysis
│       ├── rag.rs           # Vector search
│       └── solana.rs        # Blockchain integration
├── tests/                   # Integration tests
├── scripts/                 # Development scripts
├── docs/                    # Documentation
└── assets/                  # Static assets
```

### Module Guidelines
- Keep modules focused and single-purpose
- Use clear, descriptive names
- Export only necessary public APIs
- Include comprehensive error handling
- Add unit tests for all public functions

## 🚨 Issue Reporting

### Bug Reports
When reporting bugs, please include:
- Clear title and description
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version)
- Error messages and stack traces

### Feature Requests
For new features, please provide:
- Clear description of the feature
- Use case and benefits
- Implementation suggestions
- Related issues or PRs

## 📚 Resources

### Documentation
- [README.md](./README.md) - Project overview
- [Development Environment Guide](./docs/development-environment.md)
- [API Documentation](./docs/) - Detailed guides

### External Resources
- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)

## 🤝 Code of Conduct

### Our Standards
- Be respectful and inclusive
- Focus on constructive feedback
- Help newcomers learn
- Maintain professional communication
- Respect differing viewpoints

### Unacceptable Behavior
- Harassment or discrimination
- Offensive language or content
- Personal attacks
- Spam or off-topic content
- Violation of laws or regulations

## 📞 Contact

### Getting Help
- **Issues**: [GitHub Issues](https://github.com/guglxni/iora/issues)
- **Discussions**: [GitHub Discussions](https://github.com/guglxni/iora/discussions)
- **Documentation**: Check the [docs/](./docs/) directory

### Maintainers
- **Aaryan Guglani** - Project Lead
- **GitHub**: [@guglxni](https://github.com/guglxni)

## 🙏 Recognition

Contributors will be recognized in:
- GitHub repository contributors list
- CHANGELOG.md for significant contributions
- Project documentation acknowledgments

---

**Thank you for contributing to I.O.R.A.!** 🚀

Your contributions help build innovative AI-Web3 solutions and advance the blockchain oracle ecosystem.

