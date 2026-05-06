# 🌼 희희 (Heehee) — 2주 스케쥴

> 2026-04-29 작성. plan.md의 Step 1~7을 14일에 매핑한 실행 스케쥴.
> **목표:** 2주 후 .dmg를 본인 + 친구 1명에게 AirDrop으로 전달.

---

## 핵심 의사결정 (2026-04-29)

- **이미지 도구:** 나노바나나 (미드저니 X). 프롬프트는 [ref/prompts.md](ref/prompts.md)에 정합.
- **🌼 2D 트랙 확정 (2026-04-29 야간):** Meshy/Tripo 무료 export 막힘 → 2D pivot.
  - **렌더링:** HTML/CSS + 정적 PNG 다중 스프라이트 (Tauri webview)
  - **애니메이션:** CSS keyframe (호흡, 통통 튀기, fade) + PNG 교체 (눈 깜빡, 하품)
  - **메인 자산:** S 시리즈 PNG ~14장 (이전엔 컨셉용, 이제 메인)
  - 향후 검토: Rive 2D (친구 반응 좋으면 Phase 2)
- **데이지 처리 = 하이브리드:** S 시리즈는 데이지 포함해서 통합 PNG, 풀↔데이지 swap만 분리 합성 (P3 단독 PNG 사용).
  - 손 ↔ 귀 전환: 각 포즈 PNG에 그대로 (S9 STRETCHING은 귀에 꽂힌 버전 따로)
  - 풀↔데이지 swap: 코드로 P3 위치에 풀 PNG 합성
  - 미래 데이지 계절 변화: P3만 교체하면 됨

## 컨셉

- **Phase 1 (1주차):** 캐릭터 + 존재감 — *"희희가 떠있다"*
- **Phase 2 (2주차):** 룸메이트의 영혼 + 배포 — *"희희가 친구 손에 있다"*

---

## 🌼 Phase 1 (Day 1~7) — 캐릭터 + 존재감 만들기

> **종료 조건:** 주말 끝에 희희가 데스크탑 우하단에서 눈을 깜빡이고 있다.

| Day | 작업 | Step | 결과물 |
|-----|------|------|--------|
| Day 1 | 나노바나나로 희희 외모 확정 — REF-1~4 (정면/측면/3⁄4/뒷면). **꽃 포함, 외모 평가용** | Step 1 | ✅ 레퍼런스 시트 4장 (2026-04-29 완료) |
| Day 2 | **3D 분기점 검증 시도** (Meshy → Tripo → 무료 export 막힘) → **2D pivot 확정** | Step 1.5 | ✅ 분기 결정 (2026-04-29 야간) |
| Day 3 | **S 시리즈 핵심 6장 + P3 데이지 단독** + **HTML 프로토타입 검증** (S1↔S2 깜빡임 + 호흡 CSS) | Step 1 자산 + 검증 | 메인 6장 + P3 + 동작 검증 |
| Day 4 | Tauri 프로젝트 셋업, 투명·프레임리스·alwaysOnTop 창 띄우기 | Step 2 | 빈 투명 창이 우하단에 떠있음 |
| Day 5 | 클릭 패스스루, 멀티 Space 표시, 위치 저장 / 복원 | Step 2 | 룸메이트의 "그릇" 완성 |
| Day 6 | **2D 스프라이트 시스템 구축** + Wave 2A/2B 자산 통합 (S1↔S1b 호흡 PNG 교체, S2-half 부드러운 깜빡, S1c 눈 웃음) | Step 3 | 살아있는 호흡 + 표정 |
| Day 7 | **걷기 시스템 (Wave 2C)** — 자발적 + **SHY_AWAY 마우스 회피** + S3 재시도 + 이미지 로드 실패 fallback | Step 3 | **Phase 1 종료: 희희가 걸어다님 + 부끄러워 비켜남** 🚶 |

### Phase 1 게이트 — Day 2 분기점 결과 (✅ 통과)

