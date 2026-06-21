# Tench-Authoring 레포 추출 계획

## Objective

`~/tench/Tench-One` 모노레포에서 story, universe 앱과 그 의존 크레이트 9개를 독립적인 `~/tench-authoring` 워크스페이스로 추출. 완전히 독립적인 Cargo 워크스페이스로 동작해야 하며, 원본 레포와의 동기화 없음.

---

## 사전 조사 결과 요약

### 소스: `/home/yoon/tench/Tench-One`

- 15개 앱, 35개 크레이트, 2개 도구를 포함한 모노레포
- 워크스페이스 루트 `Cargo.toml`에서 52개 멤버 관리
- `[workspace.dependencies]`에 ~40개 외부 의존성 중앙 관리
- `.gitea/workflows/ci.yml`, `AGENTS.md`, `ARCHITECTURE.md` 존재

### 추출 대상

| 카테고리 | 항목 | 소스 경로 |
|----------|------|-----------|
| 앱 | story | `apps/story/` |
| 앱 | universe | `apps/universe/` |
| 크레이트 | tench-ui | `crates/tench-ui/` |
| 크레이트 | ui-automation-core | `crates/ui-automation-core/` |
| 크레이트 | tench-ui-test | `crates/tench-ui-test/` |
| 크레이트 | storage-core | `crates/storage-core/` |
| 크레이트 | fs-core | `crates/fs-core/` |
| 크레이트 | document-core | `crates/document-core/` |
| 크레이트 | office-io | `crates/office-io/` |
| 크레이트 | story-core | `crates/story-core/` |
| 크레이트 | workspace-core | `crates/workspace-core/` |
| 도구 | architecture-guard | `tools/architecture-guard/` |
| 문서 | plans (8개 하위 디렉토리) | `plans/{spec,design,background,implement}/{story,universe}/` |
| 문서 | template (5개 파일) | `template/` |
| 문서 | AGENTS.md, ARCHITECTURE.md | 루트 |

### 주의사항 (조사에서 발견)

