# Implement: backspace-composer

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Backspace 키로 마지막 글자를 지운다.
- design: 키보드 단축키.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/mod.rs::on_text_event` | Backspace 키 처리 | `NamedKey::Backspace` |
| `apps/universe/src-tauri/src/ui/state.rs::backspace_input` | 입력 삭제 | `fn backspace_input` |

## 필요한 변경
### 1. Backspace 키 처리
- **입력**: `LogicalKey::Named(NamedKey::Backspace)` + `is_pressed`
- **처리**: `state.backspace_input()` → 현재 포커스(input_text 또는 search_query)의 마지막 문자 제거
- **출력**: 텍스트 갱신, repaint

## 새 자동화 노드
없음.

## 의존
- 선행 implement: `composer-text-input`
