# Implement: pdf-export-format-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- Click the "PDF (.pdf)" row in the export modal initiates PDF export of the story.
- design: export modal with format rows; PDF row is the second entry.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/story/src-tauri/src/ui/mod.rs::on_pointer_event` (export modal hit-test) | `hit_test_export_format` returns index 1 for PDF row | `grep -n 'hit_test_export_format'` |
| `apps/story/src-tauri/src/ui/mod.rs::paint_export_modal` (export modal rendering) | Paints PDF row at index 1 | `fn paint_export_modal` |
| `apps/story/src-tauri/src/ui/commands.rs::export_formats` | Returns format labels including "PDF (.pdf)" at index 1 | `fn export_formats` |
| `crates/story-core/src/project_io.rs::export_pdf` | Generates PDF bytes from StoryDocument | `fn export_pdf` |
| `apps/story/src-tauri/src/ui/mod.rs::story_automation_nodes` | Emits `story.export.pdf` node | `grep -n 'story.export.pdf'` |

## 필요한 변경 (의도 단위)

### 1. Export modal hit-test and dispatch
- **입력**: PointerEvent::Down while `show_export == true`, point inside the PDF format row rect (index 1 in export_formats)
- **처리**: `hit_test_export_format` returns `Some(1)`. The dispatch sets `saved_at = "exported PDF (.pdf)"`, closes export modal. Future: should call `project_io::export_pdf()` and write bytes to user-chosen path.
- **출력/사이드 이펙트**: Export modal closed, status bar updated, `request_paint()` triggered.
- **순서/우선순위**: Export modal hit-test runs before other pointer handlers (checked first when `show_export` is true).

### 2. Automation node emission
- **입력**: `story_automation_nodes` iterates export format debug_ids when `show_export == true`
- **처리**: Emit node with `debug_id = "story.export.pdf"`, `role = "button"`, `label = "PDF (.pdf)"`, bounds from `export_format_rect(size, 1)`.
- **출력/사이드 이펙트**: Only visible when export modal is open.
- **순서/우선순위**: After `story.export.docx`, before `story.export.epub`.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|-----------|
| `story.export.pdf` | Button | `"PDF (.pdf)"` | `show_export == true` |

## 의존
- 선행 implement: none (export modal infrastructure exists)
- 영향 받는 implement: `plain-text-export-format-button`, `tench-story-bundle-export-format-button` (shared export modal)

## 작업 절차
1. spec/design/background 읽기
2. grep으로 `hit_test_export_format` 및 `export_formats` 위치 확정
3. 의도대로 코드 변경
4. `cargo check --workspace --locked` 통과 확인
