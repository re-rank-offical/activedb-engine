# Analyzer Module

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어) · [中文](#中文)

## Overview
The analyzer module performs static analysis and type checking on the ActiveQL AST, ensuring queries are grammatically and semantically correct before code generation.

## Structure

### Core Components
- **`mod.rs`** - Main analyzer entry point, orchestrates validation passes
- **`types.rs`** - Type system definitions and type inference structures
- **`diagnostic.rs`** - Diagnostic messages and error reporting
- **`error_codes.rs`** - Error code definitions and messages
- **`errors.rs`** - Error handling utilities
- **`fix.rs`** - Auto-fix suggestions for common errors
- **`pretty.rs`** - Pretty printing utilities for diagnostics
- **`utils.rs`** - Helper functions for analysis

### Validation Methods (in `methods/`)
- **`schema_methods.rs`** - Schema validation and field lookup building
- **`query_validation.rs`** - Query structure and parameter validation
- **`migration_validation.rs`** - Schema migration consistency checks
- **`statement_validation.rs`** - Statement-level validation
- **`traversal_validation.rs`** - Graph traversal operation validation
- **`graph_step_validation.rs`** - Individual graph step validation
- **`object_validation.rs`** - Object literal and remapping validation
- **`infer_expr_type.rs`** - Expression type inference
- **`exclude_validation.rs`** - Field exclusion validation

## Analysis Flow

1. **Input**: Parsed AST from the parser module
2. **Schema Validation**: Verifies schema definitions are valid
3. **Migration Validation**: Ensures migrations are consistent across versions
4. **Query Validation**: Type-checks queries against schemas
5. **Output**: Diagnostics (errors/warnings) and validated AST for code generation


## Error Handling
- Error codes provide consistent, searchable error identification
- Diagnostics include source location for precise error reporting
- Fix suggestions help users resolve common issues

---

## 한국어

# 분석기 모듈

### 개요
분석기 모듈은 ActiveQL AST에 대한 정적 분석과 타입 검사를 수행하여, 코드 생성 전에 쿼리가 문법적, 의미적으로 올바른지 확인합니다.

### 구조

#### 핵심 컴포넌트
- `mod.rs` — 분석기 진입점. 검증 패스를 조율합니다
- `types.rs` — 타입 시스템 정의와 타입 추론 구조
- `diagnostic.rs` — 진단 메시지와 오류 리포트
- `error_codes.rs` — 오류 코드 정의와 메시지
- `errors.rs` — 오류 처리 유틸리티
- `fix.rs` — 일반적인 오류에 대한 자동 수정 제안
- `pretty.rs` — 진단용 pretty print 유틸리티
- `utils.rs` — 분석 도우미 함수

#### 검증 메서드 (`methods/`)
- `schema_methods.rs` — 스키마 검증 및 필드 조회 테이블 구축
- `query_validation.rs` — 쿼리 구조와 파라미터 검증
- `migration_validation.rs` — 스키마 마이그레이션의 버전 간 일관성 검증
- `statement_validation.rs` — 문장 수준 검증
- `traversal_validation.rs` — 그래프 트래버설 연산 검증
- `graph_step_validation.rs` — 개별 그래프 스텝 검증
- `object_validation.rs` — 오브젝트 리터럴 및 리매핑 검증
- `infer_expr_type.rs` — 표현식 타입 추론
- `exclude_validation.rs` — 필드 제외 검증

### 분석 흐름
1. **입력**: 파서 모듈에서 전달된 파싱된 AST
2. **스키마 검증**: 스키마 정의의 유효성 확인
3. **마이그레이션 검증**: 버전 간 마이그레이션의 일관성 확인
4. **쿼리 검증**: 스키마에 대한 쿼리의 타입 검사
5. **출력**: 진단(오류/경고) 및 코드 생성용으로 검증된 AST

### 오류 처리
- 오류 코드로 일관되고 검색 가능한 오류 식별 제공
- 진단에 소스 위치를 포함하여 정밀한 오류 리포트
- 수정 제안을 통해 사용자가 일반적인 문제를 해결하도록 지원

---

## 中文

# 分析器模块

### 概述
分析器模块对 ActiveQL AST 进行静态分析与类型检查,确保在代码生成前查询在语法和语义上均正确。

### 结构

#### 核心组件
- `mod.rs` — 分析器入口,协调各验证阶段
- `types.rs` — 类型系统定义与类型推断结构
- `diagnostic.rs` — 诊断消息与错误报告
- `error_codes.rs` — 错误代码定义与消息
- `errors.rs` — 错误处理工具
- `fix.rs` — 针对常见错误的自动修复建议
- `pretty.rs` — 用于诊断的美观打印工具
- `utils.rs` — 分析辅助函数

#### 验证方法(`methods/`)
- `schema_methods.rs` — schema 验证与字段查找表构建
- `query_validation.rs` — 查询结构与参数验证
- `migration_validation.rs` — schema 迁移的跨版本一致性检查
- `statement_validation.rs` — 语句级验证
- `traversal_validation.rs` — 图遍历操作验证
- `graph_step_validation.rs` — 单个图步骤验证
- `object_validation.rs` — 对象字面量与重映射验证
- `infer_expr_type.rs` — 表达式类型推断
- `exclude_validation.rs` — 字段排除验证

### 分析流程
1. **输入**:来自解析器模块的已解析 AST
2. **Schema 验证**:确认 schema 定义有效
3. **迁移验证**:确保迁移跨版本一致
4. **查询验证**:对照 schema 对查询进行类型检查
5. **输出**:诊断(错误/警告)以及供代码生成使用的已验证 AST

### 错误处理
- 错误代码提供一致且可搜索的错误识别
- 诊断附带源码位置,便于精准报告错误
- 修复建议帮助用户解决常见问题
