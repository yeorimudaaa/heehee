# 🌼 희희 (Heehee) — Daily Log

> 맥북 코너에 사는 픽사 톤 쿼카 룸메이트. 본인 + 친구 5명 선물용.
> 마감: 2주 (~2026-05-12) · .dmg AirDrop 전달

---

## 📊 전체 진척

```
일정    [██████████████████████████░░░░] 13/14 days  (93%)  ← Phase 2 코드 통합 거의 완료
자산    [████████████████████████████░░] 28/34 png   (82%)  ← Phase 1 자산 11장 추가
코드    [██████████████████████████░░░░] 7/8 모듈    (88%)  ← Phase 2 인터랙션 통합, Phase 3 남음
```

| Phase | 상태 |
|-------|------|
| Phase 1 (캐릭터 + 존재감, Day 1~7) | 🟡 진행 중 |
| Phase 2 (룸메이트 영혼 + 배포, Day 8~14) | ⬜ 대기 |

### Day별 진척 시각화

```
Day:  1  2  3  4  5  6  7  8  9 10 11 12 13 14
      ✅ ✅ ✅ ✅ ✅ ✅ ✅ ✅ ✅ ✅ ✅ ✅ ⏳ ⬜
                                          ↑ 지금 여기 (dogfood 직전)
```

---

## 🔑 핵심 의사결정 타임라인

| 날짜 | 결정 | 영향 |
|------|------|------|
| 2026-04-29 | 데이지 = **하이브리드 분리 합성** | S 시리즈는 데이지 통합, 풀↔데이지만 코드 swap |
| 2026-04-29 | 이미지 도구 = **나노바나나** (미드저니 X) | 빠른 톤 일관성 |
| **2026-04-29 야간** | **🌼 2D pivot 확정** (3D Meshy/Tripo 무료 export 막힘) | Three.js / Mixamo 트랙 폐기, HTML/CSS+PNG 트랙 |
| 2026-04-29 야간 | Wave 작업 게이트 도입 (검증 후 다음 Wave) | 톤 일관성 보호 |
| **2026-04-30 새벽** | **CSS 진폭 호흡 폐기 → PNG 키프레임 교체** (사용자 인사이트: "픽사 2D 단편처럼 다중 PNG가 정답") | 자산 +5장 (S1b/S7b/S8b/S2-half/S1c) |
| **2026-04-30 새벽** | **걷기 시퀀스 MVP 승격** (Phase 2 WALKING_EDGE 별도) | 자산 +4장 (W1~W4), Day 7 걷기 시스템 추가 |
| 2026-04-30 새벽 | 창 크기 240→160, JS 드래그 추가, capabilities 권한 확장 | 작은 룸메이트 + 자유 이동 |
| **2026-04-30 저녁** | **Tauri 회색창 fix** — lib.rs에 cocoa 직접 호출 (NSWindow setOpaque false + clearColor) | macOS Tauri 2 알려진 이슈 우회, 진짜 투명 창 |
| **2026-04-30 저녁** | **호흡 옵션 A 채택** — PNG cross-fade 폐기, CSS 미세 transform sine wave (배만 1.2% 부풂, transform-origin 80%) | "갑자기 커지는 느낌" 어색함 해결, 호흡 들숨 PNG 보관 (옵션 C 갈 때 사용) |
| **2026-04-30 저녁** | 부드러운 깜빡 시퀀스 (S1 → S2-half → S2 → S2-half → S1, 4단계) | 자연스러운 눈꺼풀 동작 |
| **2026-04-30 저녁** | 잔잔한 눈 웃음 (S1c) 25~40초마다 4초 등장 | "지금 행복해" 표정 변화 |
| **2026-04-30 밤** | **걷기 4→2프레임 단순화** — W1, W2만으로 사이클. W3는 모델 강제 어려움 (측면 포즈에서 항상 카메라쪽 발 앞) | 다마고치/마리오 정석 워크플로우, 자산 +2장 절약 |
| **2026-05-03** | **걷기 4→4프레임 복귀** — Phase 5 톤 통일에서 자산 자체를 손보기로 (CSS 필터는 노출 빠진 느낌이라 폐기) | 걷기 자산 그대로 사용, 톤 보정은 자산 단계에서 |
| **2026-05-03** | **자발적 어슬렁 제거** — 사용자: "마우스 갖다댈때 휘리릭만 있고 자동으로 어슬렁거리는건 없어" | 룸메이트가 산만하지 않게, SHY_AWAY (1초 휘리릭, 120px, 3초 쿨다운)만 남김 |
| **2026-05-03** | **창 240×240 → 240×460 영구 확장** | 모달 영역을 영구적으로 캐릭터 머리 위에 확보. 동적 리사이즈 폐기 (희희가 갑자기 움직이는 현상 제거) |
| **2026-05-03** | **HTML 구조 분리** — `.modal-area` (위 220px) + `.character-area` (아래 240px) | 모달과 캐릭터 영역 명확히 분리, SHY_AWAY는 character-area에서만 발동 |
| **2026-05-03** | **`pmset` 배터리 감지 버그 수정** — `discharging`이 `charging` 부분 매칭으로 false-positive | `s.contains("AC Power")` 단독 체크로 변경 |
| **2026-05-03** | **잠시 쉬게 하기 플로우 D안 채택** — YAWN(1초) → hide → 1시간 → show + STRETCHING(1.5초) | 룸메이트가 자러 갔다가 일어나는 자연스러운 게임적 디테일 |
| **2026-05-03** | **메뉴 라벨에 (1시간 뒤 복귀) 명시** | 친구 첫 사용 시 잠시 쉬게 하기의 시간 약속이 명확 |

