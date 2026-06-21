# Tench-Authoring 레포 추출 계획

## 원칙

- 완전히 독립적인 Cargo 워크스페이스
- 필요한 공유 크레이트는 물리적으로 복사
- 다른 레포와의 동기화 없음. 각 크레이트는 독립적으로 진화

---

## 앱

| 앱 | 패키지명 | 비고 |
|----|---------|------|
| story | `tench-story` | 도서 저작 도구 |
| universe | `tench-universe` | 캐릭터 기반 대화 도구 |

---

## 포함할 크레이트

| 크레이트 | 패키지명 | 직접 소비 앱 | 내부 의존성 |
|----------|---------|-------------|------------|
| tench-ui | `tench-ui` | story, universe | `ui-automation-core` |
| ui-automation-core | `tench-ui-automation-core` | 전체 (dev) | 없음 |
| tench-ui-test | `tench-ui-test` | 전체 (dev) | `tench-ui`, `ui-automation-core` |
| storage-core | `tench-storage-core` | story, universe | 없음 |
| fs-core | `tench-fs-core` | (workspace-core 경유) | 없음 |
| document-core | `tench-document-core` | story, (story-core 경유) | 없음 |
| office-io | `tench-office-io` | story, (story-core 경유) | `document-core`, `fs-core`, `storage-core` |
| story-core | `tench-story-core` | story | `document-core`, `office-io` |
| workspace-core | `tench-workspace-core` | story | `fs-core`, `storage-core` |

---

## 크레이트 의존성 그래프

```
tench-ui ──────── ui-automation-core
tench-ui-test ─── tench-ui, ui-automation-core

story-core ────── document-core, office-io
office-io ─────── document-core, fs-core, storage-core
workspace-core ── fs-core, storage-core
```

---

## 앱별 상세 의존성

### story (`apps/story/src-tauri`)

```
tench-document-core
tench-office-io
tench-storage-core
tench-story-core
tench-workspace-core
tench-ui (features = ["tauri"])
tench-ui-test (dev)
tench-ui-automation-core (dev)
```

### universe (`apps/universe/src-tauri`)

```
tench-storage-core
tench-ui (features = ["tauri"])
tench-ui-test (dev)
tench-ui-automation-core (dev)
```

---

## 디렉토리 구조

```
Tench-Authoring/
├── Cargo.toml              (워크스페이스 루트)
├── Cargo.lock
├── .gitea/
│   └── workflows/ci.yml
├── AGENTS.md
├── ARCHITECTURE.md
├── apps/
│   ├── story/
│   │   └── src-tauri/
│   └── universe/
│       └── src-tauri/
├── crates/
│   ├── tench-ui/
│   ├── ui-automation-core/
│   ├── tench-ui-test/
│   ├── storage-core/
│   ├── fs-core/
│   ├── document-core/
│   ├── office-io/
│   ├── story-core/
│   └── workspace-core/
├── plans/
│   ├── spec/story/
│   ├── spec/universe/
│   ├── design/story/
│   ├── design/universe/
│   ├── background/story/
│   ├── background/universe/
│   ├── implement/story/
│   ├── implement/universe/
│   ├── test/story/
│   └── test/universe/
├── template/
└── tools/
    └── architecture-guard/
```

---

## 워크스페이스 설정

```toml
[workspace]
members = [
  "apps/story/src-tauri",
  "apps/universe/src-tauri",
  "crates/tench-ui",
  "crates/ui-automation-core",
  "crates/tench-ui-test",
  "crates/storage-core",
  "crates/fs-core",
  "crates/document-core",
  "crates/office-io",
  "crates/story-core",
  "crates/workspace-core",
  "tools/architecture-guard",
]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "UNLICENSED"
authors = ["Tench"]
```

---

## 이관 체크리스트

1. Gitea에 `Tench-Authoring` 빈 레포 생성
2. `apps/story`, `apps/universe` 복사
3. 9개 크레이트를 `crates/` 하위에 복사
4. `tools/architecture-guard` 복사, baseline을 이 레포 크레이트 9개로 재생성
5. 워크스페이스 루트 `Cargo.toml` 작성 (위 설정 기준)
6. `[workspace.dependencies]` 정리 — 이 레포에서 사용하는 외부 의존성만 남기기
7. 각 앱/크레이트의 `path` 참조 정리 — `path = "../../../crates/..."` → `path = "crates/..."` 로 통일
8. `cargo generate-lockfile` 실행
9. `.gitea/workflows/ci.yml` 작성
10. `AGENTS.md`, `ARCHITECTURE.md` 작성
11. `plans/` 하위에서 story/universe 관련 문서만 복사
12. `template/` 복사
13. `cargo check --workspace --locked` 통과 확인
14. `cargo test --workspace --locked` 통과 확인
15. Gitea CI 파이프라인 녹색 확인
