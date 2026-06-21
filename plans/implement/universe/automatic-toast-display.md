# Implement: automatic-toast-display

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 작업 완료/오류 시 화면 하단에 토스트 메시지가 자동으로 표시되고 일정 시간 후 사라진다.
- background: 토스트 큐 관리, 자동 타이머 만료.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/state.rs::UniverseState` | toasts 벡터 | `toasts` |
| `apps/universe/src-tauri/src/ui/mod.rs::event` | 타이머 이벤트 | `ToastExpired` |
| `apps/universe/src-tauri/src/ui/chat.rs::render` | 토스트 렌더 | `toast` |

## 필요한 변경
### 1. 토스트 표시 및 자동 소멸
- **입력**: 상태 변경(저장 완료, 오류 등)으로 push_toast 호출
- **처리**: toasts 벡터에 메시지 추가. 타이머 설정 후 ToastExpired 이벤트로 벡터에서 제거
- **출력**: 화면 하단에 토스트가 표시되고 3초 후 자동으로 사라짐

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.toast` | `status` | 토스트 메시지 | toasts 벡터 비어있지 않음 |

## 의존
- 선행 implement: 없음