---

## 📅 Day별 상세 진척

### Day 1 — 2026-04-29 (낮) ✅
**REF 4장 캐릭터 외모 확정**

산출물:
- [ref-front.png](ref/images/ref-front.png) — 정면 (스타일 기준)
- [ref-side.png](ref/images/ref-side.png) — 측면
- [ref-threequarter.png](ref/images/ref-threequarter.png) — 3/4
- [ref-back.png](ref/images/ref-back.png) — 뒷모습 (둥근 흰 꼬리 시그니처)

배운 것: REF-1 처음엔 3/4로 나옴 → "perfectly symmetrical front view, no three-quarter angle" 강제 키워드 추가 후 정면 확보.

---

### Day 2 — 2026-04-29 (야간) ✅
**3D 분기점 검증 → 2D pivot 결정**

시도:
1. Meshy 6 무료 → GLB 다운로드 유료 ❌
2. Tripo3D 무료 → export 유료 ❌
3. 결론: 무료 3D 트랙은 검증 불가

결정: **2D pivot** (HTML/CSS + 정적 PNG)
- 데스크탑 펫의 정석 (다마고치, Discord 이모지 등)
- 200×200에선 3D 디테일 불필요
- 엔지니어링 부담 ↓, 디자인 자산 작업 ↑

산출물: 모든 문서 (plan.md / schedule.md / prompts.md / SKILL.md ×2)에 2D 트랙 정합성 반영.

---

### Day 3 — 2026-04-29 ~ 04-30 (야간) ✅
**Wave 1 자산 6장 + P3 + HTML 프로토타입 검증**

산출물:
- [s1-awake-default.png](ref/images/s1-awake-default.png) — 기본 idle (모든 idle 베이스)
- [s2-awake-blink.png](ref/images/s2-awake-blink.png) — 깜빡 (실제론 happy-blink 톤, 그대로 채택)
- [s3-awake-eartwitch.png](ref/images/s3-awake-eartwitch.png) — 귀 쫑긋 🟡 미세 재시도 권장
- [s7-charging-idle.png](ref/images/s7-charging-idle.png) — 충전 중 멍때리기
- [s8-napping.png](ref/images/s8-napping.png) — 낮잠 (데이지 꼭 쥔 채)
- [p3-prop-daisy.png](ref/images/p3-prop-daisy.png) — 데이지 단독 (분리 합성용)

추가:
- ✅ macOS "배경 제거" 자동 처리로 모든 PNG 투명화
- ✅ 파일명 오타 정정 (점 두 개 → 한 개)
- ✅ [prototype/index.html](prototype/index.html) 작성 — S1↔S2 깜빡임 + 호흡 CSS + 다양한 배경 토글
- ✅ 브라우저 검증 통과 ("졸라 귀엽닼ㅋㅋㅋ")
- ✅ Wave 2 게이트 통과