- **3D 트랙 검증:** Meshy 6 / Tripo3D 모두 무료 export 막힘
- **결정:** 2D pivot — HTML/CSS + 정적 PNG
- **장점:** 엔지니어링 가벼움, 200×200에서 더 또렷, 데스크탑 펫 정석
- **트레이드오프:** 디자인 자산 작업 ↑ (S 시리즈 14장)

---

## 🌼 Phase 2 (Day 8~14) — 룸메이트의 영혼 + 배포

> **종료 조건:** 다음 주말에 .dmg를 친구 1명에게 AirDrop. 본인은 이미 1일 dogfood 완료.

| Day | 작업 | Step | 결과물 |
|-----|------|------|--------|
| Day 8 | Rust `pmset` 충전 감지 + CHARGING_IDLE (S7↔S7b 호흡) → 5분 후 NAPPING (S8↔S8b 쎄근쎄근) + 풀밭 fade-in (BG1) + ZZZ (E2) | Step 4 | 맥북 충전하면 쎄근쎄근 잠든다 |
| Day 9 | STRETCHING (S9) + 쓰다듬기 → HAPPY (S4) + **데이지 꽃잎 이펙트** (E_petals, CSS keyframe) | Step 5 | 핵심 인터랙션 + S9, S4 추가 자산 |
| Day 10 | 풀 먹이기 (`held_item` = **이미지 swap**, P3↔P2) + 트레이 메뉴 + 이름 말풍선 모달 (D6, D7) + S5, S6 자산 | Step 5 | 룸메이트 톤 완성 |
| Day 11 | FIRST_GREETING (S11, 1회성) + SURPRISED (S12, 꽃이 귀에) + 자정 풀→데이지 이미지 swap | Step 6 | S11, S12 추가 자산 |
| Day 12 | DROWSY (S10) + 랜덤 YAWNING (S13) + IDLE_GESTURE + S14 (이름 갸우뚱) + 모션 감소 대응 (D8) | Step 6 | **MVP 행동 셋 완성** + S10/S13/S14 자산 |
| Day 13 | .dmg 패키징, README 프라이버시 문구, "개발자 확인 안 됨" 우회 가이드, 본인 dogfood 1일 | Step 7 | 배포 직전 |
| Day 14 | 버그 픽스 + 본인 + 친구 1명에게 AirDrop 전달 | Step 7 | **🎁 선물 완료** |

---

## 🎨 자산 작업 분산표 (2D 트랙)

> 자산은 Day 작업과 분리해서 **미리** 뽑아둘 수 있음. 코드 진행과 별도로 사용자가 짬짬이 뽑기 가능.
> 단, 한 번에 16장 전부 뽑지 말고 **3개 묶음으로 나눠서** 톤 일관성 유지.

### Wave 1: 핵심 6장 + P3 — ✅ 완료 (2026-04-29)

| 자산 | 사용 Day | 상태 |
|------|---------|------|
| S1 awake-default | Day 6, 7, 12 | ✅ |
| S2 awake-blink | Day 6 | ✅ |
| S3 awake-eartwitch | Day 7 | 🟡 미세 재시도 권장 |
| S7 charging-idle | Day 8 | ✅ |
| S8 napping | Day 8 | ✅ |
| P3 prop-daisy | Day 10, 11 (swap용) | ✅ |

### Wave 2A: 호흡 키프레임 3장 ⭐ 다음 우선순위 — 살아있는 룸메이트 핵심

> 2026-04-30 사용자 인사이트 반영: CSS 진폭 폐기, PNG 키프레임 교체로 진짜 호흡.

| # | 자산 | 사용 Day | 비고 |
|---|------|---------|------|
| 1 | **S1b awake-breathe** | Day 6 | S1↔S1b 사이클로 호흡 |
| 2 | **S7b charging-breathe** | Day 8 | S7↔S7b 편안한 호흡 |
| 3 | **S8b napping-breathe** | Day 8 | S8↔S8b 깊은 쎄근쎄근 |

