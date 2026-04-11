# Parser Module

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어) · [中文](#中文)

## Overview
The parser module transforms ActiveQL (AQL) source code into an Abstract Syntax Tree (AST) using the Pest parser generator framework.

## Structure

### Core Components
- **`mod.rs`** - Main parser entry point, orchestrates parsing of schemas, queries, and migrations
- **`grammar.pest`** - Pest grammar defining AQL syntax rules
- **`types.rs`** - AST node definitions and data structures
- **`location.rs`** - Location tracking for error reporting

### Parse Methods (by domain)
- **`schema_parse_methods.rs`** - Parses node, edge, and vector schema definitions
- **`query_parse_methods.rs`** - Parses query definitions with parameters and statements
- **`migration_parse_methods.rs`** - Parses schema migration definitions
- **`traversal_parse_methods.rs`** - Parses traversal (anonymous/id/starting node, vector or edge etc)
- **`graph_step_parse_methods.rs`** - Parses graph step operations (object remapping/order by/where/range etc)
- **`creation_step_parse_methods.rs`** - Parses node/edge/vector creation operations
- **`expression_parse_methods.rs`** - Parses expressions e.g. assignment, for loop, boolean expressions etc
- **`object_parse_methods.rs`** - Parses object fields for remappings/parameters/item creations etc
- **`return_value_parse_methods.rs`** - Parses return statements and remappings

## Parsing Flow

1. **Input**: AQL files containing schemas, queries, and migrations
2. **Lexing**: Pest tokenizes input according to `grammar.pest` rules
3. **AST Construction**:
   - Schemas parsed first (establishing type definitions)
   - Migrations parsed second (for schema evolution)
   - Queries parsed last (can reference schema types)
4. **Output**: `Source` struct containing parsed schemas, migrations, and queries

## Key Types

- `Source` - Top-level container for all parsed content
- `Schema` - Contains node, edge, and vector type definitions
- `Query` - Parsed query with parameters, statements, and return values
- `Migration` - Schema version transition definitions

## Error Handling
- `ParserError` enum handles parse errors, lex errors, and schema validation
- Location tracking enables precise error reporting with file/line/column info

---

## 한국어

# 파서 모듈

### 개요
파서 모듈은 Pest 파서 생성기 프레임워크를 사용하여 ActiveQL(AQL) 소스 코드를 추상 구문 트리(AST)로 변환합니다.

### 구조

#### 핵심 컴포넌트
- **`mod.rs`** — 파서 메인 진입점. 스키마, 쿼리, 마이그레이션 파싱을 조율합니다
- **`grammar.pest`** — AQL 문법을 정의하는 Pest 문법 파일
- **`types.rs`** — AST 노드 정의 및 자료 구조
- **`location.rs`** — 오류 리포트를 위한 위치 추적

#### 파싱 메서드 (도메인별)
- `schema_parse_methods.rs` — 노드 / 엣지 / 벡터 스키마 정의 파싱
- `query_parse_methods.rs` — 파라미터 및 문장이 포함된 쿼리 정의 파싱
- `migration_parse_methods.rs` — 스키마 마이그레이션 정의 파싱
- `traversal_parse_methods.rs` — 익명/ID/시작 노드, 벡터, 엣지 등 트래버설 파싱
- `graph_step_parse_methods.rs` — 그래프 스텝 연산(리매핑, order by, where, range 등) 파싱
- `creation_step_parse_methods.rs` — 노드 / 엣지 / 벡터 생성 연산 파싱
- `expression_parse_methods.rs` — 할당, for 루프, 불리언 표현식 등 파싱
- `object_parse_methods.rs` — 리매핑 / 파라미터 / 아이템 생성용 오브젝트 필드 파싱
- `return_value_parse_methods.rs` — return 문과 리매핑 파싱

### 파싱 흐름

1. **입력**: 스키마, 쿼리, 마이그레이션이 포함된 AQL 파일
2. **렉싱**: `grammar.pest` 규칙에 따라 Pest가 입력을 토큰화
3. **AST 구성**: 스키마(타입 정의) → 마이그레이션(스키마 진화) → 쿼리(스키마 타입 참조) 순으로 파싱
4. **출력**: 파싱 결과를 담은 `Source` 구조체

### 주요 타입
- `Source` — 모든 파싱 콘텐츠의 최상위 컨테이너
- `Schema` — 노드 / 엣지 / 벡터 타입 정의
- `Query` — 파라미터 / 문장 / 반환값이 포함된 파싱된 쿼리
- `Migration` — 스키마 버전 전환 정의

### 오류 처리
- `ParserError` enum이 파싱 오류, 렉스 오류, 스키마 검증 오류를 처리
- 위치 추적을 통해 파일/라인/컬럼 정보로 정밀한 오류 리포트 제공

---

## 中文

# 解析器模块

### 概述
解析器模块使用 Pest 解析器生成框架,将 ActiveQL(AQL)源码转换为抽象语法树(AST)。

### 结构

#### 核心组件
- **`mod.rs`** — 解析器主入口,协调对 schema、query 和 migration 的解析
- **`grammar.pest`** — 定义 AQL 语法规则的 Pest 文法
- **`types.rs`** — AST 节点定义与数据结构
- **`location.rs`** — 用于错误报告的位置跟踪

#### 解析方法(按领域)
- `schema_parse_methods.rs` — 解析节点、边、向量 schema 定义
- `query_parse_methods.rs` — 解析包含参数与语句的查询定义
- `migration_parse_methods.rs` — 解析 schema 迁移定义
- `traversal_parse_methods.rs` — 解析匿名/ID/起始节点、向量、边等遍历
- `graph_step_parse_methods.rs` — 解析图步骤操作(对象重映射、order by、where、range 等)
- `creation_step_parse_methods.rs` — 解析节点/边/向量创建操作
- `expression_parse_methods.rs` — 解析赋值、for 循环、布尔表达式等
- `object_parse_methods.rs` — 解析用于重映射/参数/项创建的对象字段
- `return_value_parse_methods.rs` — 解析 return 语句与重映射

### 解析流程

1. **输入**:包含 schema、query、migration 的 AQL 文件
2. **词法分析**:Pest 根据 `grammar.pest` 规则对输入进行词法化
3. **AST 构建**:先解析 schema(确立类型定义),再解析 migration(schema 演进),最后解析 query(可引用 schema 类型)
4. **输出**:包含已解析内容的 `Source` 结构体

### 关键类型
- `Source` — 所有解析内容的顶层容器
- `Schema` — 包含节点、边与向量类型定义
- `Query` — 含参数、语句和返回值的已解析查询
- `Migration` — schema 版本迁移定义

### 错误处理
- `ParserError` 枚举处理解析错误、词法错误与 schema 校验错误
- 位置跟踪可基于文件/行/列提供精确的错误报告
