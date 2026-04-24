# 희희 데스크탑 펫 — 나노바나나 프롬프트

## 사용법
1. 나노바나나에 qqref.png를 **레퍼런스 이미지**로 업로드 (my-bday-card/ref/qqref.png)
2. 아래 프롬프트를 **그대로 복사 → 붙여넣기**
3. 생성된 이미지를 `heehee/ref/images/`에 저장

> 데스크탑 펫은 200×200px로 표시되므로, 작아도 잘 보이게 단순하고 뚜렷한 실루엣이 중요!
> 모든 이미지에서 희희는 **데이지 꽃**(하얀 꽃잎, 노란 수술, 연두 줄기)을 가지고 있어야 함!
> **꽃 위치 룰:** 평소엔 **오른손**. 양손 쓰는 포즈(STRETCHING, YAWNING, SURPRISED)에선 **오른쪽 귀에 꽂음**.

---

## 공통 스타일 키워드
> 모든 프롬프트 앞에 이걸 붙이세요:

```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic,
```

---

## 1. 캐릭터 레퍼런스 (4장) — 외모 확정용

### REF-1 — ref-front.png
> 정면 전신 기본 포즈
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, full body, standing upright, gentle happy smile, Pixar style big eyes with crescent moon shape, ears slightly perked up
```

### REF-2 — ref-side.png
> 옆모습 전신
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, side view facing right, full body, standing upright, gentle happy smile, Pixar style
```

### REF-3 — ref-threequarter.png
> 3/4 앵글
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, three-quarter view, full body, standing upright, gentle happy smile, slightly tilted head, Pixar style
```

### REF-4 — ref-back.png
> 뒷모습
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes, pink blush cheeks, light beige fur, holding a small white daisy flower in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, back view, full body, standing upright, small round tail visible, Pixar style
```

---

## 2. 상태별 포즈 (10장) — 실제 앱에서 사용

### AWAKE 상태 (기본)

#### S1 — awake-default.png
> 기본 서있는 포즈. 화면을 바라보며 웃고 있음
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, standing upright, looking slightly upward at camera, gentle happy smile with upturned mouth corners, crescent moon eyes, ears perked up, curious and content expression
```

#### S2 — awake-blink.png
> 눈 깜빡임 (눈 감은 순간)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes closed shut, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, standing upright, eyes fully closed in a blink, happy smile still on face, relaxed expression
```

#### S3 — awake-eartwitch.png
> 귀 쫑긋 (뭔가 들은 것처럼)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, standing upright, one ear raised higher than the other, curious alert expression, head slightly tilted, happy smile
```

### HAPPY 상태 (쓰다듬기 반응)

#### S4 — happy-bounce.png
> 기뻐서 통통 튀는 포즈
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes squeezed shut with joy, pink blush cheeks extra rosy, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, jumping up in the air with both feet off ground, arms raised in excitement, overjoyed expression, biggest smile, sparkle effects around
```

### IDLE_GESTURE 상태 (먹고 싶다)

#### S5 — idle-hungry.png
> 풀을 쳐다보며 "먹고 싶다" 표정
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes looking downward to the side, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, standing upright, looking down at something on the ground with curious hungry expression, one hand near mouth, slight drooling cute expression, head tilted down
```

### EATING 상태 (풀 먹기)

#### S6 — eating.png
> 풀을 먹는 중
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes happy and content, pink blush cheeks, light beige fur, holding a small white daisy flower in right hand, LEFT hand holding a small green leaf near mouth, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, sitting down, happily munching on a green leaf, satisfied chewing expression, eyes closed in bliss, small green leaf sticking out of mouth
```

### CHARGING_IDLE 상태 (충전 중 멍때리기)

#### S7 — charging-idle.png
> 앉아서 멍때리는 포즈
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with soft unfocused gaze, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand resting on lap, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, sitting down on the ground with legs spread forward, relaxed slouchy posture, peaceful blank stare, zoning out expression, content and calm
```

### NAPPING 상태 (낮잠)

#### S8 — napping.png
> 눈 감고 자는 포즈 (데이지 꼭 쥔 채)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, eyes fully closed sleeping peacefully, pink blush cheeks, light beige fur, clutching a small white daisy flower with yellow center and green stem tightly in right hand even while sleeping, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, curled up sitting position, head tilted to one side, peaceful sleeping face with gentle smile, cozy and warm feeling
```

### STRETCHING 상태 (기지개)

#### S9 — stretching.png
> 기지개 켜는 포즈. **양손 사용 → 꽃은 오른쪽 귀에 꽂혀 있음**
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes squinting half-open, pink blush cheeks, light beige fur, a small white daisy flower with yellow center and green stem tucked behind right ear, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, standing up with both arms raised high above head in a big stretch, yawning slightly, just woke up expression, eyes half open and sleepy
```

### DROWSY 상태 (졸림 — MVP 승격)

#### S10 — drowsy.png
> 졸린 표정 + 하품
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes droopy and half-closed, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, front view, standing but slightly swaying, big yawn with mouth wide open, sleepy droopy eyes, tired but still cute expression, one hand rubbing eye
```