### Wave 2B: 표정 디테일 2장

| # | 자산 | 사용 Day | 비고 |
|---|------|---------|------|
| 1 | S2-half awake-blink-half | Day 6 | 깜빡 부드러운 중간 프레임 |
| 2 | S1c awake-eyes-soft | Day 6 | 잔잔한 눈 웃음 |

### Wave 2C: 걷기 시퀀스 4장 🚶 NEW (MVP 승격)

> 2026-04-30 추가: 가끔 자발적으로 그 자리에서 살짝 걸어다님. 30~60초마다 5초간.

| # | 자산 | 사용 Day | 비고 |
|---|------|---------|------|
| 1 | W1 walking-step1 | Day 7 | 측면, 오른발 앞 |
| 2 | W2 walking-step2 | Day 7 | 측면, 양발 모음 |
| 3 | W3 walking-step3 | Day 7 | 측면, 왼발 앞 |
| 4 | W4 walking-step4 | Day 7 | 측면, 양발 모음 (다음 사이클) |

### Wave 2D: HAPPY 인터랙션 3장

| # | 자산 | 사용 Day | 비고 |
|---|------|---------|------|
| 1 | S4 happy-bounce | Day 9 | 통통 튀기 |
| 2 | S4b happy-daisy-show | Day 9 | ⭐ **친구 훅** (D10) |
| 3 | E_petals daisy-petals | Day 9 | HAPPY 시그니처 이펙트 |

### Wave 2E: NAPPING + 양손 룰 + 배경 5~6장

| # | 자산 | 사용 Day | 비고 |
|---|------|---------|------|
| 1 | E2 effect-zzz | Day 8 | NAPPING ZZZ |
| 2 | BG1 grassfield | Day 8 | 풀밭 배경 |
| 3 | S3 재시도 (귀 쫑긋) | Day 7 | Wave 1 미세 재시도 |
| 4 | S9 stretching | Day 9 | 양손 → **꽃은 귀에** |
| 5 | S11 first-greeting | Day 11 | 최초 1회 인사 |
| 6 | S12 surprised | Day 11 | 양손 → **꽃은 귀에** |

### Wave 3: 행동 풍부함 + 풀 먹이기 8장 — Day 10 전 작업

| # | 자산 | 사용 Day | 비고 |
|---|------|---------|------|
| 1 | S5 idle-hungry | Day 10 | 먹고 싶다 표정 |
| 2 | S6 eating | Day 10 | 풀 먹는 중 |
| 3 | S6b awake-with-grass | Day 10 | 풀 들고 다니기 |
| 4 | S10 drowsy | Day 12 | 졸림 |
| 5 | S13 yawning | Day 12 | 하품, **꽃은 귀에** |
| 6 | S14 name-dialog-tilt | Day 12 | 이름 갸우뚱 |
| 7 | P1 prop-grass | Day 10 | 풀 |
| 8 | P2 prop-leaf | Day 10 | 나뭇잎 |

### 🚦 자산 작업 게이트 (Wave 시작 조건)

**검증 우선 워크플로우.** Wave 작업 시작 전 반드시 게이트 통과 확인.

| Wave | 시작 조건 | 검증 방법 |
|------|----------|----------|
| **Wave 1** | (없음) | ✅ 완료 (디자이너 검토 통과) |
| **Wave 2** | Wave 1 자산이 **HTML 프로토타입**에서 동작 검증 OK | [prototype/index.html](prototype/index.html) 브라우저로 열어 깜빡임/호흡 확인 |
| **Wave 3** | Wave 2 자산이 Tauri 통합 후 **Day 9 인터랙션** (HAPPY/STRETCHING)에서 동작 검증 OK | Day 9 작업 마지막에 통합 검증 |

