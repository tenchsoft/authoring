# Design: story-format-roundtrip

## 한 줄 정의
파일 열기/저장 시 포맷 변환은 native 파일 다이얼로그와 상태 바 메시지로만 사용자에게 노출된다. 신규 시각 요소 없음.

## 시각적 레이아웃
신규 시각 요소 없음. 파일 다이얼로그, 상태 바 텍스트는 모두 기존 컴포넌트.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| File open dialog | `NativeDialog` | — |
| Status bar update | `Label` | `story.status_bar` |
| Export modal | `Dialog` | `story.export.modal` |

모두 기존 디자인 사용. 별도 visual properties / states 명세 불필요.

## Out of scope
- 파일 다이얼로그 자체 디자인 (OS native).
- 포맷 변환 엔진 (background에서 정의).
