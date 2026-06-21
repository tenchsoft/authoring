# Implement: automatic-prompt-preview-width

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 프롬프트 미리보기는 컴포저 너비에 맞게 자동 조정된다.
- background: 컴포저 리사이즈 시 프롬프트 미리보기 너비 자동 업데이트.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/chat.rs::render` | 프롬프트 미리보기 렌더 | `preview` |
| `apps/universe/src-tauri/src/ui/state.rs::UniverseState` | 상태 | `composer_width` |

## 필요한 변경
### 1. 너비 자동 조정
- **입력**: 윈도우 리사이즈 또는 패널 전환
- **처리**: render 시 컴포저 영역 너비를 읽어 프롬프트 미리보기 텍스트 래핑 너비로 사용
- **출력**: 프롬프트 미리보기가 항상 컴포저 너비에 맞게 표시

## 의존
- 선행 implement: 없음