**왜 게이트?** 22장 미리 다 뽑으면 코드 합쳐봤을 때 톤/위치/크기 어긋남이 발견될 경우 매몰 비용. 검증 통과한 톤으로 다음 Wave 진행하면 일관성 보장.

### 🌟 톤 일관성 가드

- **각 Wave는 같은 세션에서 뽑기** — 나노바나나 톤이 시간 지나면 미세 차이 발생
- **각 PNG 뽑을 때 ref-front.png + qqref.png 같이 업로드** (지원되면)
- **Wave 사이에는 1~2장 검증 후 다음 Wave** — Wave 1로 검증된 톤이 Wave 2에서 유지되는지 첫 1~2장으로 체크

### 🎯 권장 진행 순서

1. ✅ Wave 1 (6장) — 검증 완료
2. ⏳ **HTML 프로토타입 검증** ← 🚦 Wave 2 게이트
3. ⏳ Wave 2 (8장) — 게이트 통과 후
4. (Day 4~5 코드 작업과 병행 가능)
5. Wave 3 (9장) — Day 9 끝나기 전

---

## 리스크 & 버퍼 전략

- **Day 3 (S 시리즈 6장 + P3):** 자산 작업 분량 많음. 한 장당 15~20분이면 여유. 핵심 자산(S1, S2, S7, S8) 우선.
- **Day 4~5 (Tauri 투명 창 / 클릭 패스스루 / Spaces):** 이 2주에서 가장 어려운 구간. 비엔지니어 + AI 페어 코딩 가정. 막히면 Day 7에 흡수.
- **Day 6 (2D 스프라이트 시스템):** Three.js 부담 없어졌음. CSS keyframe + PNG 교체는 비엔지니어에게 비교적 쉬움. 추가 자산(S3) 동시 작업 가능.
- **Day 9~12 (자산 + 행동):** 매일 자산 추가 + 코드 통합. 자산 막히면 코드 먼저 (placeholder 색상으로 시작) → 자산 채우기.
- **Day 10 (트레이 메뉴):** 이름 검증 묵직. `held_item`은 이미지 swap 한 줄로 끝남 — 부담 적음.
- **Day 14는 비워두는 게 정답:** 버그 픽스 + 친구 전달 외에 다른 작업 금지.

---

## 이번 2주에 **안** 넣는 것 (plan.md "Phase 2 (나중에)")

- WALKING_EDGE (모서리 걷기)
- 멀티 모니터 이동
- 친구 쿼카 방문
- 공유 모먼트 (Cmd+Shift+H GIF)
- 데이지 계절 변화
- 카메라 / 마이크 / 날씨 / IDE 인식

→ **친구가 "안 지웠다"는 신호 받은 다음에 다시 평가.**

---

## Definition of Done (Day 14 기준)

plan.md의 Success Criteria 기능적 목표 전부:

- [x] 맥북 코너에 희희가 떠있다
- [x] 충전 꽂으면 풀밭 나오고 잠든다
- [x] 충전 빼면 기지개 켜고 깨어난다 (S9 STRETCHING)
- [x] 클릭하면 반응한다 (+ 데이지 살짝 보임 — S4b)
- [x] 밤 11시 이후엔 졸려 보인다 (DROWSY S10 + 잦은 YAWN)
- [x] 이름 바꾸면 놀란다 (SURPRISED S12)
- [x] 풀을 주면 풀을 쥐고 다니다가 다음 날 아침 데이지로 돌아간다 (S5/S6/S6b + 자정 swap)
- [x] 트레이 메뉴에서 조용히 재울 수 있다 (잠시 쉬게 하기 — YAWN→hide→1시간→show+STRETCH)
- [ ] 런타임 메모리 100MB 이하, 번들 크기 30MB 이하 (현재 65MB → Phase 4 WebP 변환 필요)
- [ ] 본인 + 친구 1명 .dmg 전달 완료 (Heehee.zip 준비됨, dogfood 진행 중)

---

*🌼 희희가 2주 후 친구의 맥북에서도 웃고 있길*
