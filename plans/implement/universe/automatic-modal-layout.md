# Implement: automatic-modal-layout

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 모달은 화면 중앙에 표시되고 반투명 오버레이 뒤에 위치한다.
- background: 모달 열기 시 자동으로 레이아웃 계산.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/editor.rs::render_modal` | 모달 렌더링 | `fn render_modal` |
| `apps/universe/src-tauri/src/ui/chat.rs::render` | 전체 렌더 트리 | `fn render` |

## 필요한 변경
### 1. 모달 중앙 배치
- **입력**: 모달 show 플래그가 true
- **처리**: `render_modal`에서 윈도우 크기를 기준으로 중앙 rect 계산. 오버레이는 전체 화면 반투명 레이어
- **출력**: 모달이 화면 중앙에 배치되고 배경이 반투명하게 표시

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.modal.overlay` | `overlay` | `""` | 모달 열림 |

## 의존
- 선행 implement: `automatic-modal-exclusivity`
