---
name: heehee (데이지 쿼카 데스크탑 펫) 프로젝트
description: 맥북 데스크탑 펫 개인 프로젝트. 2026-04-29~30 야간에 본격 개발 — Day 1~4 완료, 2D pivot 확정, 희희가 진짜 데스크탑에 떠있음. 핵심 비전: "2D지만 동적인 희희" — 다중 PNG 키프레임 교체로 진짜 살아있는 룸메이트
type: project
originSessionId: 794bdd9c-2820-4d60-a22d-665085100ac3
---
맥OS용 픽사 톤 쿼카 데스크탑 펫. 본인 + 친구 5명 선물용. 경로: [heehee/](../../Documents/MYproject/heehee/).

# 🌟 핵심 비전 (절대 잊지 말 것)

**"2D지만 동적인 희희"**

- 2D 트랙으로 결정했지만 **정적인 일러스트가 아님**
- **다중 PNG 키프레임 교체**로 진짜 살아있는 룸메이트
- 픽사/디즈니 2D 단편, Cuphead 같은 톤
- 호흡, 깜빡임, 걷기, 표정 변화 — 다 PNG 사이클로 표현
- **CSS 진폭 트릭은 보조에 불과** — 본질은 PNG 교체

이 비전은 사용자가 명시적으로 강조했음 (2026-04-30 새벽). 향후 작업에서 정적/단순화 방향으로 후퇴하지 말 것.

---

## Why

개인 사이드 프로젝트. "룸메이트 · 강제성 없음" 철학이 제품·UI·종료 경로까지 관통.
선물의 본질 = 친구가 매일 켜두는 살아있는 존재. 그래서 동적 표현이 필수.

---

## 현재 진척 (2026-05-03 기준)

- ✅ Day 1~13 완료 — **Phase 1+2 모두 통합, dogfood 직전**
- 🌼 **MVP 행동 셋 완성** — 7가지 인터랙션 모두 작동:
  1. FIRST_GREETING (S11→S11a, 1회성)
  2. 충전 → CHARGING → NAPPING → STRETCHING (해제 시)
  3. 풀 먹이기 (트레이) → S5→S6→S6b, held_item swap, 자정 복귀
  4. DROWSY (23:00~06:00) + YAWNING (자발적, DROWSY는 잦게)
  5. SURPRISED (이름 변경)
  6. TILT (이름 검증 실패)
  7. SHY_AWAY (마우스 다가오면 휘리릭 1초 도망)
- 트레이 메뉴: 이름 + 상태 동적 표시, 풀 주기, 잠시 쉬게 하기 (YAWN→hide→1시간→show+STRETCH), 정보, 종료
- 자산: 사용 28장 + 보관 5장 + REF 4장 = **37장**
  - **Phase 2 추가 (11장)**: S11, S11a, S5, S6, S6b, P2, S10, S13, S12, S9, S14
- 친구 전달 패키지: Heehee.zip + 희희_사용법.md ✅ (Desktop)
- **남은 것**: Phase 3 (클릭 패스스루 / Spaces / 우하단) + Phase 4 (WebP 30MB / .dmg) + Phase 5 (톤 보정) + Phase 6 (dogfood + AirDrop)

## 핵심 의사결정 (다 반영됨)

