# 희희 데스크탑 펫 — 나노바나나 프롬프트

_2026-04-29 plan.md 정합성 정리 (D2 데이지 꽃잎 이펙트 / D9 아웃라인 / MVP 필수 재분류)_
_2026-04-29 야간: 2D pivot 확정 — S 시리즈 ~14장이 메인 자산으로 승격_

## 🌼 2D 트랙 메모

- **모든 PNG는 Tauri webview에서 직접 표시.** Three.js / Meshy / Mixamo 단계 삭제.
- **데이지 처리 = 하이브리드:** S 시리즈는 데이지 포함해서 통합 PNG, 풀↔데이지 swap만 P3 단독 PNG로 분리 합성.
- **눈 깜빡 / 호흡 / 통통 튀기는 CSS로** — S2(눈 감음)와 S1(눈 뜸) 교체 + CSS scale 변환.
- **하품·기지개 같은 연속 동작은 PNG 2~3장 빠른 교체** (0.15~0.3초).
- **희희 1캐릭터당 평균 1~2번 시도면 OK** (이미 REF 톤이 잡혀 있어 일관성 좋음).

## 사용법
1. 나노바나나에 [qqref.png](qqref.png)를 **레퍼런스 이미지**로 업로드 (`heehee/ref/qqref.png`)
2. 아래 프롬프트를 **그대로 복사 → 붙여넣기**
3. 생성된 이미지를 [heehee/ref/images/](images/)에 저장

> 데스크탑 펫은 200×200px로 표시되므로, 작아도 잘 보이게 단순하고 뚜렷한 실루엣이 중요!
> 모든 이미지에서 희희는 **데이지 꽃**(하얀 꽃잎, 노란 수술, 연두 줄기)을 가지고 있어야 함!
> **꽃 위치 룰:** 평소엔 **오른손**. 양손 쓰는 포즈(STRETCHING, YAWNING, SURPRISED, FIRST_GREETING 양손 인사 시)에선 **오른쪽 귀에 꽂음**.
> **다크 데스크탑 대응(D9):** 모든 캐릭터 외곽에 **1px 어두운 아웃라인**을 살짝 둘러서 다크모드 배경에서도 묻히지 않도록.

---

## 공통 스타일 키워드
> 모든 프롬프트 앞에 이걸 붙이세요:

```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette for visibility on dark backgrounds,
```

---

## 1. 캐릭터 레퍼런스 (4장) — 외모 확정용

### REF-1 — ref-front.png
> 정면 전신 기본 포즈. **완전한 정면 — 좌우 대칭, 카메라 정면 응시**
```
perfectly symmetrical front view, facing camera directly head-on, zero rotation, no three-quarter angle, both eyes equally visible and aligned, both ears equally visible and symmetric, both arms symmetric on each side, body perfectly centered in frame, orthographic front projection, 3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, full body standing upright, gentle happy smile, Pixar style big eyes with crescent moon shape, ears slightly perked up
```

### REF-2 — ref-side.png
> 옆모습 전신
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, side view facing right, full body, standing upright, gentle happy smile, Pixar style
```

### REF-3 — ref-threequarter.png
> 3/4 앵글
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, three-quarter view, full body, standing upright, gentle happy smile, slightly tilted head, Pixar style
```

### REF-4 — ref-back.png
> 뒷모습
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes, pink blush cheeks, light beige fur, holding a small white daisy flower in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, back view, full body, standing upright, small round tail visible, Pixar style
```

---

## 2. 상태별 포즈 (MVP 필수, 14장) — 실제 앱에서 사용

### AWAKE 상태 (기본)

#### S1 — awake-default.png
> 기본 서있는 포즈. 화면을 바라보며 웃고 있음. **GLTFLoader 실패 시 fallback PNG로도 사용(D5)**
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, standing upright, looking slightly upward at camera, gentle happy smile with upturned mouth corners, crescent moon eyes, ears perked up, curious and content expression
```