---

### Day 7~13 — 2026-05-01 ~ 2026-05-03 — Phase 1+2 통합 ✅

**Phase 1 자산 11장 완료** (체인 톤 매칭, 한 세션에 다 뽑기)

| # | 자산 | 의도 |
|---|------|------|
| 1 | S11 first-greeting + S11a show | 첫 실행 1회 인사 (친구 첫 인상) |
| 2 | S5 idle-hungry | 풀 먹고 싶다 |
| 3 | S6 eating | 풀 먹는 중 |
| 4 | S6b awake-with-grass | 풀 들고 다님 (데이지는 귀에) |
| 5 | P2 prop-leaf | 나뭇잎 단독 (P1 풀은 폐기, 쿼카는 잎 먹음) |
| 6 | S10 drowsy | 졸림 (밤 11시) |
| 7 | S13 yawning | 하품 |
| 8 | S12 surprised | 깜짝 (이름 변경 시) |
| 9 | S9 stretching | 기지개 (충전 해제) |
| 10 | S14 name-tilt | 갸우뚱 (이름 검증 실패) |

**Phase 2 코드 통합 — 7개 인터랙션 모두 통합됨**

- ✅ FIRST_GREETING — localStorage `heehee-first-launch-done` 1회성, S11→S11a→AWAKE
- ✅ 풀 먹이기 — 트레이 "🌿 풀 주기" → S5→S6→S6b, held_item swap (데이지 ↔ 풀), 자정 자동 데이지 복귀
- ✅ DROWSY — 시간대 23:00~06:00 자동 (1분마다 체크)
- ✅ YAWNING — 자발적 (AWAKE 45분/회, DROWSY 15분/회)
- ✅ SURPRISED — 이름 변경 시 1.5초
- ✅ STRETCHING — 충전 해제 시 1.5초
- ✅ TILT — 이름 검증 실패 시 (1자 미만/12자 초과)

**UX 폴리싱 (2026-05-03)**

- 자발적 어슬렁 제거 → SHY_AWAY 휘리릭 (1초/120px/3초 쿨다운)만 남김
- 걷기 프레임별 다리 속도 조정: 속도 100px/s 넘으면 110ms, 아니면 280ms
- 트레이 메뉴: 이름 + 상태 동적 표시 (`👫 같이 있는 중`, `🌼 데이지 자랑 중` 등)
- 트레이 모달 톤: 핑크 → `#a8d89c` 연두 (희희 색감 일관)
- 회색창 fix (cocoa setWantsLayer + layer.backgroundColor = clearColor) — 클릭 시 회색 깜빡 제거
- 잠시 쉬게 하기 D안: YAWN 1초 → hide → 1시간 → show + STRETCH 1.5초

**아키텍처 결정 (오늘)**

- 창 영구 240×460: 동적 리사이즈 폐기, `.modal-area`(상단 220) + `.character-area`(하단 240) 분리
  - 이전 설계: 모달 열면 창이 위로 늘어났다 줄어듦 → 캐릭터가 갑자기 움직이는 시각 결함
  - 현재 설계: 모달 영역 항상 위에 확보, 희희는 절대 안 움직임
- `pmset` substring 버그 fix: "discharging".contains("charging") → true 였음. `AC Power` 단독 체크.

**친구 전달 패키지 (2026-05-02)**

- ✅ Heehee.zip (.app 압축, 65MB) — Desktop에 생성
- ✅ ~/Desktop/희희_사용법.md — 친구 가이드 (설치/사용/프라이버시)
- ⏳ .dmg 정식 빌드 재시도 (Phase 4)

---

### Day 5~6 — 2026-04-30 (저녁/밤, 부분 진행) ⏳
**Wave 2A/2B/2C 자산 + idle 시스템 통합**

