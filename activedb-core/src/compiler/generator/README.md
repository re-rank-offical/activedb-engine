# Generator Module

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어) · [中文](#中文)

## Overview
The generator module transforms the validated ActiveQL AST into executable Rust code, creating type-safe graph database operations.

## Structure

### Core Components
- **`mod.rs`** - Main generator entry point, defines output structure
- **`utils.rs`** - Helper functions and code generation utilities

### Code Generation Methods (by domain)
- **`schemas.rs`** - Generates Rust structs for nodes, edges, and vectors
- **`queries.rs`** - Generates query functions with proper signatures
- **`migrations.rs`** - Generates migration code for schema evolution
- **`statements.rs`** - Generates statement execution code
- **`traversal_steps.rs`** - Generates graph traversal operations
- **`source_steps.rs`** - Generates source operations (add_n, add_e, n_from_id, n_from_type, etc.)
- **`bool_ops.rs`** - Generates boolean expression evaluators
- **`object_remappings.rs`** - Generates object transformation code
- **`return_values.rs`** - Generates return value processing
- **`tsdisplay.rs`** - TypeScript display utilities

## Generation Flow

1. **Input**: Validated AST from the analyzer module
2. **Schema Generation**: Creates Rust structs for all schema types
3. **Query Generation**: Transforms queries into Rust functions
4. **Migration Generation**: Creates migration execution code
5. **Output**: Complete Rust source code ready for compilation

## Code Generation Patterns
- Uses Rust's `Display` trait for code generation
- Maintains proper indentation and formatting
- Generates idiomatic Rust code with appropriate error handling

---

## 한국어

# 코드 생성기 모듈

### 개요
코드 생성기 모듈은 검증된 ActiveQL AST를 실행 가능한 Rust 코드로 변환하여, 타입 안전한 그래프 데이터베이스 연산을 생성합니다.

### 구조

#### 핵심 컴포넌트
- `mod.rs` — 생성기 메인 진입점. 출력 구조 정의
- `utils.rs` — 도우미 함수 및 코드 생성 유틸리티

#### 코드 생성 메서드 (도메인별)
- `schemas.rs` — 노드 / 엣지 / 벡터에 대한 Rust 구조체 생성
- `queries.rs` — 적절한 시그니처를 갖춘 쿼리 함수 생성
- `migrations.rs` — 스키마 진화를 위한 마이그레이션 코드 생성
- `statements.rs` — 문장 실행 코드 생성
- `traversal_steps.rs` — 그래프 트래버설 연산 코드 생성
- `source_steps.rs` — 소스 연산(add_n, add_e, n_from_id, n_from_type 등) 생성
- `bool_ops.rs` — 불리언 표현식 평가기 생성
- `object_remappings.rs` — 오브젝트 변환 코드 생성
- `return_values.rs` — 반환값 처리 코드 생성
- `tsdisplay.rs` — TypeScript 표기 유틸리티

### 생성 흐름
1. **입력**: 분석기 모듈의 검증된 AST
2. **스키마 생성**: 모든 스키마 타입에 대한 Rust 구조체 생성
3. **쿼리 생성**: 쿼리를 Rust 함수로 변환
4. **마이그레이션 생성**: 마이그레이션 실행 코드 생성
5. **출력**: 컴파일 준비가 완료된 Rust 소스 코드

### 코드 생성 패턴
- 코드 생성에 Rust의 `Display` 트레이트 사용
- 적절한 들여쓰기와 포매팅 유지
- 적절한 오류 처리를 포함한 idiomatic Rust 코드 생성

---

## 中文

# 代码生成器模块

### 概述
代码生成器模块将已验证的 ActiveQL AST 转换为可执行的 Rust 代码,生成类型安全的图数据库操作。

### 结构

#### 核心组件
- `mod.rs` — 生成器主入口,定义输出结构
- `utils.rs` — 辅助函数与代码生成工具

#### 代码生成方法(按领域)
- `schemas.rs` — 为节点、边、向量生成 Rust 结构体
- `queries.rs` — 生成具备正确签名的查询函数
- `migrations.rs` — 为 schema 演进生成迁移代码
- `statements.rs` — 生成语句执行代码
- `traversal_steps.rs` — 生成图遍历操作
- `source_steps.rs` — 生成源操作(add_n、add_e、n_from_id、n_from_type 等)
- `bool_ops.rs` — 生成布尔表达式求值器
- `object_remappings.rs` — 生成对象转换代码
- `return_values.rs` — 生成返回值处理代码
- `tsdisplay.rs` — TypeScript 显示工具

### 生成流程
1. **输入**:来自分析器模块的已验证 AST
2. **Schema 生成**:为所有 schema 类型生成 Rust 结构体
3. **查询生成**:将查询转换为 Rust 函数
4. **迁移生成**:生成迁移执行代码
5. **输出**:可直接编译的完整 Rust 源代码

### 代码生成模式
- 使用 Rust 的 `Display` trait 进行代码生成
- 保持适当的缩进与格式
- 生成包含合理错误处理的 idiomatic Rust 代码
