# Implement: novel-mode-tab

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 소설 모드 탭을 클릭하면 소설 모드로 전환된다.
- design: 헤더 내 두 번째 탭, 활성 시 ACCENT_UNIVERSE 텍스트.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/universe/src-tauri/src/ui/state.rs::UniverseMode::Novel` | 모드 변형 | `Novel` |
| `apps/universe/src-tauri/src/ui/chat.rs::paint_novel` | 소설 모드 렌더링 | `fn paint_novel` |

## 필요한 변경
### 1. 모드 전환
- **입력**: `UniverseHit::Mode(UniverseMode::Novel)` 클릭
- **처리**: `state.set_mode(UniverseMode::Novel)`
- **출력**: mode 변경, repaint

### 2. 소설 모드 렌더링
- **입력**: messages 리스트
- **처리**: 챕터 제목 + 메시지 텍스트를 소설 형식으로 표시
- **출력**: center 패널에 소설 텍스트 렌더링

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `universe.mode.novel` | `tab` | `"Novel"` | 항상 |

## 의존
- 선행 implement: 없음
- 영향 받는 implement: `automatic-active-mode-content-render`
