# Contributing to Lièvre

We welcome contributions! Here's how to get started.

## Development Setup

### Prerequisites

- Rust ≥ 1.75
- Node.js ≥ 20
- Docker + Docker Compose

### Clone and Build

```bash
git clone https://github.com/avi-xe/lievre.git
cd lievre

# Start services
docker compose up -d

# Run tests
cargo test --workspace --lib
cd frontend && npm run lint && npm run typecheck
```

### Project Structure

See [Architecture Overview](Architecture-Overview.md) for the full layout.

## Making Changes

### Branch Naming

- `feat/description` — New features
- `fix/description` — Bug fixes
- `docs/description` — Documentation

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add FIT file import
fix: like toggle returns 400 on duplicate
docs: update federation guide
```

### Code Style

**Rust:**
- Run `cargo fmt --all` before committing
- Run `cargo clippy --workspace --all-targets` for warnings
- Follow existing patterns in the codebase

**TypeScript:**
- Run `npm run lint` in `frontend/`
- Run `npm run typecheck` for type errors
- Use existing component patterns

### Testing

- **Unit tests:** `cargo test --workspace --lib`
- **E2E tests:** `cd e2e && node test-e2e.mjs` (requires Docker services)
- **New features:** Add unit tests for business logic
- **Bug fixes:** Add a test that reproduces the bug

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Ensure CI passes (lint, tests, fmt)
5. Submit a PR with a clear description
6. Wait for review

### PR Description Template

```markdown
## What

Brief description of the change.

## Why

Why this change is needed.

## How

How the change works.

## Testing

How to verify the change.

## Checklist
- [ ] Tests pass locally
- [ ] Code is formatted
- [ ] No clippy warnings
- [ ] Documentation updated (if applicable)
```

## Areas for Contribution

### High Priority

- [ ] FIT file import
- [ ] Data export (GPX, FIT, JSON)
- [ ] HTTP Signatures for federation
- [ ] Privacy zones

### Medium Priority

- [ ] Activity stats dashboard
- [ ] Personal records
- [ ] Push notifications
- [ ] Segment leaderboards

### Low Priority

- [ ] Gear tracking UI
- [ ] Heatmap visualization
- [ ] Training load calculation
- [ ] Mobile PWA improvements

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions
- Check existing issues before creating new ones

---

**See also:** [Architecture Overview](Architecture-Overview.md) | [API Reference](API-Reference.md)
