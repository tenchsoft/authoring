# Background: automatic-export-modal-render-behavior

## 한 줄 정의
Export 버튼 클릭 또는 Ctrl+E로 `show_export`가 true가 되면, Export Story 모달이 자동으로 프로젝트 이름, 총 단어 수, 익스포트 포맷 행을 포함하여 렌더링된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| `show_export` true 전환 | Export 버튼 클릭 / Ctrl+E / 커맨드 팔레트 Export | 사용자 액션 |
| 문서 데이터 변경 | 모달 열려 있는 상태에서 편집 | 편집 시마다 |
| 윈도우 리사이즈 | 모달 열려 있음 | 리사이즈 이벤트 |

## Lifecycle & State
```
hidden ──[show_export=true]──→ visible ──[show_export=false]──→ hidden
   │                              │
   │                              └──[window resize]──→ visible (re-centered)
   │                              │
   │                              └──[data change]──→ visible (refreshed counts)
```

- **hidden**: 모달 DOM 미렌더링. `show_export=false`.
- **visible**: 모달 중앙 배치. 프로젝트명, 단어 수, 포맷 행 표시. 데이터 변경 시 자동 갱신.

## Concurrency
- 인스턴스 정책: 단일. 동시에 하나의 Export 모달만.
- 동시성 모델: 동기 직렬. paint 사이클 내에서 state 읽기.
- 재진입성: 이미 visible 상태에서 `show_export=true` 재발동 시 무시 (idempotent).
- 취소: backdrop 클릭, Escape, 포맷 선택 완료 시 `show_export=false` 전환.

## Resource budget
- CPU/메모리 추가 비용 없음. 모달 데이터는 기존 `StoryState`에서 읽기.
- 모바일/데스크톱 동일.

## Data flow
- Read: `StoryState.engine.get_document()` (프로젝트명, 챕터, 단어 수).
- Write: `StoryState.show_export` (boolean toggle).
- Persistence: 없음. 세션 메모리만.
- IPC: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 문서 데이터 없음 | `document.is_empty()` | 빈 상태 모달 렌더링 | 무알림 |
| 윈도우 크기 과소 | width < modal width | 모달 clamp | 무알림 |

## Observability
- Log: N/A (UI 상태만).
- Metric: N/A.

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `story.export.modal` | `Dialog` | visible / hidden | 모달 표시 상태 |
| `story.export.backdrop` | `Region` | visible / hidden | 백드롭 |

## UI 인터페이스
design(`plans/design/story/story-export-modal.md`)이 모달의 시각 정의. 이 background는 `show_export` 상태와 데이터 갱신 책임만.

## Out of scope
- 실제 파일 익스포트 (별도 spec).
- 익스포트 진행률 표시 (별도 spec).
