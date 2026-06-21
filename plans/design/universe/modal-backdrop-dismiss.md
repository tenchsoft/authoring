# Design: modal-backdrop-dismiss

## 한 줄 정의
모달 외부 배경 클릭으로 모달을 닫는 동작. UI 요소는 배경 오버레이.

## 시각적 레이아웃
```
┌─ Backdrop (full screen, semi-transparent) ─────┐
│                                                 │
│           ┌─ Modal ──────────┐                  │
│           │                  │                  │
│           └──────────────────┘                  │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Visual properties
| 속성 | 값 |
|------|----|
| 배경색 | 흑색 40% opacity |

## Out of scope
- 모달 내용 (별도 design).
