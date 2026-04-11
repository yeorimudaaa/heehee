# 희희 (Heehee) — TODOs

## 구현 시 바로 하기

### GLTFLoader 에러 핸들링
**What:** GLTFLoader 실패 시 fallback 처리 추가
**Why:** 모델 파일 없거나 경로 오류 시 희희가 침묵으로 사라짐. 원인을 알 수가 없어 디버깅 시간 낭비.
**Where to start:** `ThreeJSPetRenderer.start()` 내 `GLTFLoader.load()` error callback
**How:**
```ts
// Tauri의 asset protocol로 모델 로드 (asset:// 또는 fetch convertFileSrc)
import { convertFileSrc } from '@tauri-apps/api/core'
const modelUrl = convertFileSrc('models/heehee.glb')

loader.load(modelUrl,
  (gltf) => { /* 성공 */ },
  undefined,
  (err) => {
    console.error('[희희] GLTFLoader 실패:', err)
    // fallback: 정적 PNG 쿼카 이미지로 전환
    this.showStaticFallback()
  }
)
```

---

## 배포 시 하기

### macOS Notarization 설정 (Tauri)
**What:** Apple Developer 등록 + Tauri codesign + notarytool 설정
**Why:** 없으면 모든 macOS 사용자에게 Gatekeeper 차단. 수동 우회 설명 없이는 설치 불가.
**Steps:**
1. Apple Developer Program 등록: https://developer.apple.com/programs/ ($99/년)
2. Apple Developer ID Application 인증서 생성 + 키체인 등록
3. `tauri.conf.json`에 macOS signing 설정:
```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
        "providerShortName": "TEAM_ID",
        "entitlements": "./entitlements.plist"
      }
    }
  }
}
```
4. App-specific password 생성 (https://appleid.apple.com/account/manage)
5. 빌드 후 `notarytool`로 직접 노타라이즈:
```bash
xcrun notarytool submit target/release/bundle/dmg/heehee.dmg \
  --apple-id "$APPLE_ID" \
  --password "$APPLE_APP_SPECIFIC_PASSWORD" \
  --team-id "$APPLE_TEAM_ID" \
  --wait
xcrun stapler staple target/release/bundle/dmg/heehee.dmg
```
6. CI에서 자동화하려면 환경변수: `APPLE_ID`, `APPLE_APP_SPECIFIC_PASSWORD`, `APPLE_TEAM_ID`, `APPLE_SIGNING_IDENTITY`
**Depends on:** Apple Developer 계정 먼저
**Note:** Tauri는 Electron과 달리 별도 plugin 없이 `cargo tauri build` + 수동 notarytool 조합이 가장 단순.

---

## 나중에 (Phase 2)

- WALKING_EDGE 상태 구현 (모서리 걷기)
- 랜덤 이벤트 (친구 방문, 나비)
- 시간대 반응 (밤 11시 졸린 표정)
- 카메라/마이크 인식
- 날씨/계절 반응
- 코딩 상태 인식 (IDE 감지)