오늘 한 일:
- ✅ Tauri 회색창 fix (cocoa 직접 호출)
- ✅ Wave 2A 호흡 키프레임 3장 (S1b/S7b/S8b) — 보관, 옵션 A 선택으로 미사용
- ✅ Wave 2B 표정 디테일 2장 (S2-half, S1c) — 통합됨
- ✅ Wave 2C 걷기 2장 (W1, W2) — 2프레임 사이클로 단순화
- ✅ 호흡 시스템 옵션 A 통합 (CSS 미세 transform sine wave)
- ✅ 부드러운 4단계 깜빡 시퀀스
- ✅ 잔잔 눈 웃음 (S1c) 가끔 등장
- ✅ 백업 자산 정리 (s1-extra-happy, s7-extra-greet)

다음:
- Wave 2D HAPPY (S4 + S4b ⭐ + E_petals) — 친구 훅 자산
- Wave 2E (E2, BG1 등) — NAPPING 풍부함
- Day 5 본격 (클릭 패스스루, Spaces, 위치 저장)
- Day 7 걷기 시스템 코드 통합 (W1↔W2 + SHY_AWAY 마우스 회피)

### Day 4 — 2026-04-30 (완료) ✅
**Tauri 셋업 + 투명·프레임리스·alwaysOnTop 창**

목표: HTML 프로토타입을 진짜 데스크탑 펫으로 띄우기.

체크리스트:
- [x] Rust 1.95.0 설치
- [x] Node.js v24.14.1 / npm 11.11.0 확인 (이미 있음)
- [x] Xcode CLT 확인 (이미 있음)
- [x] Tauri 프로젝트 생성 ([app/](app/), Vanilla 템플릿)
- [x] npm install
- [x] [tauri.conf.json](app/src-tauri/tauri.conf.json) 설정 (240×240, 투명, 프레임리스, alwaysOnTop, skipTaskbar, shadow:false, macOSPrivateApi)
- [x] 자산 복사 (5장 PNG → [app/src/assets/heehee/](app/src/assets/heehee/))
- [x] [app/src/index.html](app/src/index.html) 작성 (idle blink + breathe + 키보드 디버깅)
- [x] 첫 Rust 빌드 완료 ✅
- [x] **🎉 희희가 진짜 데스크탑에 떠있음 (2026-04-30 새벽)** 🌼

추가 작업 (2026-04-30 새벽):
- [x] 창 크기 240→200→160 (사용자 피드백 반영)
- [x] CSS 진폭 호흡 시도 → 통째로 부푸는 게 어색 → 제거
- [x] `transform-origin: 50% 78%` + scaleY로 배만 부풀리는 호흡 시도
- [x] 깜빡임 빈도 ↑ + 더블 블링크 (살아있는 동물처럼)
- [x] 상태별 호흡 사이클 다르게 (AWAKE 3.5s / NAPPING 4.5s 깊은 호흡)
- [x] AWAKE에서 가끔 EAR 쫑긋 잡동작 추가
- [x] **드래그로 창 이동** — JS `appWindow.startDragging()` + capabilities 권한 추가
- [x] **사용자 인사이트로 방향 전환**: 진폭 트릭 → PNG 키프레임 교체 결정
- [x] **걷기 시퀀스 비전 추가**: Phase 2 → MVP 승격
- [x] 자산 계획 26→36장 확장 (Wave 2A~E 재구성)

키보드 단축키 (창 떴을 때):
- `1`: AWAKE / `2`: 깜빡 / `3`: EAR / `7`: CHARGING / `8`: NAPPING

---

## 🎨 자산 진척 상세 (Wave별)

```
Wave 1 (핵심)            [██████████████████████] 10/10 ✅ 완료
Wave 2A (호흡 키프레임)  [██████████████████████]  3/3  ✅ 자산 보관 (옵션 A 선택, 미사용)
Wave 2B (표정 디테일)    [██████████████████████]  2/2  ✅ 통합 완료
Wave 2C (걷기 사이클)    [██████████████████████]  2/2  ✅ 4→2 단순화 완료
Wave 2D (HAPPY)          [░░░░░░░░░░░░░░░░░░░░░░]  0/3  ⭐ 다음 우선순위
Wave 2E (NAPPING+양손)   [░░░░░░░░░░░░░░░░░░░░░░]  0/6
Wave 3 (행동 풍부함)     [░░░░░░░░░░░░░░░░░░░░░░]  0/8
```

### Wave 1 ✅ (10장)
REF×4 + S1, S2, S3, S7, S8 + P3

