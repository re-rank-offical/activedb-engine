# activedb CLI Testing Guide

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어) · [中文](#中文)

For each of these, make sure you're in the activedb-cli directory.
Then, create a directory called test-(some name).
`cd` into that directory.
Instead of the `activedb` command in the commands below, use `cargo run -- <args passed to activedb>`.

## Local Flows (Non-Cloud Testing)

- `activedb init` with default settings; check activedb.toml created with empty template and ./db/ queries path
- `activedb init --path /custom/path` with custom directory; verify project created in specified location
- `activedb init --template custom_template` with custom template; confirm template applied correctly
- `activedb init --queries-path ./custom-queries/` with custom queries directory; validate queries path set correctly
- `activedb check` to validate all instances; verify all configurations and queries validated
- `activedb check my-instance` to validate specific instance; confirm only specified instance checked
- `activedb compile` to compile queries; verify queries compiled to default output
- `activedb compile --path ./custom-output --output my-instance` with custom settings; check compilation to specified path and instance
- `activedb build my-instance` to build local Docker instance; verify Dockerfile and docker-compose.yml generated; confirm Docker image built successfully
- `activedb push my-instance` to deploy local Docker instance; verify container starts and is accessible on configured port
- `activedb start my-instance` to start existing local Docker instance; verify container starts without rebuild
- `activedb stop my-instance` to stop running local Docker instance; confirm container stops cleanly
- `activedb status` to view all instances; confirm all instances listed with correct status and Docker container states
- `activedb prune` to clean unused resources; verify containers, images cleaned while preserving volumes
- `activedb prune my-instance` to clean specific instance resources; confirm only specified instance cleaned
- `activedb prune --all` to clean all instances; verify all project instances cleaned
- `activedb metrics full` to enable full metrics; verify metrics collection enabled
- `activedb metrics basic` to enable basic metrics; confirm reduced metrics collection
- `activedb metrics off` to disable metrics; verify metrics collection disabled
- `activedb metrics status` to check metrics state; confirm current metrics setting displayed
- `activedb update` to upgrade to latest version; verify CLI updated successfully
- `activedb update --force` to force update; confirm update proceeds even if already latest
- `activedb init` in directory with existing activedb.toml; verify appropriate error message
- `activedb build non-existent-instance` with invalid instance; confirm error for missing instance
- `activedb start my-instance` without building first; verify error about missing docker-compose.yml
- `activedb build my-instance` without Docker installed/running; confirm Docker availability error
- `activedb push my-instance` without Docker daemon running; verify Docker daemon error
- `activedb add` with conflicting instance names; verify duplicate name error

## Cloud/Remote Flows

## Project Initialization

- `activedb init --cloud` with cloud instance; verify cloud instance configured in activedb.toml
- `activedb init --cloud --cloud-region eu-west-1` with custom region; check region set correctly
- `activedb init --ecr` with ECR instance; confirm ECR instance added to config
- `activedb init --fly` with Fly.io instance; verify Fly instance created with default settings
- `activedb init --fly --fly-auth token --fly-volume-size 50 --fly-vm-size performance-2x --fly-public false` with custom Fly settings; check all parameters applied

## Instance Management

- `activedb add my-instance --cloud` to add cloud instance; verify instance added to existing project
- `activedb add my-ecr --ecr` to add ECR instance; confirm ECR instance configured
- `activedb add my-fly --fly --fly-volume-size 30` to add Fly instance with custom volume; check instance created with correct volume size
- `activedb delete my-instance` to remove instance; verify instance completely removed from config and infrastructure

## Build and Deployment

- `activedb build my-instance` to build instance; verify build process completes successfully
- `activedb push my-instance` to deploy instance; confirm instance deployed and running
- `activedb start my-instance` to start existing instance; verify instance starts without rebuild
- `activedb stop my-instance` to stop running instance; confirm instance stops cleanly

## Data Operations

- `activedb sync my-instance` to sync source files from remote; verify local queries updated from instance
- `activedb sync` in a workspace without activedb.toml; ensure standard and enterprise clusters are selectable

## Authentication

- `activedb auth login` to authenticate with activedb cloud; verify login successful and credentials stored
- `activedb auth logout` to sign out; confirm credentials cleared
- `activedb auth create-key my-cluster` to generate API key; verify key created for specified cluster

## Error Scenarios

- `activedb push` without building first; verify appropriate build dependency error
- Commands requiring authentication without login; confirm proper authentication error messages

---

## 한국어

# activedb CLI 테스트 가이드

각 테스트 전에 `activedb-cli` 디렉토리에 있는지 확인하세요.
그런 다음 `test-(적당한 이름)` 형태의 디렉토리를 생성하고 해당 디렉토리로 `cd` 하세요.
아래 명령어의 `activedb` 대신 `cargo run -- <activedb에 전달할 인자>`를 사용하세요.

### 테스트 범위 요약

- **로컬 플로우**: 프로젝트 초기화(`init`), 검증(`check`), 컴파일(`compile`), 빌드(`build`), 배포(`push`), 인스턴스 관리(`start`/`stop`/`status`), 정리(`prune`), 메트릭(`metrics`), 업데이트(`update`)를 테스트합니다.
- **클라우드/원격 플로우**: `--cloud`, `--ecr`, `--fly` 옵션으로 프로젝트 초기화 및 인스턴스 추가/삭제를 테스트합니다.
- **빌드 및 배포**: 원격 인스턴스에 대한 `build`/`push`/`start`/`stop` 흐름을 검증합니다.
- **데이터 작업**: 원격 인스턴스와의 `sync` 동작을 검증합니다.
- **인증**: `auth login`/`logout`/`create-key` 플로우를 테스트합니다.
- **오류 시나리오**: 빌드 없이 push, 인증 없이 명령 실행 등 오류 경로를 검증합니다.

구체적인 테스트 명령어와 예상 결과는 위 영어 섹션을 참고하세요.

---

## 中文

# activedb CLI 测试指南

执行每项测试前,请确保您位于 `activedb-cli` 目录。
然后创建一个名为 `test-(某名称)` 的目录,并 `cd` 进入。
以下命令中的 `activedb` 请用 `cargo run -- <传递给 activedb 的参数>` 替换。

### 测试范围摘要

- **本地流程**:测试项目初始化(`init`)、验证(`check`)、编译(`compile`)、构建(`build`)、部署(`push`)、实例管理(`start`/`stop`/`status`)、清理(`prune`)、指标(`metrics`)、更新(`update`)。
- **云端/远程流程**:使用 `--cloud`、`--ecr`、`--fly` 选项测试项目初始化以及实例的添加/删除。
- **构建与部署**:验证远程实例的 `build`/`push`/`start`/`stop` 流程。
- **数据操作**:验证与远程实例之间的 `sync` 行为。
- **认证**:测试 `auth login`/`logout`/`create-key` 流程。
- **错误场景**:验证未构建直接 push、未登录执行命令等错误路径。

具体的测试命令及预期结果请参考上方英文章节。
