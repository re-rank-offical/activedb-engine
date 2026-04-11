# activedb CLI

> Language / 언어 / 语言: **English** (primary) · [한국어](#한국어) · [中文](#中文)

Command-line interface for managing activedb projects and deployments.

## Commands

- `init`: initialize a new project with `activedb.toml`.
- `add`: add an instance to an existing project.
- `check`: validate config and queries.
- `compile`: compile queries into the workspace.
- `build`: build an instance (local or remote prep).
- `push`: deploy/start an instance.
- `sync`: sync source/config from activedb Cloud (standard or enterprise).
- `start` / `stop` / `status`: manage running instances.
- `logs`: view or stream logs.
- `auth`: login/logout/create-key.
- `prune`: clean containers/images/workspaces.
- `delete`: remove an instance.
- `metrics`: manage telemetry level.
- `dashboard`: manage the activedb Dashboard.
- `update`: update the CLI.
- `migrate`: migrate v1 projects to v2.
- `backup`: back up an instance.
- `feedback`: send feedback to the activedb team.

Run `activedb <command> --help` for command-specific flags and options.

## Error handling

- Recoverable/library errors use `thiserror::Error` (config, project, port).
- CLI commands return `eyre::Result` and render `CliError` for consistent output.

---

## 한국어

# activedb CLI

activedb 프로젝트와 배포를 관리하는 명령줄 인터페이스입니다.

### 명령어

- `init`: `activedb.toml`로 새 프로젝트를 초기화합니다.
- `add`: 기존 프로젝트에 인스턴스를 추가합니다.
- `check`: 설정과 쿼리를 검증합니다.
- `compile`: 쿼리를 워크스페이스로 컴파일합니다.
- `build`: 인스턴스를 빌드합니다 (로컬 또는 원격 준비).
- `push`: 인스턴스를 배포하거나 시작합니다.
- `sync`: activedb Cloud(standard 또는 enterprise)에서 소스/설정을 동기화합니다.
- `start` / `stop` / `status`: 실행 중인 인스턴스를 관리합니다.
- `logs`: 로그를 조회하거나 스트리밍합니다.
- `auth`: login / logout / create-key.
- `prune`: 컨테이너 / 이미지 / 워크스페이스를 정리합니다.
- `delete`: 인스턴스를 제거합니다.
- `metrics`: 텔레메트리 레벨을 관리합니다.
- `dashboard`: activedb 대시보드를 관리합니다.
- `update`: CLI를 업데이트합니다.
- `migrate`: v1 프로젝트를 v2로 마이그레이션합니다.
- `backup`: 인스턴스를 백업합니다.
- `feedback`: activedb 팀에 피드백을 전송합니다.

명령어별 플래그와 옵션은 `activedb <command> --help`로 확인하세요.

### 오류 처리

- 복구 가능 오류 / 라이브러리 오류는 `thiserror::Error`를 사용합니다 (config, project, port).
- CLI 명령은 `eyre::Result`를 반환하며 일관된 출력을 위해 `CliError`를 렌더링합니다.

---

## 中文

# activedb CLI

用于管理 activedb 项目和部署的命令行界面。

### 命令

- `init`:使用 `activedb.toml` 初始化新项目。
- `add`:向现有项目添加实例。
- `compile`:将查询编译到工作区。
- `check`:验证配置与查询。
- `build`:构建实例(本地或远程准备)。
- `push`:部署/启动实例。
- `sync`:从 activedb Cloud(标准版或企业版)同步源代码/配置。
- `start` / `stop` / `status`:管理运行中的实例。
- `logs`:查看或流式查看日志。
- `auth`:登录/登出/创建密钥。
- `prune`:清理容器/镜像/工作区。
- `delete`:移除实例。
- `metrics`:管理遥测级别。
- `dashboard`:管理 activedb 仪表板。
- `update`:更新 CLI。
- `migrate`:将 v1 项目迁移至 v2。
- `backup`:备份实例。
- `feedback`:向 activedb 团队发送反馈。

运行 `activedb <command> --help` 可查看特定命令的标志和选项。

### 错误处理

- 可恢复错误/库错误使用 `thiserror::Error`(config、project、port)。
- CLI 命令返回 `eyre::Result`,并渲染 `CliError` 以保持输出一致。