#### S2 — awake-blink.png
> 눈 깜빡임 (눈 감은 순간)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes closed shut, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, standing upright, eyes fully closed in a blink, happy smile still on face, relaxed expression
```

#### S3 — awake-eartwitch.png
> 귀 쫑긋 (뭔가 들은 것처럼). **몸은 옆모습, 머리/귀만 정면으로 돌려 카메라를 봄** — "뭔가 들었어?" 느낌
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes wide open and bright with highlight sparkle, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, body in side view facing right (profile pose), but head turned and rotated toward camera looking directly forward at viewer, both ears clearly visible from the front and perked up sharply pointing upward, alert curious "did I hear something?" expression, ears as the focal point of the image, neck twisted slightly, happy gentle smile, standing upright, dynamic asymmetric pose with body and head facing different directions
```

### HAPPY 상태 (쓰다듬기 반응)

#### S4 — happy-bounce.png
> 기뻐서 통통 튀는 포즈. **이펙트는 데이지 꽃잎(D2) — 하트/sparkle 금지**
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes squeezed shut with joy, pink blush cheeks extra rosy, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, jumping up in the air with both feet off ground, arms raised in excitement, overjoyed expression, biggest smile, 3 to 5 small white daisy petals with yellow center floating up around the character
```

#### S4b — happy-daisy-show.png  ⭐ 친구 훅 (D10)
> 데이지를 자랑스럽게 카메라 쪽으로 내미는 "어머 귀여워" 순간
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, light beige fur, pink blush cheeks rosy, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, front view standing upright. POSE: right hand holding small white daisy flower with yellow center and green stem RAISED UP AND EXTENDED FORWARD TOWARD THE CAMERA — daisy is the centerpiece in the foreground, slightly closer to camera than the face. Like a small child shyly offering a flower they picked. Body leaning slightly forward presenting the flower. Eyes squeezed into crescent moon closed-eye smile (눈 웃음). Small soft proud-and-shy closed smile, no open laugh. Head slightly tilted, cute and bashful expression. Left arm relaxed by body.
```

### IDLE_GESTURE 상태 (먹고 싶다)

#### S5 — idle-hungry.png
> 떨어진 나뭇잎을 발견하고 쳐다보는 표정 (강제성 X — 안 줘도 슬프지 않음)
```
MATCH the attached s1-awake-default.png in color tone, lighting, character size, fur texture, daisy color EXACTLY. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes looking down and to the side with curious longing expression, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, front view, standing upright. SCENE: a small fresh green leaf (with visible vein lines) is placed/sitting just in front of the character on the ground (lower part of frame), and the character is looking down at it with wishful hungry curiosity, head slightly tilted down, "is that for me?" expression, one paw hesitantly near the chest. The leaf is visible at the bottom of the frame, the character is gazing at it longingly.
```

### EATING / 풀 들고 있는 AWAKE 상태

#### S6 — eating.png
> 풀을 먹는 중. **데이지는 오른쪽 귀에 꽂힘** (양손 풀 먹기 위해)
```
MATCH the attached s1-awake-default.png in color tone, lighting, character size, fur texture, daisy color EXACTLY. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes happy and content (closed in bliss), pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, front view, sitting down, happily munching on a small green leaf held in BOTH hands near the mouth, satisfied chewing expression, small green leaf sticking out of mouth. CRITICAL: a small white daisy flower with yellow center and green stem is TUCKED BEHIND THE RIGHT EAR (visible from front view, the daisy peeks out by the right ear, NOT in hands), since both hands are busy eating the grass.
```

#### S6b — awake-with-grass.png  ⭐ held_item 룰 (풀 먹은 후 ~ 자정까지)
> 풀 한 손에 쥐고 다님. **데이지는 오른쪽 귀에 그대로** (다음 자정에 데이지 손으로 복귀)
```
MATCH the attached s1-awake-default.png in color tone, lighting, character size, fur texture, daisy color EXACTLY. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, perfectly symmetrical front view, standing upright, gentle happy smile, ears perked up, content expression. POSE: holding a small fresh green leaf (with visible vein lines) in RIGHT HAND instead of the daisy. CRITICAL: the white daisy flower is TUCKED BEHIND THE RIGHT EAR (clearly visible peeking out from behind the right ear), NOT in any hand. The character is carrying the grass leaf in hand while the daisy stays decoratively in the ear until midnight.
```

