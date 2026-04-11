# Contributing to activedb

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어-요약) · [中文](#中文概要)
>
> The main guide below is in English. Concise Korean and Chinese summaries are provided at the bottom of this document. For full details, refer to the English sections.

## Overview
activedb is a high-performance graph-vector database built in Rust, optimized for RAG and AI applications. It combines graph traversals, vector similarity search, and full-text search in a single database.

We welcome contributions from the community! This guide will help you get started with contributing to activedb.

## How to Contribute

### Reporting Issues
- Check existing [GitHub Issues](https://github.com/HelixDB/helix-db/issues) to avoid duplicates
- Use a clear, descriptive title
- Include steps to reproduce for bugs
- Provide system information (OS, Rust version, activedb version)
- Add relevant logs or error messages

### Contribution Workflow
1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/helix-db.git
   cd helix-db
   ```
3. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make your changes** following our coding guidelines
5. **Commit your changes** with clear, descriptive commit messages:
   ```bash
   git commit -m "feat: add new feature description"
   ```
6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Open a Pull Request** against the `main` branch
8. **Respond to feedback** from reviewers

### Pull Request Guidelines
- Link related issues in the PR description
- Ensure all tests pass
- Add tests for new features
- Update documentation if needed
- Keep PRs focused on a single feature or fix
- Write clear commit messages following conventional commits format

## Prerequisites and Development Setup

### Required Tools
- **Rust**: 1.75.0 or later (install via [rustup](https://rustup.rs/))
- **Cargo**: Comes with Rust
- **Git**: For version control

### Optional Tools
- **cargo-watch**: For development auto-reloading
- **cargo-nextest**: Faster test runner
- **rust-analyzer**: IDE support

### Building the Project
1. **Clone the repository**:
   ```bash
   git clone https://github.com/HelixDB/helix-db.git
   cd helix-db
   ```

2. **Build all components**:
   ```bash
   cargo build
   ```

3. **Build in release mode** (optimized):
   ```bash
   cargo build --release
   ```

### Building Specific Components
- **CLI only**: `cargo build -p helix-cli`
- **Core database**: `cargo build -p helix-db`
- **Container**: `cargo build -p helix-container`

### Running activedb Locally
1. Install the CLI (development version):
   ```bash
   cargo install --path helix-cli
   ```

2. Initialize a test project:
   ```bash
   mkdir test-project && cd test-project
   helix init
   ```

3. Deploy locally:
   ```bash
   helix push dev
   ```

## Project Structure

### Core Components

#### `/helix-db/` - Main Database Library
The heart of activedb containing all database functionality.

- **`helix_engine/`** - Database engine implementation
  - `bm25/` - Full-text search using BM25 algorithm
  - `storage_core/` - LMDB-based storage backend via heed3
  - `traversal_core/` - Graph traversal operations and query execution
  - `vector_core/` - Vector storage and HNSW similarity search
  - `tests/` - Integration and unit tests
  - `types.rs` - Core type definitions
  - `macros.rs` - Helper macros

- **`helix_gateway/`** - Network layer
  - `builtin/` - Built-in query handlers (node_by_id, all_nodes_and_edges, node_connections, nodes_by_label)
  - `embedding_providers/` - Integration with embedding services
  - `router/` - Request routing to handlers
  - `worker_pool/` - Concurrent request processing (formerly thread_pool)
  - `mcp/` - Model Context Protocol support
  - `gateway.rs` - Main gateway implementation
  - `introspect_schema.rs` - Schema introspection utilities

- **`helixc/`** - Query compiler
  - `parser/` - Parser for `.hx` files (using Pest grammar)
  - `analyzer/` - Type checking, validation, and diagnostics
  - `generator/` - Rust code generation from parsed queries

- **`grammar.pest`** - 295-line Pest grammar defining HQL syntax

- **`protocol/`** - Wire protocol and data types

- **`utils/`** - Shared utilities across the codebase

#### `/helix-container/` - Runtime Container
The server process that hosts compiled queries and handles requests.

**Files:**
- `main.rs` - Initializes graph engine and HTTP gateway
- `queries.rs` - Generated code placeholder (populated during build)
- `docker-compose.yml` - Container orchestration configuration
- `Dockerfile` - Development container image

**Architecture:**
- Loads compiled queries via inventory crate route discovery
- Creates HelixGraphEngine with LMDB storage backend
- Starts HelixGateway on configured port (default: 6969)
- Routes HTTP requests to registered handlers

**Environment Variables:**
- `HELIX_DATA_DIR` - Database storage location
- `HELIX_PORT` - Server port

#### `/helix-cli/` - Command-Line Interface
User-facing CLI for managing activedb instances and deployments.

**Directory Structure:**
```
helix-cli/
├── src/
│   ├── commands/           # CLI command implementations
│   │   ├── integrations/   # Cloud deployment integrations
│   │   │   ├── docker_hub.rs
│   │   │   ├── ecr.rs      # AWS ECR
│   │   │   ├── fly.rs      # Fly.io
│   │   │   ├── ghcr.rs     # GitHub Container Registry
│   │   │   └── helix.rs    # Helix Cloud
│   │   ├── add.rs         # Add dependencies
│   │   ├── auth.rs        # Authentication (login/logout/create-key)
│   │   ├── build.rs       # Build queries
│   │   ├── check.rs       # Validate schema and queries
│   │   ├── compile.rs     # Compile queries
│   │   ├── delete.rs      # Delete instances
│   │   ├── init.rs        # Initialize new projects
│   │   ├── metrics.rs     # Metrics configuration
│   │   ├── migrate.rs     # Database migrations
│   │   ├── prune.rs       # Cleanup unused resources
│   │   ├── pull.rs        # Pull from cloud deployments
│   │   ├── push.rs        # Push to cloud deployments
│   │   ├── start.rs       # Start instances
│   │   ├── status.rs      # Instance status
│   │   ├── stop.rs        # Stop instances
│   │   └── update.rs      # Update CLI
│   ├── tests/             # CLI tests
│   ├── config.rs          # Configuration management
│   ├── docker.rs          # Docker integration
│   ├── errors.rs          # Error handling
│   ├── lib.rs             # Library interface
│   ├── main.rs            # Entry point
│   ├── metrics_sender.rs  # Metrics collection
│   ├── project.rs         # Project management
│   ├── update.rs          # Self-update functionality
│   └── utils.rs           # Utilities
```

**Available Commands:**
- `helix add` - Add dependencies to project
- `helix auth` - Authentication management (login/logout/create-key)
- `helix build` - Build queries without deploying
- `helix check` - Validate schema and query syntax
- `helix compile` - Compile queries to Rust code
- `helix delete` - Remove instance and data
- `helix init` - Create new project with template files
- `helix metrics` - Configure metrics collection (full/basic/off/status)
- `helix migrate` - Run database migrations
- `helix prune` - Clean up unused resources
- `helix pull` - Pull deployment from cloud
- `helix push` - Push deployment to cloud (dev/staging/prod)
- `helix start` - Start stopped instances
- `helix status` - Show instance status
- `helix stop` - Stop running instances
- `helix update` - Update CLI to latest version

**Deployment Integrations:**
- Helix Cloud (managed hosting)
- AWS ECR (Elastic Container Registry)
- Fly.io
- Docker Hub
- GitHub Container Registry (GHCR)
- Local deployment

**Build & Deploy Flow:**
1. Read `.hx` files (schema.hx, queries.hx)
2. Parse and analyze using helixc
3. Generate Rust code with handler functions
4. Write to container/src/queries.rs
5. Build release binary with optimizations
6. Push to target deployment (cloud or local)

### Supporting Components

#### `/helix-macros/` - Procedural Macros
Procedural macros for activedb including route registration and code generation utilities.

#### `/hql-tests/` - HQL Test Suite
Test files for the Helix Query Language (HQL).

#### `/metrics/` - Performance Metrics
Performance benchmarking and metrics collection.

## Key Concepts

### Query Language
activedb uses a custom query language defined in `.hx` files:
```
QUERY addUser(name: String, age: I64) =>
   user <- AddN<User({name: name, age: age})
   RETURN user
```

### Data Model
- **Nodes** (N::) - Graph vertices with properties
- **Edges** (E::) - Relationships between nodes
- **Vectors** (V::) - High-dimensional embeddings

### Operations
- **Graph traversals**: `In`, `Out`, `InE`, `OutE`
- **Vector search**: HNSW-based similarity search
- **Text search**: BM25 full-text search
- **CRUD**: `AddN`, `AddE`, `Update`, `Drop`

## Architecture Flow

1. **Definition**: Write queries in `.hx` files
2. **Compilation**: `helix check` parses and validates
3. **Deployment**: `helix deploy` loads into container
4. **Execution**: Gateway routes requests to compiled handlers
5. **Storage**: LMDB handles persistence with ACID guarantees

## Development Guidelines

### Code Style
- Prefer functional patterns (pattern matching, iterators, closures)
- Document code inline - no separate docs needed
- Minimize dependencies
- Use asserts liberally in production code

### Linting

Run Clippy to check code quality:
```bash
./clippy_check.sh
```

The `clippy_check.sh` script at the repository root runs clippy with project-specific rules:
- Treats warnings as errors
- Excludes `hql-tests` crate
- Can run in dashboard mode with additional features

### Testing

activedb has a comprehensive test suite organized across multiple levels:

#### Test Structure

**Unit Tests** (within `src/` directories)
- `/helix-db/src/helix_engine/tests/` - Engine unit tests
- `/helix-db/src/helix_gateway/tests/` - Gateway unit tests
- Inline `#[cfg(test)]` modules throughout the codebase

**Integration Tests**
- `/helix-db/tests/` - Database integration tests

**CLI Tests**
- `/helix-cli/src/tests/` - Command-line interface tests
  - `check_tests.rs` - Validation testing
  - `compile_tests.rs` - Compilation testing
  - `init_tests.rs` - Project initialization
  - `project_tests.rs` - Project management

**HQL End-to-End Tests**
- `/hql-tests/tests/` - 54+ test directories covering:
  - Graph operations (add_n, add_e, traversals)
  - Vector search (search_v_with_embed)
  - Text search (search_bm25)
  - Aggregations and counting
  - Migrations
  - Cloud queries
  - Rerankers
  - Knowledge graphs
  - Benchmarks

**Benchmark Tests**
- `/helix-db/benches/bm25_benches.rs` - Full-text search performance
- `/helix-db/benches/hnsw_benches.rs` - Vector search performance

#### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p helix-db
cargo test -p helix-cli

# Run HQL tests
cd hql-tests
./test.sh

# Run benchmarks
cargo test --benches
```

#### Testing Guidelines
- Write tests for all new features
- Include both positive and negative test cases
- Add benchmarks before optimizing performance-critical code
- Ensure tests pass locally before opening PR
- DST (Deterministic Simulation Testing) coming soon

### Performance
- Currently 1000x faster than Neo4j for graph operations
- On par with Qdrant for vector search
- LMDB provides memory-mapped performance

## Communication Channels

### Getting Help
- **Discord**: Join our [Discord community](https://discord.gg/2stgMPr5BD) for real-time discussions, questions, and support
- **GitHub Issues**: Report bugs or request features at [github.com/HelixDB/helix-db/issues](https://github.com/HelixDB/helix-db/issues)
- **Documentation**: Check [docs.helix-db.com](https://docs.helix-db.com) for comprehensive guides
- **Twitter/X**: Follow [@helixdb](https://x.com/helixdb) for updates and announcements

### Before You Ask
- Search existing GitHub issues and Discord for similar questions
- Check the documentation for relevant guides
- Try to create a minimal reproducible example
- Include error messages, logs, and system information

### Community Guidelines
- Be respectful and constructive
- Help others when you can
- Share your use cases and learnings
- Follow our [Code of Conduct](CODE_OF_CONDUCT.md)

## Code Review Process

### What Reviewers Look For
- **Correctness**: Does the code work as intended?
- **Tests**: Are there adequate tests? Do they pass?
- **Code style**: Does it follow Rust and activedb conventions?
- **Performance**: Are there obvious performance issues?
- **Documentation**: Are complex parts explained?
- **Scope**: Is the PR focused on a single feature/fix?

### Common Reasons PRs Get Rejected
- Failing tests or CI checks
- No tests for new functionality
- Breaks existing functionality
- Code style violations
- Too broad in scope (mixing multiple unrelated changes)
- Missing documentation for complex features
- Performance regressions without justification

### How to Respond to Feedback
- Address all reviewer comments
- Ask for clarification if feedback is unclear
- Make requested changes in new commits (don't force push during review)
- Mark conversations as resolved after addressing them
- Be patient and respectful - reviewers are volunteers

### Review Timeline
- Initial response: Usually within 2-3 days
- Follow-up reviews: 1-2 days after updates
- Complex PRs may take longer
- Feel free to ping on Discord if your PR hasn't been reviewed after a week

## Getting Started

1. Install CLI: `curl -sSL "https://install.helix-db.com" | bash`
2. Install Helix: `helix install`
3. Initialize project: `helix init --path <path>`
4. Write queries in `.hx` files
5. Deploy: `helix deploy`

## License
AGPL (Affero General Public License)

For commercial support: founders@helix-db.com

---

## 한국어 요약

### 개요
activedb는 Rust로 구축된 고성능 그래프-벡터 데이터베이스로, RAG와 AI 애플리케이션에 최적화되어 있습니다. 그래프 탐색, 벡터 유사도 검색, 전문 검색을 하나의 데이터베이스에 통합합니다. 커뮤니티의 기여를 환영합니다.

### 기여 방법
- **이슈 보고**: 중복 방지를 위해 기존 GitHub Issues를 먼저 확인하고, 명확한 제목과 재현 단계, 시스템 정보(OS, Rust 버전, activedb 버전)를 포함해 주세요.
- **Pull Request 워크플로우**: 저장소를 포크 → `main`에서 피처 브랜치 생성 → 변경사항 작성 → 명확한 커밋 메시지 작성 → 포크에 푸시 → `main`을 대상으로 PR 오픈 → 리뷰 피드백에 응답.
- **PR 지침**: 관련 이슈를 링크하고, 모든 테스트가 통과하며, 새 기능에는 테스트를 추가하고, PR은 단일 기능/수정에 집중하며, Conventional Commits 형식의 커밋 메시지를 작성하세요.

### 개발 환경
- **필수**: Rust 1.75.0 이상, Cargo, Git
- **선택**: cargo-watch, cargo-nextest, rust-analyzer
- **빌드**: `cargo build` / 릴리스는 `cargo build --release`
- **로컬 실행**: `cargo install --path activedb-cli` 후 `activedb init` 및 `activedb push dev`

### 테스트
`cargo test --workspace`로 전체 테스트를 실행합니다. 단위 테스트, 통합 테스트, CLI 테스트, AQL E2E 테스트, 벤치마크로 구성되어 있습니다. 새 기능에는 반드시 테스트를 작성하고, 성능에 민감한 코드는 최적화 전에 벤치마크를 추가하세요.

### 소통 채널
- **Discord**: 실시간 논의 및 지원
- **GitHub Issues**: 버그 리포트 및 기능 요청
- **문서**: 포괄적인 가이드 제공

### 코드 리뷰 기준
정확성, 충분한 테스트, 코드 스타일(Rust 및 activedb 컨벤션), 성능, 문서화, 단일 범위에 초점을 둡니다.

---

## 中文概要

### 概述
activedb 是使用 Rust 构建的高性能图-向量数据库,专为 RAG 和 AI 应用优化。它在单一数据库中结合了图遍历、向量相似度搜索和全文检索。我们欢迎社区贡献。

### 贡献方式
- **报告问题**: 为避免重复,请先检查现有的 GitHub Issues,并提供清晰的标题、复现步骤以及系统信息(操作系统、Rust 版本、activedb 版本)。
- **Pull Request 工作流**: Fork 仓库 → 从 `main` 创建功能分支 → 进行更改 → 编写清晰的提交信息 → 推送到您的 fork → 针对 `main` 开启 PR → 响应评审反馈。
- **PR 准则**: 链接相关 issue,确保所有测试通过,为新功能添加测试,PR 专注于单一功能或修复,使用 Conventional Commits 格式的提交信息。

### 开发环境
- **必需**: Rust 1.75.0 或更新版本、Cargo、Git
- **可选**: cargo-watch、cargo-nextest、rust-analyzer
- **构建**: `cargo build`;发布版为 `cargo build --release`
- **本地运行**: `cargo install --path activedb-cli`,然后执行 `activedb init` 和 `activedb push dev`

### 测试
使用 `cargo test --workspace` 运行全部测试。测试包含单元测试、集成测试、CLI 测试、AQL 端到端测试和基准测试。新功能必须编写测试;在优化对性能敏感的代码前,请先添加基准测试。

### 沟通渠道
- **Discord**: 实时讨论与支持
- **GitHub Issues**: Bug 报告与功能请求
- **文档**: 提供完整的使用指南

### 代码评审标准
正确性、充分的测试、代码风格(Rust 与 activedb 约定)、性能、文档质量,以及专注于单一范围。
