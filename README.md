# 🌼 희희 (Heehee)

> 맥북 코너에 사는 데이지 쿼카 룸메이트
> 같이 호흡하고, 같이 잠들고, 가끔 외출했다 돌아와요

---

## 📥 다운로드

[Releases 페이지](https://github.com/yeorimudaaa/heehee/releases) → 최신 `Heehee.dmg`

## 📦 설치

1. **Heehee.dmg** 더블클릭 → 마운트
2. 창 안의 **Heehee.app**을 **Applications 폴더로 드래그**
3. Applications에서 **우클릭 → "열기"** → 경고창 **"열기"** (첫 1회만)
4. 끝! 이후엔 일반 앱처럼 더블클릭

## 🌸 함께 사는 법

### 평소 모습
- 맥북 우하단에 등장 (드래그로 이동, 위치 기억)
- 호흡 + 깜빡 + 가끔 눈웃음 / 하품
- 모든 Spaces / Mission Control 따라옴

### 인터랙션
| 동작 | 반응 |
|---|---|
| **클릭** | 데이지 자랑 + 점프 + 꽃잎 🌸 |
| **드래그** | 자리 옮기기 |
| **마우스 가까이** | 휘리릭 도망 (1초) |
| **충전 꽂기** | 풀밭 + 멍 → 5분 후 잠 💤 |
| **충전 빼기** | 졸린 듯 깨어나며 기지개 |
| **23시~6시** | 졸림 모드 (DROWSY) — 호흡 깊어짐, 하품 자주 |

### 트레이 메뉴 (우상단 단색 데이지)
- 🌼 새로운 이름 짓기
- 🌿 풀 주기 (자정 지나면 데이지 복귀)
- 🚶 잠깐 외출 보내기 (1시간) / 🏠 돌아와 희희
- 📖 사용법
- ℹ️ 희희 정보 / 버전
- 👋 다음에 또 봐 (종료)

## 🔒 프라이버시 약속

희희는:
- ❌ **카메라 / 마이크 / 스크린 / 키보드 입력 전혀 읽지 않음**
- ✅ 유일하게 보는 것: macOS 충전 상태
- ❌ 외부 서버로 데이터 전송 X
- ✅ 모든 데이터는 로컬 (이름, 위치, 풀 먹은 날짜)

> v0.2부터 GitHub Releases API에서 새 버전 체크만 (다른 정보 송수신 X)

## 🛠️ 기술 스택

- [Tauri 2](https://tauri.app/) (Rust + WKWebView)
- HTML / CSS / Vanilla JS (자산은 PNG 키프레임)
- macOS 전용 (Apple Silicon / Intel universal)
- 자산 ~30장 (S 시리즈 + 걷기 + 배경 + 이펙트)
- 번들 크기: ~10MB (.dmg)

### 로컬 개발
```bash
cd app
bun install                # 또는 npm install
bun run tauri dev          # 개발 모드 (Rust 변경 시 자동 재빌드)
bun run tauri build        # 릴리스 빌드 (.app + .dmg)
```

## 🎁 만든 이

**최여름** ([@yeorimudaaa](https://github.com/yeorimudaaa))
친구들에게 주는 선물 — 매일 같이 있어주는 룸메이트

🌼 항상 웃고 있을 테니, 가끔 나를 봐 — 희희
