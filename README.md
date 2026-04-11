# ActiveDB

> An open-source graph-vector database engine built from scratch in Rust.

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어) · [中文](#中文)

ActiveDB unifies graph, vector, and full-text (BM25) search in a single engine. Build RAG pipelines on one platform instead of stitching together a graph DB, vector DB, and search engine.

## Features

- **Graph + Vector + BM25** — Graph traversal, vector similarity, and keyword search on a single storage engine
- **ActiveQL** — PEG-based, type-safe query language. Catches type errors at compile time and auto-generates TypeScript types
- **HNSW Vector Index** — AVX2 SIMD-accelerated cosine similarity. LMDB-backed persistent storage
- **17 Graph Algorithms** — Built-in analytics including PageRank, Louvain, Betweenness Centrality, and more
- **Hybrid Reranking** — RRF, MMR, and CrossEncoder strategies for fusing BM25 + vector results
- **Built-in MCP Server** — Model Context Protocol support so AI agents can explore the graph session-by-session
- **Built-in Embeddings** — Vectorize text with the `Embed` function — no external embedding service required
- **Rust + LMDB** — Zero-copy reads, core-affine worker pool, and `bumpalo` arena allocation for ultra-low latency

## Quick Start

### Install the CLI

```bash
curl -sSL "https://install.activedb.dev" | bash
```

### Create a project

```bash
mkdir my-app && cd my-app
activedb init
```

### Write schema & queries

Define nodes, edges, vectors, and queries in `.hx` files.

```
// Node definition
N::User {
    INDEX name: String,
    age: U32
}

// Edge definition
E::Follows {
    From: User,
    To: User,
    Properties: {
        since: String
    }
}

// Vector node definition
V::Document {
    INDEX title: String,
    content: String
}

// Query
QUERY getUser(user_name: String) =>
    user <- N<User>({name: user_name})
    RETURN user

QUERY getUserFollowers(user_name: String) =>
    user <- N<User>({name: user_name})
    followers <- user.In<Follows>
    RETURN followers
```

### Validate & deploy

```bash
activedb check    # Type check & diagnostics
activedb build    # Compile
activedb push dev # Deploy instance
```

### Query via SDK

```typescript
import ActiveDB from "activedb-ts";

const client = new ActiveDB();

await client.query("addUser", { name: "Alice", age: 28 });

const user = await client.query("getUser", { user_name: "Alice" });
console.log(user);
```

## Architecture

```
activedb-engine/
├── activedb-core/       # Core engine
│   ├── compiler/        #   AQL parser → analyzer → code generator
│   ├── engine/          #   Storage, HNSW vector, BM25, graph algorithms, reranker
│   ├── gateway/         #   Axum HTTP server, router, worker pool, MCP server
│   └── protocol/        #   Data serialization protocol
├── activedb-cli/        # CLI (init, build, check, push, auth, logs, ...)
├── activedb-container/  # Deployment instance runtime
├── activedb-macros/     # Procedural macros
├── aql-tests/           # AQL integration tests
└── metrics/             # Metrics collection
```

## Graph Algorithms

| Category | Algorithms |
|----------|------------|
| **Centrality** | PageRank, Degree, Betweenness, Closeness, Eigenvector, Harmonic |
| **Community** | Louvain, Label Propagation, Connected Components, K-Core, Triangle Count, Clustering Coefficient |
| **Paths** | Cycle Detection, Max Flow, Minimum Spanning Tree |
| **Similarity** | Jaccard, Cosine Neighbor |

## CLI Commands

| Command | Description |
|---------|-------------|
| `activedb init` | Create a new project |
| `activedb check` | Type-check schemas & queries |
| `activedb build` | Compile AQL to Rust code |
| `activedb push <env>` | Deploy instance |
| `activedb start / stop / restart` | Manage local instances |
| `activedb logs` | View instance logs (with TUI) |
| `activedb status` | Inspect instance status |
| `activedb auth` | GitHub OAuth authentication |
| `activedb backup` | Back up data |
| `activedb dashboard` | Open the web dashboard |
| `activedb migrate` | Schema migrations |

## Docker

```bash
# Production
docker build -f docker/Dockerfile -t activedb .
docker run -p 6969:6969 activedb

# Development
docker build -f docker/Dockerfile.dev -t activedb-dev .
docker run -p 6969:6969 activedb-dev
```

## Build

```bash
# Full build
cargo build --workspace

# Release build
cargo build --release --workspace

# Test
cargo test --workspace

# Clippy
cargo clippy --workspace -- -D warnings
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `server` | HTTP server + compiler + vectors (default) |
| `production` | API key auth + server |
| `compiler` | AQL compiler only |
| `vectors` | Vector search (cosine similarity) |
| `bench` | Polars-based benchmarks |
| `dev` | Debug output + server + benchmarks |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[AGPL-3.0](LICENSE) — If you modify the source code to provide a service, you must also publish the modified source.

For a managed option, check out [ActiveDB Cloud](https://cloud.activedb.dev).

---

## 한국어

# ActiveDB

> Rust로 처음부터 구축한 오픈소스 그래프-벡터 데이터베이스 엔진.

ActiveDB는 그래프, 벡터, 전문 검색(BM25)을 하나의 엔진에 통합한 데이터베이스입니다. AI 애플리케이션에서 별도의 그래프 DB, 벡터 DB, 검색 엔진을 조합할 필요 없이 단일 플랫폼으로 RAG 파이프라인을 구축할 수 있습니다.

### 주요 특징

- **Graph + Vector + BM25** — 하나의 스토리지 엔진에서 그래프 탐색, 벡터 유사도 검색, 키워드 검색을 동시에 수행
- **ActiveQL** — PEG 기반 타입 안전 쿼리 언어. 컴파일 타임에 타입 오류를 잡아내고 TypeScript 타입을 자동 생성
- **HNSW 벡터 인덱스** — AVX2 SIMD 가속 코사인 유사도. LMDB 기반 영속 저장
- **17개 그래프 알고리즘** — PageRank, Louvain, Betweenness Centrality 등 분석 알고리즘 내장
- **하이브리드 리랭킹** — RRF, MMR, CrossEncoder 세 가지 전략으로 BM25 + 벡터 결과 통합
- **MCP 서버 내장** — AI 에이전트가 세션 기반으로 그래프를 탐색할 수 있는 Model Context Protocol 지원
- **내장 임베딩** — 외부에서 벡터를 생성할 필요 없이 `Embed` 함수로 텍스트를 벡터화
- **Rust + LMDB** — 제로카피 읽기, 코어 어피니티 워커풀, `bumpalo` 아레나 할당으로 초저지연 달성

### 빠른 시작

```bash
# CLI 설치
curl -sSL "https://install.activedb.dev" | bash

# 프로젝트 생성
mkdir my-app && cd my-app
activedb init

# 검증 & 배포
activedb check    # 타입 검증 & 오류 진단
activedb build    # 컴파일
activedb push dev # 인스턴스 배포
```

### 그래프 알고리즘

| 카테고리 | 알고리즘 |
|---------|---------|
| **중심성** | PageRank, Degree, Betweenness, Closeness, Eigenvector, Harmonic |
| **커뮤니티** | Louvain, Label Propagation, Connected Components, K-Core, Triangle Count, Clustering Coefficient |
| **경로** | Cycle Detection, Max Flow, Minimum Spanning Tree |
| **유사도** | Jaccard, Cosine Neighbor |

### 기여하기

[CONTRIBUTING.md](CONTRIBUTING.md)를 참고해주세요.

### 라이선스

[AGPL-3.0](LICENSE) — 소스 코드를 수정하여 서비스를 제공하는 경우 수정본도 공개해야 합니다.

매니지드 서비스가 필요하다면 [ActiveDB Cloud](https://cloud.activedb.dev)를 확인하세요.

---

## 中文

# ActiveDB

> 使用 Rust 从零构建的开源图-向量数据库引擎。

ActiveDB 将图、向量和全文(BM25)搜索统一到一个引擎中。您无需组合单独的图数据库、向量数据库和搜索引擎,即可在单一平台上构建 RAG 流水线。

### 核心特性

- **图 + 向量 + BM25** — 在同一存储引擎上同时执行图遍历、向量相似度检索和关键字搜索
- **ActiveQL** — 基于 PEG 的类型安全查询语言。在编译期捕获类型错误并自动生成 TypeScript 类型
- **HNSW 向量索引** — AVX2 SIMD 加速的余弦相似度。基于 LMDB 的持久化存储
- **17 种图算法** — 内置 PageRank、Louvain、介数中心性等分析算法
- **混合重排序** — RRF、MMR、CrossEncoder 三种策略融合 BM25 与向量结果
- **内置 MCP 服务器** — 支持 Model Context Protocol,AI 智能体可基于会话探索图
- **内置嵌入** — 使用 `Embed` 函数将文本向量化,无需外部嵌入服务
- **Rust + LMDB** — 零拷贝读取、核心亲和工作池以及 `bumpalo` arena 分配,实现超低延迟

### 快速开始

```bash
# 安装 CLI
curl -sSL "https://install.activedb.dev" | bash

# 创建项目
mkdir my-app && cd my-app
activedb init

# 验证和部署
activedb check    # 类型检查与诊断
activedb build    # 编译
activedb push dev # 部署实例
```

### 图算法

| 类别 | 算法 |
|------|------|
| **中心性** | PageRank、度、介数、接近度、特征向量、调和中心性 |
| **社区** | Louvain、标签传播、连通分量、K-Core、三角形计数、聚类系数 |
| **路径** | 环检测、最大流、最小生成树 |
| **相似度** | Jaccard、余弦邻居 |

### 贡献

请参阅 [CONTRIBUTING.md](CONTRIBUTING.md)。

### 许可证

[AGPL-3.0](LICENSE) — 如果您修改源代码以提供服务,必须同时公开修改后的源代码。

如需托管方案,请查看 [ActiveDB Cloud](https://cloud.activedb.dev)。