### CHARGING_IDLE 상태 (충전 중 멍때리기)

#### S7 — charging-idle.png
> 앉아서 멍때리는 포즈
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes with soft unfocused gaze, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand resting on lap, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, sitting down on the ground with legs spread forward, relaxed slouchy posture, peaceful blank stare, zoning out expression, content and calm
```

### NAPPING 상태 (낮잠)

#### S8 — napping.png
> 눈 감고 자는 포즈 (데이지 꼭 쥔 채)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, eyes fully closed sleeping peacefully, pink blush cheeks, light beige fur, clutching a small white daisy flower with yellow center and green stem tightly in right hand even while sleeping, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, curled up sitting position, head tilted to one side, peaceful sleeping face with gentle smile, cozy and warm feeling
```

### STRETCHING 상태 (기지개)

#### S9 — stretching.png
> 기지개 켜는 포즈. **양손 사용 → 꽃은 오른쪽 귀에 꽂혀 있음**
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes squinting half-open, pink blush cheeks, light beige fur, a small white daisy flower with yellow center and green stem tucked behind right ear, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, standing up with both arms raised high above head in a big stretch, yawning slightly, just woke up expression, eyes half open and sleepy
```

### DROWSY 상태 (졸림 — D4 명시)

#### S10 — drowsy.png
> 졸린 표정. 눈 50% 감김 + 살짝 흔들 (애니메이션). 꽃은 손에
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes droopy and exactly half-closed at 50%, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, standing but slightly swaying, sleepy droopy expression, tired but still cute, mouth in a small relaxed smile
```

### YAWNING 상태 (랜덤 — Step 6 MVP)

#### S13 — yawning.png
> 하품 단독 포즈. **양손이 위로 가서 꽃은 오른쪽 귀에 꽂혀 있음**
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes squeezed shut from yawning, pink blush cheeks, light beige fur, a small white daisy flower with yellow center and green stem tucked behind right ear, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, standing upright, big wide yawn with mouth open in a round O shape, both hands raised slightly near face in yawning gesture, eyes scrunched closed
```

### FIRST_GREETING 상태 (최초 1회 — Step 6 MVP)

#### .png
> 앱 최초 실행 시 1회만, 1단계. **소심한 첫 인사 — 양손에 데이지를 가슴 앞에 들고**
```
MATCH the attached s1-awake-default.png in color tone, lighting, character size, fur texture, daisy color EXACTLY. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes wide open and bright with highlight sparkle, pink blush cheeks rosy, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, front view, standing upright. POSE: both hands together holding a LARGE prominent white daisy flower with bright yellow center and green stem at chest level, daisy is visibly bigger than usual (eye-catching focal point of the image), shy nervous-happy smile, slight bow of head, "안녕하세요" first meeting moment, slightly bashful body language. The daisy should be SIZED 1.5x larger than typical for visual emphasis.
```

#### S11a — first-greeting-show.png  ⭐ 친구 훅 2단계
> S11 다음 시퀀스. **데이지를 카메라 쪽으로 더 크게 내밀어 보임 — "이거 봐!"**
```
MATCH the attached s11-first-greeting.png in character pose, color tone, and daisy appearance. Same character continuing the greeting sequence — moved one moment forward. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, light beige fur, pink blush cheeks rosy, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, front view standing upright. POSE: both hands together EXTENDED FORWARD toward the camera/viewer, holding the SAME large white daisy flower now MUCH CLOSER to the camera as if offering it forward, daisy is in the FOREGROUND of the image (occupying noticeably more frame space than in the previous frame), body slightly leaned forward in offering gesture, eyes squeezed into happy crescent moon shapes (closed-eye smile from joy of giving), proud-happy small closed-mouth smile, head slightly tilted, "이거 너 줄게!" gift-giving moment. The daisy is the visual centerpiece — closer to the viewer than the character's face.
```

### SURPRISED 상태 (이름 변경 반응 — Step 6 MVP)

#### S12 — surprised.png
> 이름이 바뀌는 순간 깜짝. **양손 입가로 → 꽃은 오른쪽 귀에**
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes wide open in surprise, pupils dilated big, pink blush cheeks, light beige fur, a small white daisy flower with yellow center and green stem tucked behind right ear, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, standing upright, both hands raised near mouth in a startled "oh!" gesture, mouth in a small round O shape, ears perked up sharply, cute startled but not scared expression
```