- **2D pivot 확정 (2026-04-29 야간):** Meshy/Tripo 무료 export 막힘 → HTML/CSS + 정적 PNG 트랙. Three.js / Mixamo 폐기.
- **🌟 2D지만 동적 (2026-04-30):** 사용자 인사이트 — "CSS 진폭으로는 자연스러움 한계, 픽사 2D 단편처럼 다중 PNG가 정답". CSS 진폭 호흡 폐기. PNG 키프레임 교체로 진짜 호흡.
- **걷기 시퀀스 MVP 승격 (2026-04-30):** Phase 2 → 본 작업. W1~W4 4 프레임 사이클로 가끔 자발적 걷기.
- **SHY_AWAY 마우스 회피 추가 (2026-04-30 새벽):** 사용자 피드백 "사용에 방해가 된다" → 마우스가 희희에 다가오면 부끄러워 살짝 비켜남. 룸메이트 강제성 없음 철학 강화. 자산 W1~W2 그대로 활용 (CSS scaleX 좌우반전), Day 7 걷기 시스템에 통합.
- **Tauri 회색창 fix (2026-04-30 저녁):** macOS Tauri 2 webview 기본 회색 배경 이슈. lib.rs에 cocoa 직접 호출로 NSWindow.setOpaque(false) + setBackgroundColor(clearColor). Cargo.toml에 cocoa+objc 의존성 추가.
- **호흡 옵션 A 채택 (2026-04-30 저녁):** PNG cross-fade가 "갑자기 커지는 느낌" 어색. CSS 미세 transform sine wave로 전환 (배만 1.2% scaleY, transform-origin 50% 80%, 위아래 1.5px 들썩, 사이클 3.8s). 호흡 들숨 PNG (S1b/S7b/S8b)는 보관 — 옵션 C 3프레임 호흡 갈 때 활용 가능.
- **부드러운 깜빡 4단계 (2026-04-30 저녁):** S1 → S2-half → S2 → S2-half → S1 시퀀스 (각 80~100ms). 진짜 눈꺼풀이 내려갔다 올라오는 자연스러움.
- **잔잔한 눈 웃음 (S1c) 가끔 등장 (2026-04-30 저녁):** AWAKE에서 25~40초마다 4초간 S1c 표정. "지금 행복해" 룸메이트 표정.
- **걷기 4→2 프레임 단순화 (2026-04-30 밤):** 측면 포즈에서 모델이 항상 카메라쪽 발을 앞으로 그림 (W3 강제 어려움). 다마고치/마리오 정석 — 2프레임 사이클 (W1↔W2)로 충분히 살아있는 걷기. 자산 +2장 절약. **→ 2026-05-03 4프레임 복귀 (W3/W4 자산 다시 살리고, 톤 보정은 Phase 5에서 자산 단계로 미룸).**
- **자발적 어슬렁 폐기 (2026-05-03):** 사용자: "마우스 갖다댈때 휘리릭만 있고 자동으로 어슬렁거리는건 없어". 룸메이트가 산만해 보이는 게 더 큰 문제. SHY_AWAY (1초/120px/3초 쿨다운)만 남김.
- **창 240×240 → 240×460 영구 확장 (2026-05-03):** 동적 리사이즈 시 모달 열고닫는 사이 캐릭터가 갑자기 움직이는 결함. HTML 구조 분리 — `.modal-area` (위 220px) + `.character-area` (아래 240px). 희희는 절대 안 움직임. SHY_AWAY는 character-area에서만 발동.
- **`pmset` substring 버그 fix (2026-05-03):** `discharging`이 `charging` 부분 매칭으로 false-positive — 시작 시 충전 상태로 잘못 표시. `s.contains("AC Power")` 단독 체크로 수정. 룸메이트 디테일 함정 — 친구한테 갔을 때 충전 안 하는데 자고 있으면 신뢰 깨짐 포인트였음.
- **잠시 쉬게 하기 D안 (2026-05-03):** YAWN(1초) → hide → 1시간 → show + STRETCH(1.5초). 사라지는 모습 + 깨어나는 모습이 룸메이트답게 자연스러워짐. 트레이 라벨에 "(1시간 뒤 복귀)" 명시.
- **풀↔데이지 룰 재정립 (2026-05-03):** 옵션 B — 풀 먹이는 동안 데이지는 오른쪽 귀에 꽂힘. 자정 자동 데이지 손 복귀. 쿼카는 풀이 아니라 나뭇잎 먹는다는 사용자 인사이트 → P1(풀) 폐기, P2(나뭇잎)만 자산화.
- **데이지 = 하이브리드 합성**: S 시리즈는 데이지 통합, 풀↔데이지 swap만 P3 단독 분리.
- **시그니처 꽃 = 데이지** (민들레 X). 평소 오른손, 양손 포즈에선 오른귀.
- **HAPPY 이펙트 = 데이지 꽃잎** (하트/sparkle ❌, plan.md D2).

## 자산 총량 (2D 동적 비전 반영)

26장 → 36장 → **34장으로 조정** (걷기 4→2 단순화):
- Wave 1 ✅: REF×4 + S1, S2, S3, S7, S8 + P3 = 10장
- Wave 2A ✅ 보관: 호흡 키프레임 3장 (S1b, S7b, S8b)
- Wave 2B ✅ 통합: 표정 디테일 2장 (S2-half, S1c)
- Wave 2C ✅ 통합: 걷기 사이클 2장 (W1, W2) — 4→2 단순화
- Wave 2D ⏳ 다음: HAPPY 3장 (S4, S4b, E_petals) ⭐ 친구 훅
- Wave 2E: NAPPING+양손 6장 (E2, BG1, S3재시도, S9, S11, S12)
- Wave 3: 행동 풍부함 8장
- 백업: s1-extra-happy, s7-extra-greet (Wave 2A 1차 시도, 재활용 가능)

