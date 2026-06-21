# Implement: automatic-export-modal-render-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- `show_export == true`일 때 export modal이 프로젝트명, 단어 수, 포맷 행 목록을 표시한다.
- backdrop 클릭으로 닫힌다.
- 포맷 행 클릭 시 export 액션 수행 후 modal이 닫힌다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::paint` | overlay 섹션에서 `show_export` 분기 | `grep -n 'show_export' mod.rs` |
| `apps/story/src-tauri/src/ui/mod.rs::paint_export_modal` | 모달 프레임 + 프로젝트 정보 + 포맷 행 렌더 | `fn paint_export_modal` |
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` | 포맷 행 클릭, backdrop 클릭 처리 | `grep -n 'hit_test_export_format\|export_modal_rect' mod.rs` |
| `apps/story/src-tauri/src/ui/commands.rs::export_formats` | 7개 포맷 라벨 | `fn export_formats` |
| `apps/story/src-tauri/src/ui/export.rs::word_count_label` | 단어 수 포맷팅 | `fn word_count_label` |
| `apps/story/src-tauri/src/ui/state.rs::open_export` | `show_export = true`, 다른 오버레이 닫기 | `fn open_export` |

## 필요한 변경 (의도 단위)

### 1. Export 모달 렌더
- **입력**: `show_export == true`
- **처리**: `paint_export_modal`에서 모달 rect(`export_modal_rect`)에 `theme.surface` 배경, `theme.border` 스트로크, `theme.border_radius` 라운딩. "Export Story" 타이틀 후 프로젝트명 + 단어 수 서브타이틀. `export_formats()` 순회하며 각 포맷 행을 36.0 간격으로 렌더.
- **출력/사이드 이펙트**: 시각적 오버레이.
- **순서/우선순위**: overlay 섹션 첫 번째.

### 2. Backdrop 클릭으로 닫기
- **입력**: pointer down, `show_export == true`, 클릭 위치가 `export_modal_rect` 밖
- **처리**: `close_overlays()` 호출.
- **출력/사이드 이펙트**: 모달 닫힘, repaint.
- **순서/우선순위**: 오버레이 hit-test 중 첫 번째.

### 3. 포맷 행 클릭 → export
- **입력**: pointer down, `show_export == true`, `hit_test_export_format`이 인덱스 반환
- **처리**: `saved_at = format!("exported {}", export_formats()[index])`, `show_export = false`.
- **출력/사이드 이펙트**: 모달 닫힘, saved_at 갱신, repaint.
- **순서/우선순위**: backdrop 닫기 체크 전.

### 4. 자동화 노드
- **입력**: `show_export == true`
- **처리**: `story.export.backdrop`(Button), `story.export.modal`(Dialog), 7개 포맷 행(Button) 노드 emit.
- **출력/사이드 이펙트**: automation tree에 노드 추가.
- **순서/우선순위**: 다른 오버레이 노드 전.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `story.export.backdrop` | Button | `"Export backdrop"` | `show_export == true` |
| `story.export.modal` | Dialog | `"Export modal"` | `show_export == true` |
| `story.export.docx` | Button | `"DOCX (.docx)"` | `show_export == true` |
| `story.export.pdf` | Button | `"PDF (.pdf)"` | `show_export == true` |
| `story.export.epub` | Button | `"EPUB (.epub)"` | `show_export == true` |
| `story.export.markdown` | Button | `"Markdown (.md)"` | `show_export == true` |
| `story.export.html` | Button | `"HTML (.html)"` | `show_export == true` |
| `story.export.plain_text` | Button | `"Plain Text (.txt)"` | `show_export == true` |
| `story.export.bundle` | Button | `"Tench Story Bundle (.tench-story)"` | `show_export == true` |

## 의존
- 선행 implement: 없음.
- 영향 받는 implement: `automatic-overlay-exclusivity-behavior` (동일 오버레이 메커니즘).

## 작업 절차
1. spec/design/background 읽기
2. grep으로 위치 확정 (`grep -n 'show_export\|paint_export_modal\|hit_test_export_format' mod.rs`)
3. 의도대로 코드 변경 (현재 구현이 spec과 일치하는지 확인)
4. cargo check 통과 확인
