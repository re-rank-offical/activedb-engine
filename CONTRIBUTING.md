# Contributing to ActiveDB

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어) · [中文](#中文)

Thank you for your interest in contributing to ActiveDB! This document provides guidelines for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/re-rank/activedb-engine.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests: `cargo test --workspace`
6. Run clippy: `cargo clippy --workspace -- -D warnings`
7. Run fmt: `cargo fmt --all`
8. Commit and push
9. Open a Pull Request

## Development Setup

### Prerequisites
- Rust 1.83+ (stable)
- Docker or Podman (for integration tests)

### Build
```bash
cargo build --workspace
```

### Test
```bash
cargo test --workspace
```

## Code Style

- Follow existing code patterns and conventions
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Write tests for new functionality

## Pull Request Guidelines

- Keep PRs focused on a single change
- Write a clear description of what changed and why
- Reference any related issues
- Ensure CI passes before requesting review

## Reporting Issues

Please report bugs and feature requests via [GitHub Issues](https://github.com/ActiveDB/activedb-engine/issues).

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 license.

---

## 한국어

# ActiveDB 기여 가이드

ActiveDB에 기여해 주셔서 감사합니다! 본 문서는 기여 시 따라야 할 지침을 제공합니다.

### 시작하기

1. 저장소를 포크합니다
2. 포크를 클론합니다: `git clone https://github.com/re-rank/activedb-engine.git`
3. 브랜치를 생성합니다: `git checkout -b feature/your-feature`
4. 변경 사항을 작성합니다
5. 테스트를 실행합니다: `cargo test --workspace`
6. 클리피를 실행합니다: `cargo clippy --workspace -- -D warnings`
7. 포매터를 실행합니다: `cargo fmt --all`
8. 커밋 후 푸시합니다
9. Pull Request를 엽니다

### 개발 환경 구성

#### 필수 도구
- Rust 1.83 이상 (stable)
- Docker 또는 Podman (통합 테스트용)

#### 빌드
```bash
cargo build --workspace
```

#### 테스트
```bash
cargo test --workspace
```

### 코드 스타일

- 기존 코드 패턴과 규칙을 따르세요
- 커밋 전에 `cargo fmt`를 실행하세요
- `cargo clippy`가 경고 없이 통과하도록 하세요
- 새 기능에는 테스트를 작성하세요

### Pull Request 지침

- PR은 단일 변경사항에 집중하세요
- 무엇을 왜 변경했는지 명확히 기술하세요
- 관련 이슈를 참조하세요
- 리뷰 요청 전에 CI가 통과되도록 하세요

### 이슈 보고

버그와 기능 요청은 [GitHub Issues](https://github.com/ActiveDB/activedb-engine/issues)로 보고해 주세요.

### 라이선스

기여함으로써 귀하의 기여가 AGPL-3.0 라이선스 하에 배포되는 것에 동의하는 것으로 간주됩니다.

---

## 中文

# ActiveDB 贡献指南

感谢您有兴趣为 ActiveDB 做出贡献!本文档提供了贡献时应遵循的准则。

### 开始

1. Fork 本仓库
2. 克隆您的 fork:`git clone https://github.com/re-rank/activedb-engine.git`
3. 创建分支:`git checkout -b feature/your-feature`
4. 进行更改
5. 运行测试:`cargo test --workspace`
6. 运行 clippy:`cargo clippy --workspace -- -D warnings`
7. 运行 fmt:`cargo fmt --all`
8. 提交并推送
9. 开启 Pull Request

### 开发环境配置

#### 先决条件
- Rust 1.83+(stable)
- Docker 或 Podman(用于集成测试)

#### 构建
```bash
cargo build --workspace
```

#### 测试
```bash
cargo test --workspace
```

### 代码风格

- 遵循现有的代码模式和约定
- 提交前运行 `cargo fmt`
- 确保 `cargo clippy` 通过且无警告
- 为新功能编写测试

### Pull Request 准则

- 保持 PR 专注于单一更改
- 清晰地描述更改的内容及原因
- 引用相关的 issue
- 在请求评审前确保 CI 通过

### 问题报告

请通过 [GitHub Issues](https://github.com/ActiveDB/activedb-engine/issues) 报告 bug 和功能请求。

### 许可证

通过贡献,您同意您的贡献将依据 AGPL-3.0 许可证发布。