## How to apply

- **재시작 명령**: `cd /Users/yeorim.choi/Documents/MYproject/heehee/app && source ~/.cargo/env && bun run tauri dev`
- **Tauri 설정 위치**: [app/src-tauri/tauri.conf.json](../../Documents/MYproject/heehee/app/src-tauri/tauri.conf.json) — **240×460** (모달 영역 220 + 캐릭터 영역 240), transparent, decorations:false, alwaysOnTop, macOSPrivateApi
- **HTML 구조**: `.heehee-frame` flex column = `.modal-area` (위 220px) + `.character-area` (아래 240px). 캐릭터는 항상 character-area 내부 flex-center, 모달은 modal-area 내부에 absolute로 채움. 동적 리사이즈 X.
- **HTML/JS 변경 후 적용**: dev mode는 frontend hot-reload 없음 (`frontendDist: "../src"` 정적). 캐릭터 우클릭 → Inspect → 콘솔 `location.reload()` 또는 dev 재시작.
- **Rust 변경 후**: 재컴파일 필요 (lib.rs 등). dev 자동 watch.
- **드래그**: JS `appWindow.startDragging()` (data-tauri-drag-region 백업), capabilities에 `core:window:allow-start-dragging` 권한
- **Wave 자산 게이트**: HTML/Tauri 검증 통과 → Wave 2 작업 진행 중
- **다음 작업 우선순위**: **Wave 2D HAPPY (S4 + S4b ⭐ + E_petals)** — 친구 훅의 핵심. 그 다음 Wave 2E (E2, BG1 등) → Day 5/7 본격 코드
- **자산 작업 룰**: 매번 [ref-front.png](../../Documents/MYproject/heehee/ref/images/ref-front.png) + [qqref.png](../../Documents/MYproject/heehee/ref/qqref.png) 같이 업로드 (지원되면). 톤 일관성 가드.
- **배경 제거**: macOS 빠른 작업 "배경 제거" 자동 (Sonoma+).
- **호흡 룰**: 모든 동적 표현은 **PNG 키프레임 교체**가 메인, CSS는 보조 (translateY, fade만).

## 작업 워크플로우

- 자산 작업은 **Wave 단위로 검증 후 다음** (게이트). 22장 미리 다 뽑지 말 것 — 톤 어긋남 매몰비용.
- 사용자는 비엔지니어 — 코드보다 자산/디자인이 본인 영역. AI 페어가 코드 부담 흡수.
- 사용자가 "동적", "더 살아있게", "촘촘하게" 같은 말을 하면 → 자산 추가/PNG 키프레임 확장 방향. 코드 진폭 트릭으로 도망치지 말 것.

## 참고 문서

- [plan.md](../../Documents/MYproject/heehee/plan.md) — 기획서, D1~D10 디자인 룰, 상태 시스템 (WALKING 추가됨)
- [schedule.md](../../Documents/MYproject/heehee/schedule.md) — 14일 일정 + Wave 게이트 + Wave 2A~E 분산표
- [dailylog.md](../../Documents/MYproject/heehee/dailylog.md) — 일별 진척 + 프로그레스 바 + 의사결정 타임라인
- [ref/prompts.md](../../Documents/MYproject/heehee/ref/prompts.md) — 나노바나나 프롬프트 (호흡 키프레임/걷기 시퀀스 포함)
- [.claude/skills/heehee-design/SKILL.md](../../Documents/MYproject/heehee/.claude/skills/heehee-design/SKILL.md) / [heehee-eng/SKILL.md](../../Documents/MYproject/heehee/.claude/skills/heehee-eng/SKILL.md) — 멘토 스킬 (호출 시 자동 활성화)
- [prototype/index.html](../../Documents/MYproject/heehee/prototype/index.html) — 브라우저 프로토타입 (Wave 1 검증용)
- [app/](../../Documents/MYproject/heehee/app/) — Tauri 프로젝트 본체
