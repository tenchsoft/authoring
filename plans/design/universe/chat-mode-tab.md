# Design: chat-mode-tab

## 한 줄 정의
채팅 모드 탭은 상단 탭 바의 항목으로, 활성 시 채팅 UI를 표시한다. 기존 탭 컴포넌트 재사용.

## 시각적 레이아웃
신규 시각 요소 없음. 기존 탭 바의 항목.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Chat mode tab | `Tab` | `universe.tab.chat` |

기존 탭 디자인 사용. 별도 visual properties 불필요.

## Out of scope
- 탭 바 전체 디자인 (별도 design `tab-bar`).