### NAME DIALOG 에러 (D7 — 이름 입력 검증)

#### S14 — name-dialog-tilt.png
> 이름 입력이 빈값/초과 시 머리 갸우뚱 (에러 시각화)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, round chubby body, tiny limbs, small bead eyes blinking confused, pink blush cheeks, light beige fur, holding a small white daisy flower with yellow center and green stem in right hand, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline around the character silhouette, front view, standing upright, head clearly tilted to one side at 30 degrees in a "huh?" gesture, slightly puzzled expression, small question-mark feeling, gentle smile still
```

---

## 2.5 애니메이션 키프레임 (Wave 2A~C, 2026-04-30 추가)

> 사용자 인사이트: "CSS 진폭으로는 자연스러움 한계 — 다중 PNG 교체가 픽사 2D 톤의 본질".
> 호흡 + 걷기 + 표정 디테일을 **PNG 키프레임 교체**로 표현.

### 호흡 키프레임 (Wave 2A — 3장)

> ⚠️ **핵심 룰**: 베이스 PNG (s1-awake-default.png 등)를 **반드시 같이 업로드**. "프레임 단위 애니메이션"으로 강제.
> 결과 검증: 베이스 PNG 옆에 놓고 비교 — 손/얼굴/데이지 위치가 100% 같고 **배만 부풀음**.

#### S1b — s1b-awake-breathe.png  ⭐ 우선순위 1
> S1과 정확히 같은 프레임 + 배만 부푼 들숨. (베이스 PNG 같이 업로드 필수)
```
EXACT FRAME-BY-FRAME ANIMATION KEYFRAME. This is frame 2 of a breathing cycle. Frame 1 is the attached base reference image (s1-awake-default). Frame 2 MUST be IDENTICAL to frame 1 in EVERY visual detail EXCEPT belly expansion. Specifically: SAME exact head position (do not tilt rotate or move), SAME exact facial expression (identical mouth shape eye shape smile angle), SAME exact hand positions (left hand exactly where it is, right hand holding daisy at exactly same height and angle), SAME exact daisy position and orientation, SAME exact ear position and angle, SAME exact body orientation and stance, SAME exact foot placement. The ONLY difference: tummy and chest area expanded outward by approximately 8-10 percent as if mid-inhale. Imagine two consecutive frames of a hand-drawn animation showing only the act of breathing in. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, holding a small white daisy flower with yellow center and green stem in right hand, small bead eyes with highlight sparkle, pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, perfectly symmetrical front view, gentle happy smile, ears slightly perked up
```

#### S7b — s7b-charging-breathe.png
> S7과 정확히 같은 프레임 + 배만 부푼 들숨. (s7-charging-idle.png 같이 업로드 필수)
```
EXACT FRAME-BY-FRAME ANIMATION KEYFRAME. This is frame 2 of a breathing cycle for the sitting/charging pose. Frame 1 is the attached base reference image (s7-charging-idle). Frame 2 MUST be IDENTICAL to frame 1 in EVERY visual detail EXCEPT belly expansion. Specifically: SAME exact sitting position (legs spread forward, slouchy relaxed posture), SAME exact head tilt and gaze direction, SAME exact facial expression (peaceful blank unfocused stare, soft smile), SAME exact hand positions (right hand holding daisy resting on lap exactly where it is, left hand exactly where it is), SAME exact daisy position and orientation, SAME exact ear position. The ONLY difference: tummy area expanded outward by approximately 8-10 percent as if mid-inhale of a relaxed breath. Imagine two consecutive video frames of the same character breathing while sitting. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, sitting down on ground with legs spread forward, relaxed slouchy posture, holding a small white daisy flower with yellow center and green stem in right hand resting on lap, small bead eyes soft and unfocused, pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, front view
```

#### S8b — s8b-napping-breathe.png  ⭐ 쎄근쎄근
> S8과 정확히 같은 프레임 + 배가 더 깊이 부푼 들숨. (s8-napping.png 같이 업로드 필수)
```
EXACT FRAME-BY-FRAME ANIMATION KEYFRAME. This is frame 2 of a deep sleep breathing cycle. Frame 1 is the attached base reference image (s8-napping). Frame 2 MUST be IDENTICAL to frame 1 in EVERY visual detail EXCEPT belly expansion. Specifically: SAME exact curled up sitting position, SAME exact head tilt to one side, SAME exact eyes fully closed, SAME exact peaceful sleeping smile, SAME exact hands clutching daisy position (right hand holding daisy tightly exactly where it is), SAME exact daisy position and orientation, SAME exact ear position, SAME exact foot position. The ONLY difference: tummy area expanded outward by approximately 10-12 percent as if mid-inhale of a deep sleeping breath (slightly deeper than awake breathing). Imagine two consecutive video frames of the same sleeping character breathing peacefully. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, eyes closed sleeping peacefully, curled up sitting position with head tilted, peaceful sleeping face with gentle smile, clutching a small white daisy flower in right hand, pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, front view, cozy and warm
```

### 표정 디테일 (Wave 2B — 2장)

#### S2-half — s2-awake-blink-half.png
> 깜빡임 부드럽게 — 반쯤 감은 눈. 시퀀스: S1 → S2-half → S2 → S2-half → S1
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, IDENTICAL POSE to awake default but EYES HALF CLOSED at exactly 50% — eyelids drooping halfway down covering top half of bead eyes, mid-blink moment captured, gentle smile maintained, all other elements identical to standard awake pose, holding a small white daisy flower with yellow center and green stem in right hand, pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, perfectly symmetrical front view, ears perked up, peaceful expression
```

