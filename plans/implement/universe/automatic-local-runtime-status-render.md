# Implement: automatic-local-runtime-status-render

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더에 로컬 런타임 상태가 자동으로 표시된다.
- background: 항상 표시, "online: local runtime" 텍스트.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::paint_header` | 상태 텍스트 렌더링 | `"online: local runtime"` |
| `apps/universe/src-tauri/src/ui/mod.rs::universe_automation_nodes` | status 노드 | `universe.local_runtime_status` |

## 필요한 변경
### 1. 상태 표시
- **입력**: 항상
- **처리**: 헤더 우측에 "online: local runtime" 텍스트를 NEUTRAL_400으로 렌더링
- **출력**: 헤더 내 상태 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.local_runtime_status` | `status` | `"local runtime"` | 항상 |

## 의존
- 선행 implement: 없음