1. **tench-ui 인라인 의존성**: `tench-ui`는 `vello`, `parley`, `accesskit`, `kurbo`, `peniko`, `smallvec`, `log`, `raw-window-handle` 등을 인라인 버전으로 참조. 이 중 `vello`, `parley`, `accesskit`, `kurbo`, `peniko`, `smallvec`, `log`, `raw-window-handle`은 워크스페이스 의존성에 없음 → 추출 시 `[workspace.dependencies]`에 추가 필요
2. **tench-ui-test 직접 경로**: `tench-ui = { path = "../tench-ui" }` 사용 → 추출 후에도 상대경로가 유효하므로 변경 불필요
3. **tench-ui-test 인라인 의존성**: `accesskit = "0.24"`, `kurbo = "0.13"` 인라인 사용 → `[workspace.dependencies]`에 추가 필요
4. **architecture-guard 독립형**: workspace.package 미사용, 의존성 없음 → 그대로 복사 가능
5. **plans/test/{story,universe}/** 존재하지 않음 → 추출 불가, 생략

---

## Implementation Plan

### Phase 1: 디렉토리 구조 생성 및 파일 복사

- [ ] **1.1** 타겟 디렉토리 구조 생성
  ```
  ~/tench-authoring/
  ├── apps/story/
  ├── apps/universe/
  ├── crates/{9개}/
  ├── tools/architecture-guard/
  ├── plans/{8개 하위 디렉토리}/
  ├── template/
  ├── .gitea/workflows/
  ```
  사유: 빈 디렉토리 구조를 먼저 만들고 파일을 채우는 방식이 안전

- [ ] **1.2** 앱 복사: `apps/story/`, `apps/universe/` 전체를 `~/tench-authoring/apps/` 하위로 복사
  사유: Tauri 앱 전체 구조(src-tauri, frontend 등)가 필요

- [ ] **1.3** 크레이트 9개 복사: `crates/tench-ui`, `crates/ui-automation-core`, `crates/tench-ui-test`, `crates/storage-core`, `crates/fs-core`, `crates/document-core`, `crates/office-io`, `crates/story-core`, `crates/workspace-core` → `~/tench-authoring/crates/` 하위로 복사
  사유: 각 크레이트는 독립적으로 복사되어야 함

- [ ] **1.4** 도구 복사: `tools/architecture-guard/` → `~/tench-authoring/tools/architecture-guard/`
  사유: 아키텍처 가드 도구는 이 레포에서도 필요

- [ ] **1.5** plans 복사: 8개 하위 디렉토리 복사
  - `plans/spec/story/` (79파일)
  - `plans/spec/universe/` (35파일)
  - `plans/design/story/` (73파일)
  - `plans/design/universe/` (24파일)
  - `plans/background/story/` (12파일)
  - `plans/background/universe/` (11파일)
  - `plans/implement/story/` (79파일)
  - `plans/implement/universe/` (35파일)
  사유: story/universe 관련 계획 문서만 선택적 복사

- [ ] **1.6** template 복사: `template/` 디렉토리 전체 (5개 템플릿 파일)
  사유: 향후 계획 문서 작성에 필요

- [ ] **1.7** 루트 문서 복사: `AGENTS.md`, `ARCHITECTURE.md` → 이후 Phase 3에서 내용 수정

### Phase 2: 워크스페이스 설정 및 Cargo.toml 조정

- [ ] **2.1** 워크스페이스 루트 `Cargo.toml` 작성
  - `[workspace]` members에 12개 항목 등록 (2 앱 + 9 크레이트 + 1 도구)
  - `resolver = "3"`
  - `[workspace.package]`에 version, edition, license, authors 정의
  사유: 독립 워크스페이스의 핵심 설정 파일

- [ ] **2.2** `[workspace.dependencies]` 정리
  추출된 크레이트/앱에서 실제로 사용하는 외부 의존성만 포함:
  - 기존 워크스페이스에 있는 것: `tauri`, `tauri-plugin-dialog`, `serde`, `serde_json`, `regex`, `image`, `zip`, `rusqlite`, `getrandom`, `aes-gcm`, `sha2`, `pollster`, `tokio`, `winit`, `tray-icon`, `reqwest`, `log`, `smallvec`
  - tench-ui 전용으로 새로 추가 필요: `vello`, `parley`, `accesskit`, `kurbo`, `peniko`, `raw-window-handle`
  - 내부 크레이트 9개의 path 매핑
  사유: 불필요한 의존성 제거로 워크스페이스 깔끔 유지

- [ ] **2.3** 각 앱/크레이트의 `Cargo.toml` path 참조 정리
  - 모든 `path = "../../../crates/..."` → `path = "../../crates/..."` (앱 기준) 또는 `workspace = true`로 통일
  - `tench-ui-test`의 `path = "../tench-ui"`는 유효하므로 유지
  - `architecture-guard`는 workspace.package 미사용 → 수정 불필요
  사유: 새로운 디렉토리 구조에 맞게 경로 보정

- [ ] **2.4** `tench-ui` 인라인 의존성 정리
  - 워크스페이스에 있는 의존성(`tauri`, `tokio`, `serde`, `serde_json`, `pollster`, `winit`, `tray-icon`, `reqwest`, `image`, `log`)은 `.workspace = true`로 전환 검토
  - 단, feature 차이가 있는 경우(`reqwest`의 `rustls-tls`, `tokio`의 feature 축소)는 인라인 유지 가능
  - 워크스페이스에 없는 의존성(`vello`, `parley`, `accesskit`, `kurbo`, `peniko`, `smallvec`, `raw-window-handle`)은 `[workspace.dependencies]`에 추가 후 `.workspace = true`로 전환
  사유: 의존성 버전 관리 일원화

- [ ] **2.5** `tench-ui-test` 인라인 의존성 정리
  - `accesskit`, `kurbo`를 `[workspace.dependencies]`에 추가 후 `.workspace = true`로 전환
  사유: 워크스페이스 일관성 유지

- [ ] **2.6** `cargo generate-lockfile` 실행
  사유: 독립적인 lockfile 생성

### Phase 3: 문서 및 CI 조정

- [ ] **3.1** `AGENTS.md` 수정
  - story/universe 관련 내용만 유지
  - 다른 앱(engine, docs, sheets 등)에 대한 규칙 제거
  - 워크스페이스 구조(9 크레이트, 2 앱)에 맞게 업데이트
  사유: 새 레포 컨텍스트에 맞는 에이전트 규칙 필요

- [ ] **3.2** `ARCHITECTURE.md` 수정
  - 공유 크레이트 소유권 테이블을 9개 크레이트로 축소
  - 앱은 story, universe 2개만 표시
  - 레이어 구조를 Authoring 도메인에 맞게 재작성
  사유: 새 레포의 아키텍처 문서로 역할 수행

- [ ] **3.3** `.gitea/workflows/ci.yml` 작성
  - 원본 CI에서 story/universe 관련 잡만 유지 또는 단순화
  - 최소 구성: `static-quality`, `unit-tests`, `integration-tests` (또는 단일 잡으로 통합)
  - `architecture-guard` 실행 단계 포함
  사유: 독립 레포의 CI 파이프라인

- [ ] **3.4** `architecture-guard` baseline 재생성
  - `line_budget_baseline.txt`를 이 레포의 9개 크레이트 기준으로 재생성
  - `main.rs`에서 크레이트 경로가 하드코딩되어 있는지 확인 후 필요시 수정
  사유: 새 레포의 크레이트 구조에 맞는 아키텍처 규칙

### Phase 4: 검증

- [ ] **4.1** `cargo check --workspace --locked` 통과 확인
  사유: 모든 크레이트/앱이 올바르게 컴파일되는지 검증

- [ ] **4.2** `cargo test --workspace --locked` 통과 확인
  사유: 모든 테스트가 통과하는지 검증

- [ ] **4.3** `cargo run -p tench-architecture-guard` 실행 확인
  사유: 아키텍처 가드가 정상 동작하는지 검증

- [ ] **4.4** 전체 파일 누락 확인
  - plans/ 디렉토리 파일 수 일치 확인 (story: 79+73+12+79=243, universe: 35+24+11+35=105)
  - template/ 5개 파일 존재 확인
  - 각 크레이트/앱의 src/ 디렉토리 무결성 확인
  사유: 복사 누락 방지

---

## Verification Criteria

- [ ] `cargo check --workspace --locked` 성공 (컴파일 에러 0)
- [ ] `cargo test --workspace --locked` 성공 (테스트 실패 0)
- [ ] `cargo run -p tench-architecture-guard` 정상 종료
- [ ] `plans/`에 story 243파일, universe 105파일 존재
- [ ] `template/`에 5개 템플릿 파일 존재
- [ ] `AGENTS.md`, `ARCHITECTURE.md`가 Authoring 컨텍스트에 맞게 수정됨
- [ ] `.gitea/workflows/ci.yml` 존재
- [ ] `Cargo.lock` 생성됨
- [ ] 다른 레포(Tench-One)에 대한 path 참조가 0개

---

## Potential Risks and Mitigations

1. **tench-ui의 복잡한 feature 게이트와 optional 의존성**
   완화: tench-ui의 Cargo.toml은 최대한 원본 구조 유지. feature 차이가 있는 의존성은 인라인 유지

2. **원본에만 있는 공유 타입/매크로에 대한 숨은 의존성**
   완화: `cargo check --workspace`로 미사용 의존성 및 누락 의존성을 컴파일 타임에 검출

3. **architecture-guard가 다른 크레이트도 검사하도록 하드코딩되어 있을 가능성**
   완화: `main.rs` 검사 후 9개 크레이트만 검사하도록 수정

4. **CI 잡이 원본 레포의 다른 앱/크레이트를 참조할 가능성**
   완화: CI YAML을 새 레포 구조에 맞게 새로 작성

5. **plans/ 파일 내부에 다른 앱에 대한 상대 참조가 있을 가능성**
   완화: 문서 내용은 그대로 두되, 링크가 깨지는 것은 허용 (기능적 영향 없음)

---

## Alternative Approaches

1. **git filter-repo 사용**: 특정 디렉토리만 히스토리와 함께 추출. 장점: git 히스토리 보존. 단점: 모노레포 구조에서 경로가 복잡하여 필터링 어려움, 12개 멤버의 경로를 모두 지정해야 함.

2. **단순 복사 후 git init**: 파일을 물리적으로 복사하고 새 git 저장소 초기화. 장점: 단순, 확실. 단점: 히스토리 상실. (추천 방식 - 문서에 명시된 원칙에 부합)

3. **cargo workspace inherit 방식 최소화**: tench-ui의 인라인 의존성을 그대로 두고 workspace.dependencies에 추가하지 않음. 장점: 작업량 감소. 단점: 버전 관리 일관성 저하.