#### S1c — s1c-awake-eyes-soft.png
> 잔잔한 눈 웃음 — 눈만 가늘게 (입은 작게). S2(happy-blink)와 다른 결.
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, IDENTICAL POSE to awake default but EYES SOFTLY NARROWED into gentle crescent moon shapes (closed-eye smile / 눈 웃음), eyes squinted in warm contented expression, mouth in a small soft closed smile (not open laugh), peaceful loving expression, all other elements identical to base pose, holding a small white daisy flower with yellow center and green stem in right hand, pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, perfectly symmetrical front view, ears perked up
```

### 걷기 시퀀스 (Wave 2C — 4장) 🚶 NEW

> 측면 포즈 (오른쪽 향함) + 데이지 오른손에. W1→W2→W3→W4→W1 사이클 0.4초씩 = 1.6초로 한 걸음.
> 데이지 위치는 4장 모두 동일 — 코드로 합성하지 말고 PNG에 통합 (걷는 동안 흔들리면 어색).

#### W1 — w1-walking-step1.png
> 측면. **카메라에서 멀리 있는(뒤쪽) 발이 앞으로 내딛음** — 자연스러운 걷기의 한 프레임
```
EXACT WALKING POSE FRAME 1. Side view profile of character facing rightward (character body points to the right side of the frame, so the viewer/camera sees the LEFT SIDE of the character). DETAILED LEG POSITIONS: the BACK LEG (the leg that is FAR from the camera, on the side away from the viewer) is STEPPED CLEARLY FORWARD IN FRONT of the body in mid-stride walking position - this back leg should be VISIBLY PROTRUDING FORWARD at an angle as if mid-step. The FRONT LEG (the leg that is CLOSER to the camera/viewer, on the near side) is positioned BEHIND, planted firmly on the ground supporting the body weight. Both legs MUST BE IN CLEARLY DIFFERENT POSITIONS — one forward, one back — showing obvious walking motion. Body leans slightly forward in stride direction. 3D render, soft plush toy style, fluffy fur texture, cute quokka character, full body side view, small bead eyes with highlight sparkle, gentle happy smile, ears slightly bouncy from movement, holding a small white daisy flower with yellow center and green stem (clearly visible from this side angle), pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, dynamic walking pose, mid-stride
```

#### W2 — w2-walking-step2.png
> 측면, 양발 모음 (중간 단계, 오른발 다음)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, side view facing right (profile), full body, mid-step pose with FEET TOGETHER and body upright, transition between strides, small bead eyes with highlight sparkle, gentle happy smile, ears upright, holding a small white daisy flower with yellow center and green stem in right hand (visible from side), pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, neutral standing transition pose
```

