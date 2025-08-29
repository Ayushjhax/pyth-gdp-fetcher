# Contributing to Sonic SVM + Pyth GDP Dashboard

Thank you for your interest in contributing to our enterprise-grade economic data platform! This document provides guidelines and information for contributors.

## 🎯 **Project Vision**

We're building the future of decentralized economic data infrastructure by combining cutting-edge blockchain technology (Sonic SVM) with reliable oracle networks (Pyth Network) to deliver real-time US economic indicators.

## 🚀 **Getting Started**

### **Prerequisites**
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- Basic understanding of blockchain concepts
- Familiarity with REST APIs

### **Development Environment Setup**
```bash
# Fork and clone the repository
git clone https://github.com/yourusername/sonic-svm-gdp-dashboard.git
cd sonic-svm-gdp-dashboard

# Install dependencies
cargo build

# Run tests
cargo test

# Start development server
cargo run -- --port 3000
```

## 🔧 **Development Workflow**

### **1. Issue Creation**
- Check existing issues before creating new ones
- Use clear, descriptive titles
- Include reproduction steps for bugs
- Tag issues appropriately (bug, feature, enhancement, etc.)

### **2. Branch Strategy**
```bash
# Create feature branch from main
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/your-bug-description
```

### **3. Code Changes**
- Follow Rust coding conventions
- Write comprehensive tests
- Update documentation
- Ensure all tests pass

### **4. Commit Messages**
Use conventional commit format:
```
feat: add new economic indicator endpoint
fix: resolve memory leak in data processing
docs: update API documentation
test: add integration tests for fallback logic
```

### **5. Pull Request Process**
1. Ensure your branch is up to date with main
2. Run all tests and checks
3. Create a descriptive PR title
4. Fill out the PR template completely
5. Request reviews from maintainers

## 📋 **Code Standards**

### **Rust Conventions**
- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Prefer `clippy::pedantic` warnings

### **API Design**
- RESTful endpoint design
- Consistent error response format
- Proper HTTP status codes
- Comprehensive input validation

### **Error Handling**
- Use `anyhow` for error propagation
- Provide meaningful error messages
- Log errors with appropriate levels
- Implement graceful degradation

### **Testing**
- Unit tests for all functions
- Integration tests for API endpoints
- Mock external dependencies
- Test error scenarios

## 🏗️ **Architecture Guidelines**

### **Service Layer**
- Keep business logic in service modules
- Separate concerns (API, business logic, data access)
- Use dependency injection for testability
- Implement proper error boundaries

### **Data Models**
- Use strong typing with Serde
- Validate data at boundaries
- Implement proper serialization
- Handle optional fields gracefully

### **Blockchain Integration**
- Implement proper retry logic
- Handle network timeouts
- Log blockchain interactions
- Implement circuit breakers for failures

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdp_data_parsing() {
        // Test implementation
    }
}
```

### **Integration Tests**
```rust
#[tokio::test]
async fn test_gdp_endpoint() {
    // Test API endpoint
}
```

### **Test Data**
- Use realistic test data
- Mock external services
- Test edge cases
- Validate error conditions

## 📚 **Documentation**

### **Code Documentation**
- Document all public functions
- Include usage examples
- Document error conditions
- Keep examples up to date

### **API Documentation**
- Update OpenAPI specs
- Include request/response examples
- Document error codes
- Provide integration guides

### **README Updates**
- Update installation instructions
- Add new features to feature list
- Update configuration examples
- Keep deployment guides current

## 🔍 **Code Review Process**

### **Review Checklist**
- [ ] Code follows project conventions
- [ ] Tests are comprehensive
- [ ] Documentation is updated
- [ ] Error handling is proper
- [ ] Performance considerations addressed
- [ ] Security implications considered

### **Review Guidelines**
- Be constructive and respectful
- Focus on code quality and correctness
- Suggest improvements when possible
- Approve only when satisfied

## 🚀 **Deployment & Release**

### **Release Process**
1. Update version in `Cargo.toml`
2. Update changelog
3. Create release tag
4. Deploy to staging environment
5. Run integration tests
6. Deploy to production

### **Environment Management**
- Use environment variables for configuration
- Document all required environment variables
- Provide example configuration files
- Use different configs for dev/staging/prod

## 🤝 **Community Guidelines**

### **Communication**
- Be respectful and inclusive
- Use clear, professional language
- Provide constructive feedback
- Help other contributors

### **Support**
- Answer questions in issues
- Help with setup problems
- Share knowledge and best practices
- Mentor new contributors

## 📞 **Getting Help**

### **Resources**
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [Sonic SVM Docs](https://docs.sonic.game)
- [Pyth Network Docs](https://docs.pyth.network)

### **Contact**
- GitHub Issues for technical questions
- Discord for community discussions
- Email for private matters

## 🏆 **Recognition**

Contributors will be recognized in:
- Project README
- Release notes
- Contributor hall of fame
- Community acknowledgments

---

**Thank you for contributing to the future of decentralized economic data!** 🚀

*Together, we're building the infrastructure that will power the next generation of DeFi and economic applications.*