### Wave 2A 🌬️ 호흡 키프레임 ✅ 보관
S1b + S7b + S8b — 옵션 A 선택으로 미사용. 나중에 옵션 C(3프레임 호흡) 갈 때 활용 가능

### Wave 2B 😌 표정 디테일 ✅ 통합
S2-half (부드러운 깜빡) + S1c (잔잔한 눈 웃음)

### Wave 2C 🚶 걷기 사이클 ✅ 단순화
W1 + W2 — 2프레임 사이클로 결정 (W3는 측면 포즈에서 강제 어려움, 다마고치 정석 워크플로우)

### Wave 2D 🌼 HAPPY (3장) ⭐ 다음 우선순위
S4 + S4b (친구 훅) + E_petals — 데스크탑 펫의 기쁨 인터랙션

### Wave 2E 💤 NAPPING + 양손 (5~6장)
E2 + BG1 + S3재시도 + S9 + S11 + S12

### Wave 3 ⬜ (8장)
S5 + S6 + S6b + S10 + S13 + S14 + P1 + P2

### 백업 자산 (extra)
s1-extra-happy, s7-extra-greet — Wave 2A 1차 시도 결과. 다른 표정/자세로 재활용 가능

---

## 🛠️ 코드 진척 상세

| 모듈 | 상태 |
|------|------|
| HTML 프로토타입 (브라우저) | ✅ 완료 |
| Tauri 환경 셋업 | ✅ Day 4 |
| 투명·프레임리스 창 | ✅ Day 4 (cocoa fix) |
| 위치 저장 / 복원 | ✅ Day 5 |
| 2D 스프라이트 시스템 통합 | ✅ Day 6 |
| Rust pmset 충전 감지 | ✅ Day 8 (substring 버그 fix 완료) |
| 트레이 메뉴 (이름/상태/풀주기/잠시쉬게하기/정보/이불) | ✅ Day 10~13 |
| Phase 2 모든 인터랙션 (FIRST_GREETING / FEED / DROWSY / YAWN / SURPRISED / STRETCH / TILT) | ✅ Day 13 |
| 영구 모달 영역 + character-area 분리 | ✅ Day 13 |
| 클릭 패스스루 (`setIgnoreCursorEvents`) | ⬜ Phase 3 |
| Spaces / Mission Control (`canJoinAllSpaces`) | ⬜ Phase 3 |
| 첫 실행 우하단 자동 배치 | ⬜ Phase 3 |
| PNG → WebP 번들 최적화 | ⬜ Phase 4 |
| .dmg 정식 빌드 | ⬜ Phase 4 |
| 톤 일괄 보정 (자산 단계) | ⬜ Phase 5 |

---

## 🚦 다음 액션

**지금 — dogfood + 친구 전달 직전 마무리**

1. **본인 dogfood 1일** — 실제 사용 시나리오 검증 (충전, 작업, 자기 등)
2. **Phase 3** (선택) — 클릭 패스스루 + 모든 Spaces + 첫 실행 우하단
3. **Phase 4** — PNG→WebP 변환 (목표 30MB 이하), .dmg 정식 빌드
4. **Phase 5** — 자산 톤 일괄 보정 (Photoshop/Pixelmator)
5. **Phase 6** — 친구 1명 AirDrop (Heehee.zip + 희희_사용법.md)

**옵셔널/Phase 4 선택지**

- 친구 1명용은 .zip + .app 우클릭 열기로도 충분 → .dmg 안 만들어도 OK
- 5명 친구로 확장 시 .dmg + 사용법 PDF 묶음 권장

---

## 📚 참고 문서

- [plan.md](plan.md) — 기획서, D1~D10 디자인 룰
- [schedule.md](schedule.md) — 14일 일정 + Wave 게이트
- [ref/prompts.md](ref/prompts.md) — 나노바나나 프롬프트
- [.claude/skills/heehee-design/SKILL.md](.claude/skills/heehee-design/SKILL.md) — 디자인 멘토 스킬
- [.claude/skills/heehee-eng/SKILL.md](.claude/skills/heehee-eng/SKILL.md) — 엔지니어 멘토 스킬
- [prototype/index.html](prototype/index.html) — 브라우저 프로토타입