#### W3 — w3-walking-step3.png
> 측면. **W1의 반대 스트라이드** — 다른 발이 앞으로. (W1을 베이스로 같이 업로드 필수)
```
WALKING ANIMATION FRAME 3 — alternate stride. Same character and same camera angle as the attached W1 reference image, but showing the OPPOSITE walking phase. 

LEG POSITIONS (this is the key difference from W1): the leg that was BEHIND in W1 is now STEPPED FORWARD, and the leg that was forward in W1 is now BEHIND. Show the alternating stride clearly. 

If the back leg is partially behind the body silhouette, that's okay — show enough of it protruding forward to make the alternating stride visible. The character has shifted weight to the newly-planted leg. Slight forward lean. Mid-stride moment between footfalls.

Same as W1 in every other way: same character pose orientation, same daisy position in right hand, same facial expression, same body proportions, same side view facing rightward. ONLY the leg positions are swapped.

3D render, soft plush toy style, fluffy fur texture, cute quokka character, full body side view facing right, small bead eyes with highlight sparkle, gentle happy smile, ears slightly bouncy, holding a small white daisy flower with yellow center and green stem visible from side, pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline.
```

#### W4 — w4-walking-step4.png
> 측면, 양발 모음 (중간 단계, 왼발 다음 — W2와 비슷하지만 다음 사이클 진입)
```
3D render, soft plush toy style, fluffy fur texture, cute quokka character, side view facing right (profile), full body, mid-step pose with FEET TOGETHER and body upright, transition between strides preparing for next step, small bead eyes with highlight sparkle, gentle happy smile, ears upright, holding a small white daisy flower with yellow center and green stem in right hand (visible from side), pink blush cheeks, light beige fur, pastel colors, studio lighting, white background, cute kawaii aesthetic, subtle 1px dark outline, neutral standing transition pose
```

---

## 3. 소품 & 배경 (3장)

### P1 — prop-grass.png
> 풀/잔디 (먹이용)
```
3D render, soft plush toy style. A small cute tuft of green grass, simple and minimal, 3-4 blades of bright green grass, soft felt texture, looks like a plush toy grass patch. Pastel green color, studio lighting, white background.
```

### P2 — prop-leaf.png
> 나뭇잎 (먹이 아이템 / `held_item`이 풀일 때 손에 든 모습)
```
3D render, soft plush toy style. A single small cute green leaf, simple oval shape, soft felt/plush texture, bright pastel green with slightly darker vein lines, looks like a toy leaf. Studio lighting, white background.
```

### P3 — prop-daisy.png  ⭐ 분리 합성용 (Day 2)
> 데이지 꽃 단독. 캐릭터 본체에서 분리해서 손/귀에 코드로 합성. 손↔귀 전환, 풀↔데이지 swap의 핵심 자산.
```
3D render, soft plush toy style. A single small cute white daisy flower with bright yellow center and short green stem, simple and minimal, plump 5-petal rounded shape, soft felt/plush texture, looks like a toy flower prop. Pastel colors, studio lighting, transparent or white background. Standalone prop image for compositing onto a character.
```

### BG1 — bg-grassfield.png
> 충전 중 풀밭 배경. **D1: 2초 ease-out fade-in, 최대 60% 투명도, 따뜻한 녹색 ~#c8e4b5**
```
3D render, soft plush toy style. A small cozy grass field patch, round/oval shape, bright green fluffy grass in warm green tone around hex color #c8e4b5, with tiny pastel flowers scattered (white, pink, yellow), soft felt texture like a miniature garden. Seen from slightly above, circular composition, warm gentle lighting, transparent or white background. This will appear behind a small character as a resting spot.
```