---

## 3. 소품 & 배경 (3장)

### P1 — prop-grass.png
> 풀/잔디 (먹이용)
```
3D render, soft plush toy style. A small cute tuft of green grass, simple and minimal, 3-4 blades of bright green grass, soft felt texture, looks like a plush toy grass patch. Pastel green color, studio lighting, white background.
```

### P2 — prop-leaf.png
> 나뭇잎 (먹이 아이템)
```
3D render, soft plush toy style. A single small cute green leaf, simple oval shape, soft felt/plush texture, bright pastel green with slightly darker vein lines, looks like a toy leaf. Studio lighting, white background.
```

### BG1 — bg-grassfield.png
> 충전 중 풀밭 배경
```
3D render, soft plush toy style. A small cozy grass field patch, round/oval shape, bright green fluffy grass with tiny pastel flowers scattered (white, pink, yellow), soft felt texture like a miniature garden. Seen from slightly above, circular composition, warm gentle lighting, transparent or white background. This will appear behind a small character as a resting spot.
```

---

## 4. 이펙트 (코드로 처리 가능하지만, 이미지로 만들어두면 더 예쁨)

### E1 — effect-heart.png (선택)
> 하트 이펙트 (HAPPY 상태에서 사용)
```
3D render, soft plush toy style. A small cute pink heart, puffy and soft looking, plush felt texture, slightly shiny, pastel pink color, studio lighting, white background. Simple and minimal.
```

### E2 — effect-zzz.png (선택)
> ZZZ 이펙트 (NAPPING 상태에서 사용)
```
3D render, soft plush toy style. The letters "ZZZ" in a cute bubbly font, pastel blue/lavender color, soft puffy 3D text, plush felt texture, getting smaller from left to right (Z z z), studio lighting, white background.
```

### E3 — effect-sparkle.png (선택)
> 반짝이 이펙트 (HAPPY 상태에서 사용)
```
3D render, soft plush toy style. A small cute sparkle/star burst, 4-pointed star shape, pastel yellow/gold color, soft glowing, plush felt texture, studio lighting, white background. Simple and minimal.
```

---

## 체크리스트

**필수 (15장 — MVP):**
- [ ] REF-1 ref-front.png (정면)
- [ ] REF-2 ref-side.png (옆면)
- [ ] REF-3 ref-threequarter.png (3/4)
- [ ] REF-4 ref-back.png (뒷면)
- [ ] S1 awake-default.png (기본)
- [ ] S2 awake-blink.png (눈 깜빡)
- [ ] S3 awake-eartwitch.png (귀 쫑긋)
- [ ] S4 happy-bounce.png (기쁨 점프)
- [ ] S5 idle-hungry.png (먹고 싶다)
- [ ] S6 eating.png (먹는 중)
- [ ] S7 charging-idle.png (앉아 멍때리기)
- [ ] S8 napping.png (낮잠)
- [ ] S9 stretching.png (기지개, **꽃은 귀에**)
- [ ] S10 drowsy.png (졸림 하품) — **MVP 승격됨**
- [ ] BG1 bg-grassfield.png (풀밭 배경)

**추가 권장 (plan.md D1~D10 반영, 복귀 시 프롬프트 추가 필요):**
- [ ] S11 first-greeting.png (첫 실행 인사 — 데이지를 앞으로 내밀며)
- [ ] S12 surprised.png (이름 변경 반응, **꽃은 귀에**)
- [ ] S13 yawning.png (하품 단독, **꽃은 귀에**)
- [ ] S14 name-dialog-tilt.png (이름 입력 에러 시 머리 갸우뚱)

**선택 소품 (5장):**
- [ ] P1 prop-grass.png (풀)
- [ ] P2 prop-leaf.png (나뭇잎)
- [ ] E1 effect-heart.png (하트)
- [ ] E2 effect-zzz.png (ZZZ)
- [ ] E3 effect-sparkle.png (반짝이)

**총: 필수 14장 + Phase 2 1장 + 선택 5장 = 최대 20장**

---

## 뽑는 순서 추천

1. **REF-1 (정면)** 먼저 — 이게 스타일 기준!
2. REF-2~4 (나머지 앵글)
3. S1 (기본 AWAKE) — 가장 많이 보이는 포즈
4. S7, S8 (충전 중/낮잠) — 핵심 기능
5. S4 (기쁨) — 인터랙션
6. 나머지 상태들
7. 소품/이펙트는 마지막

## 팁
- **데이지 꽃 일관성**: 평소엔 오른손, 양손 쓰는 포즈(STRETCHING 등)는 **오른쪽 귀에 꽂은 모습**! 빠지면 다시 뽑기
- **200×200px로 표시됨**: 너무 디테일한 건 안 보임. 실루엣이 뚜렷한 게 중요
- **배경 제거 필수**: 모든 캐릭터/소품은 투명 배경이어야 데스크탑 위에 자연스러움
- **이펙트 방향**: HAPPY 이펙트는 하트(E1) 대신 **데이지 꽃잎**으로 — plan.md D2 참조
