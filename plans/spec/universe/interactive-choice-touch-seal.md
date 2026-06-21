# Spec: interactive-choice-touch-seal

## 한 줄 정의
사용자가 인터랙티브 모드에서 "Touch the seal" 선택지를 클릭하면 해당 선택지가 선택 상태가 되고 "You chose: Touch the seal" 토스트가 나타난다.

## 진입점
- 버튼: 인터랙티브 모드 선택지 목록의 "Touch the seal" 클릭

## 사용자 흐름
1. 사용자가 인터랙티브 모드에서 "Touch the seal" 선택지를 클릭한다.
2. 해당 선택지에 선택 스타일이 적용된다.
3. 이전에 선택된 다른 선택지의 스타일이 제거된다.
4. "You chose: Touch the seal" 토스트가 나타난다.

## 성공 조건
- [ ] "Touch the seal" 클릭 시 해당 선택지에만 선택 스타일이 적용된다.
- [ ] "You chose: Touch the seal" 토스트가 나타난다.

## 실패 / 취소 흐름
- 인터랙티브 모드가 아닐 때 클릭 → 숨겨진 선택지가 입력을 받지 않는다.

## 경계 / 예외
- 다른 선택지 선택 후 "Touch the seal" 클릭 → 선택이 이동한다.
- 스토리 블록에 선택지가 더 적을 때 → 존재하지 않는 선택지가 활성화되지 않는다.

## 범위 외
- 다른 인터랙티브 선택지 동작.
- 스토리 상태 진행 / Engine 나레이션 생성.