---

## 4. 이펙트

### E_petals — effect-daisy-petals.png  ⭐ HAPPY 시그니처 (D2)
> HAPPY 상태에서 위로 떠오르는 데이지 꽃잎. **하트 ❌ / sparkle ❌ — 데이지 꽃잎이 시그니처**
```
3D render, soft plush toy style. A cluster of 3 to 5 small white daisy petals with subtle yellow tint near the base, soft felt/plush texture, gently floating upward with slight rotation, scattered composition, pastel colors, studio lighting, transparent or white background. Simple and minimal, designed to overlay on a character.
```

### E2 — effect-zzz.png
> ZZZ 이펙트 (NAPPING 상태)
```
3D render, soft plush toy style. The letters "ZZZ" in a cute bubbly font, pastel blue/lavender color, soft puffy 3D text, plush felt texture, getting smaller from left to right (Z z z), studio lighting, white background.
```

> **이전 E1 (heart) / E3 (sparkle) 삭제됨** — plan.md D2 ("하트 이모지 ❌, 데이지 꽃잎이 시그니처") 정합성 위반.

---

## 체크리스트

**MVP 필수 (19장):**
- [ ] REF-1 ref-front.png (정면)
- [ ] REF-2 ref-side.png (옆면)
- [ ] REF-3 ref-threequarter.png (3/4)
- [ ] REF-4 ref-back.png (뒷면)
- [ ] S1 awake-default.png (기본 / fallback PNG 겸용)
- [ ] S2 awake-blink.png (눈 깜빡)
- [ ] S3 awake-eartwitch.png (귀 쫑긋)
- [ ] S4 happy-bounce.png (기쁨 점프, **꽃잎 이펙트**)
- [ ] S4b happy-daisy-show.png (**친구 훅** — 데이지 내밀기)
- [ ] S5 idle-hungry.png (먹고 싶다)
- [ ] S6 eating.png (먹는 중)
- [ ] S6b awake-with-grass.png (**풀 들고 다니기**)
- [ ] S7 charging-idle.png (앉아 멍때리기)
- [ ] S8 napping.png (낮잠)
- [ ] S9 stretching.png (기지개, **꽃은 귀에**)
- [ ] S10 drowsy.png (졸림 50%)
- [ ] S11 first-greeting.png (**최초 1회 인사**)
- [ ] S12 surprised.png (**개명 반응**, 꽃은 귀에)
- [ ] S13 yawning.png (**하품**, 꽃은 귀에)
- [ ] S14 name-dialog-tilt.png (이름 에러 갸우뚱)
- [ ] BG1 bg-grassfield.png (풀밭 배경)
- [ ] E_petals effect-daisy-petals.png (**HAPPY 시그니처**)

**선택 (3장):**
- [ ] P1 prop-grass.png (풀)
- [ ] P2 prop-leaf.png (나뭇잎)
- [ ] E2 effect-zzz.png (ZZZ)

**분리 합성 자산 (Day 2 추가):**
- [ ] **P3 prop-daisy.png** — 데이지 단독 (손/귀에 코드로 합성)

**총: MVP 필수 22장 + P3 분리 자산 1장 + 선택 3장 = 최대 26장**

> **참고:** 3D 트랙(Day 3 GO 시) S-시리즈는 본체 모델 + 애니메이션으로 처리되므로, S1만 fallback PNG로 필수. 나머지는 컨셉 레퍼런스. Rive 2D pivot 시에만 S-시리즈 모든 PNG 필요.

---

## 뽑는 순서 추천 (2D 트랙 — Wave 분산)

> 한 번에 16장 다 뽑지 말 것. **Wave별로 6~8장씩** 뽑아 톤 일관성 유지.
> 각 Wave 첫 1~2장 검증 후 다음 작업 진행.

### Wave 0: REF (Day 1) — ✅ 완료
1. ✅ REF-1~4 (정면/측면/3⁄4/뒷면)

