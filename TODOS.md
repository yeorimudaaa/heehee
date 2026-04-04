# 희희 (Heehee) — TODOs

## 구현 시 바로 하기

### GLTFLoader 에러 핸들링
**What:** GLTFLoader 실패 시 fallback 처리 추가
**Why:** 모델 파일 없거나 경로 오류 시 희희가 침묵으로 사라짐. 원인을 알 수가 없어 디버깅 시간 낭비.
**Where to start:** `ThreeJSPetRenderer.start()` 내 `GLTFLoader.load()` error callback
**How:**
```ts
loader.load('heehee://models/heehee.glb',
  (gltf) => { /* 성공 */ },
  undefined,
  (err) => {
    console.error('[희희] GLTFLoader 실패:', err)
    // fallback: SpritePetRenderer로 강제 전환
    this.onLoadError?.()
  }
)
```

---

## 배포 시 하기

### macOS Notarization 설정
**What:** Apple Developer 등록 + electron-builder notarize 설정
**Why:** 없으면 모든 macOS 사용자에게 Gatekeeper 차단. 수동 우회 설명 없이는 설치 불가.
**Steps:**
1. Apple Developer Program 등록: https://developer.apple.com/programs/ ($99/년)
2. `npm install --save-dev @electron/notarize`
3. `electron-builder.config.js`에 afterSign hook 추가:
```js
const { notarize } = require('@electron/notarize')
exports.default = async (context) => {
  await notarize({
    tool: 'notarytool',
    appPath: `${context.appOutDir}/${context.packager.appInfo.productFilename}.app`,
    appleId: process.env.APPLE_ID,
    appleIdPassword: process.env.APPLE_APP_SPECIFIC_PASSWORD,
    teamId: process.env.APPLE_TEAM_ID
  })
}
```
4. 코드사이닝: `CSC_LINK` + `CSC_KEY_PASSWORD` 환경변수
**Depends on:** Apple Developer 계정 먼저

---

## 나중에 (Phase 2)

- WALKING_EDGE 상태 구현 (모서리 걷기)
- 랜덤 이벤트 (친구 방문, 나비)
- 시간대 반응 (밤 11시 졸린 표정)
- 카메라/마이크 인식
- 날씨/계절 반응
- 코딩 상태 인식 (IDE 감지)