### Wave 1: 핵심 6장 + P3 (Day 3) — ✅ 완료
2. ✅ S1 awake-default — 모든 idle의 기준
3. ✅ S2 awake-blink — 눈 깜빡 (실제론 happy-blink 톤)
4. ✅ S7 charging-idle — 충전 중 멍때리기
5. ✅ S8 napping — 낮잠
6. ✅ P3 prop-daisy — 풀↔데이지 swap용
7. 🟡 S3 awake-eartwitch — Wave 2에서 재시도

### Wave 2A: 호흡 키프레임 3장 — ⭐ 다음 우선순위 (살아있는 룸메이트 핵심)

8. **S1b awake-breathe** — AWAKE 배 부푼 들숨
9. **S7b charging-breathe** — CHARGING 배 부푼 들숨
10. **S8b napping-breathe** — NAPPING 배 더 부푼 들숨 (쎄근쎄근)

### Wave 2B: 표정 디테일 2장

11. **S2-half awake-blink-half** — 깜빡 부드러운 중간 프레임
12. **S1c awake-eyes-soft** — 잔잔한 눈 웃음

### Wave 2C: 걷기 시퀀스 4장 🚶 NEW (MVP 승격)

13. **W1 walking-step1** — 측면, 오른발 앞
14. **W2 walking-step2** — 측면, 양발 모음
15. **W3 walking-step3** — 측면, 왼발 앞
16. **W4 walking-step4** — 측면, 양발 모음 (다음 사이클 진입)

### Wave 2D: HAPPY 인터랙션 3장

17. **S4 happy-bounce** — 통통 튀기 + 꽃잎
18. **S4b happy-daisy-show** ⭐ 친구 훅의 핵심
19. **E_petals daisy-petals** — HAPPY 시그니처 이펙트

### Wave 2E: NAPPING + 양손 룰 + 배경 5장

20. **E2 effect-zzz** — NAPPING ZZZ
21. **BG1 grassfield** — 풀밭 배경
22. **S3 재시도 awake-eartwitch** — Wave 1 미세 재시도
23. **S9 stretching** — 양손, **꽃은 귀에**
24. **S11 first-greeting** — 최초 1회 인사
25. **S12 surprised** — 양손, **꽃은 귀에**

### Wave 3: 행동 풍부함 + 풀 먹이기 9장 — Day 10 전 작업

26. S5 idle-hungry
27. S6 eating
28. S6b awake-with-grass — 풀 들고 다니는 AWAKE
29. S10 drowsy
30. S13 yawning — **꽃은 귀에**
31. S14 name-dialog-tilt
32. P1 prop-grass
33. P2 prop-leaf
34. (E_petals은 Wave 2D로 이동, P3는 Wave 1 완료)

### 🌟 Wave 톤 일관성 가드

- 각 Wave는 **같은 세션에서** 뽑기 (나노바나나 톤 미세 차이 방지)
- 매 PNG 뽑을 때 [ref-front.png](images/ref-front.png) + [qqref.png](qqref.png) 같이 업로드 (지원되면)
- Wave 시작 시 첫 1~2장 검증 → 톤 OK 하면 나머지 진행

## 팁

- **데이지 꽃 일관성**: 평소엔 오른손, 양손 쓰는 포즈(STRETCHING, YAWNING, SURPRISED, FIRST_GREETING)는 **오른쪽 귀에 꽂은 모습**. 빠지면 다시 뽑기
- **풀일 때는 데이지 빠짐**: S6b는 데이지 없이 **풀만** 들고 있어야 함 (`held_item` 룰)
- **200×200px로 표시됨**: 너무 디테일한 건 안 보임. 실루엣이 뚜렷한 게 중요
- **배경 제거 필수**: 모든 캐릭터/소품은 투명 배경이어야 데스크탑 위에 자연스러움
- **다크모드 대비**: 1px 어두운 아웃라인이 들어갔는지 확인 (D9)
- **이펙트는 데이지 꽃잎만**: 하트/sparkle은 plan.md D2가 명시적으로 금지
