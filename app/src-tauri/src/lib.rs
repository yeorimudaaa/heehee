use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// JS가 모달 열고 닫을 때 호출 — 모달 열린 동안은 클릭 패스스루 OFF
#[tauri::command]
fn set_modal_open(state: tauri::State<Arc<AtomicBool>>, open: bool) {
    state.store(open, Ordering::SeqCst);
}

// 투명 layer 강제 재적용 — 회색창 방지 (resize, focus 등 후에 호출)
#[tauri::command]
fn force_transparent(app: tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::NSColor;
        use cocoa::base::{id, nil, NO, YES};
        use objc::{msg_send, sel, sel_impl};

        let window = app.get_webview_window("main").ok_or("no main window")?;
        unsafe {
            let ns_window = window.ns_window().map_err(|e| e.to_string())? as id;
            let clear_color = NSColor::clearColor(nil);
            let content_view: id = msg_send![ns_window, contentView];
            let _: () = msg_send![content_view, setWantsLayer: YES];
            let layer: id = msg_send![content_view, layer];
            if !layer.is_null() {
                let cg_clear: id = msg_send![clear_color, CGColor];
                let _: () = msg_send![layer, setBackgroundColor: cg_clear];
            }
            // 모든 서브뷰(웹뷰)도 투명 강제
            let subviews: id = msg_send![content_view, subviews];
            let count: usize = msg_send![subviews, count];
            for i in 0..count {
                let subview: id = msg_send![subviews, objectAtIndex: i];
                let _: () = msg_send![subview, setWantsLayer: YES];
                let sub_layer: id = msg_send![subview, layer];
                if !sub_layer.is_null() {
                    let cg_clear2: id = msg_send![clear_color, CGColor];
                    let _: () = msg_send![sub_layer, setBackgroundColor: cg_clear2];
                }
                let responds: bool = msg_send![subview, respondsToSelector: sel!(setDrawsBackground:)];
                if responds {
                    let _: () = msg_send![subview, setDrawsBackground: NO];
                }
                // WKWebView이면 setOpaque:NO 도 강제
                let responds_opaque: bool = msg_send![subview, respondsToSelector: sel!(setOpaque:)];
                if responds_opaque {
                    let _: () = msg_send![subview, setOpaque: NO];
                }
            }
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
    }
    Ok(())
}

// 모달 열릴 때 창 위로 늘리기 — Cocoa NSWindow.setFrame으로 원자적 변경
// macOS bottom-left 원점이라 origin 그대로 두고 height만 늘리면 자동으로 위로 자람, 바닥 고정
// extra_pts: 늘릴 logical points (기본 220 = name/info 모달용, help 모달은 더 큰 값 사용)
#[tauri::command]
fn resize_for_modal(app: tauri::AppHandle, expand: bool, extra_pts: Option<f64>) -> Result<(), String> {
    let extra_pts = extra_pts.unwrap_or(220.0);
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::NSColor;
        use cocoa::base::{id, nil, YES};
        use cocoa::foundation::{NSRect, NSSize};
        use objc::{msg_send, sel, sel_impl};

        let window = app.get_webview_window("main").ok_or("no main window")?;
        unsafe {
            let ns_window = window.ns_window().map_err(|e| e.to_string())? as id;

            // maxSize 일시 해제 (setFrame이 constraint에 막히지 않게)
            let huge: NSSize = NSSize::new(10000.0, 10000.0);
            let _: () = msg_send![ns_window, setMaxSize: huge];

            let current_frame: NSRect = msg_send![ns_window, frame];
            let new_height = if expand {
                current_frame.size.height + extra_pts
            } else {
                (current_frame.size.height - extra_pts).max(1.0)
            };
            let new_frame = NSRect::new(
                current_frame.origin,
                NSSize::new(current_frame.size.width, new_height),
            );
            let _: () = msg_send![ns_window, setFrame: new_frame display: YES];

            // 투명 layer 재적용 — content view + 모든 서브뷰(webview 포함)
            let clear_color = NSColor::clearColor(nil);
            let content_view: id = msg_send![ns_window, contentView];
            let _: () = msg_send![content_view, setWantsLayer: YES];
            let layer: id = msg_send![content_view, layer];
            if !layer.is_null() {
                let cg_clear: id = msg_send![clear_color, CGColor];
                let _: () = msg_send![layer, setBackgroundColor: cg_clear];
            }
            // 서브뷰 (webview) 도 투명 강제
            let subviews: id = msg_send![content_view, subviews];
            let count: usize = msg_send![subviews, count];
            for i in 0..count {
                let subview: id = msg_send![subviews, objectAtIndex: i];
                let _: () = msg_send![subview, setWantsLayer: YES];
                let sub_layer: id = msg_send![subview, layer];
                if !sub_layer.is_null() {
                    let cg2: id = msg_send![clear_color, CGColor];
                    let _: () = msg_send![sub_layer, setBackgroundColor: cg2];
                }
            }

            // maxSize 복구
            let max_normal: NSSize = NSSize::new(240.0, 800.0);
            let _: () = msg_send![ns_window, setMaxSize: max_normal];
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (app, expand, extra_pts);
    }
    Ok(())
}

fn check_charging() -> bool {
    std::process::Command::new("pmset")
        .args(["-g", "batt"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.contains("AC Power"))
        .unwrap_or(false)
}

// 트레이 메뉴를 새로 빌드해서 적용 (이름/상태 동적 업데이트용)
fn build_tray_menu(
    app: &tauri::AppHandle,
    name: &str,
    status: &str,
) -> tauri::Result<Menu<tauri::Wry>> {
    let name_label = format!("🌼 {}", name);
    let status_label = format!("지금: {}", status);

    let name_item = MenuItem::with_id(app, "_name", &name_label, false, None::<&str>)?;
    let status_item = MenuItem::with_id(app, "_status", &status_label, false, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let rename = MenuItem::with_id(app, "rename", "🌼 새로운 이름 짓기", true, None::<&str>)?;
    let feed = MenuItem::with_id(app, "feed", "🌿 풀 주기", true, None::<&str>)?;
    let sleep_in = MenuItem::with_id(app, "sleep_in", "🚶 잠깐 외출 보내기 (1시간)", true, None::<&str>)?;
    let wake_now = MenuItem::with_id(app, "wake_now", "🏠 돌아와 희희", true, None::<&str>)?;
    let help = MenuItem::with_id(app, "help", "📖 사용법", true, None::<&str>)?;
    let info = MenuItem::with_id(app, "info", "ℹ️ 희희 정보 / 버전", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "👋 다음에 또 봐", true, None::<&str>)?;

    Menu::with_items(
        app,
        &[
            &name_item, &status_item, &sep1, &rename, &feed, &sleep_in, &wake_now, &help, &info, &sep2, &quit,
        ],
    )
}

// JS에서 호출 — 메뉴 갱신
#[tauri::command]
fn update_tray_menu(
    app: tauri::AppHandle,
    name: String,
    status: String,
) -> Result<(), String> {
    let menu = build_tray_menu(&app, &name, &status).map_err(|e| e.to_string())?;
    if let Some(tray) = app.tray_by_id("heehee-tray") {
        tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

// 잠시 쉬게 하기 토큰 — wake_now 누르면 증가 → 자동 1시간 타이머 무효화
static SLEEP_TOKEN: AtomicU64 = AtomicU64::new(0);

// JS가 하품 끝낸 뒤 호출 — 창 숨기고 1시간 뒤 복귀
#[tauri::command]
fn go_to_sleep(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("main") {
        w.hide().map_err(|e| e.to_string())?;
        let token = SLEEP_TOKEN.fetch_add(1, Ordering::SeqCst) + 1;
        let app_handle = app.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(60 * 60));
            // 사용자가 일찍 깨웠으면 토큰이 바뀌어서 자동 wake 스킵
            if SLEEP_TOKEN.load(Ordering::SeqCst) != token { return; }
            if let Some(w) = app_handle.get_webview_window("main") {
                let _ = w.show();
                let _ = w.emit("tray-wake", ());
            }
        });
    }
    Ok(())
}

// 사용자가 트레이 → 희희 부르기 클릭 시
#[tauri::command]
fn wake_now(app: tauri::AppHandle) -> Result<(), String> {
    SLEEP_TOKEN.fetch_add(1, Ordering::SeqCst); // 진행 중인 자동 타이머 무효화
    if let Some(w) = app.get_webview_window("main") {
        w.show().map_err(|e| e.to_string())?;
        let _ = w.emit("tray-wake", ()); // STRETCHING 트리거
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // ─── macOS 투명 강제 ───
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id, nil, NO, YES};
                use objc::{msg_send, sel, sel_impl};

                let ns_window = window.ns_window().unwrap() as id;
                unsafe {
                    let clear_color = NSColor::clearColor(nil);
                    ns_window.setBackgroundColor_(clear_color);
                    ns_window.setOpaque_(NO);
                    let _: () = msg_send![ns_window, setHasShadow: NO];

                    let content_view: id = msg_send![ns_window, contentView];
                    let _: () = msg_send![content_view, setWantsLayer: YES];
                    let layer: id = msg_send![content_view, layer];
                    if !layer.is_null() {
                        let cg_clear: id = msg_send![clear_color, CGColor];
                        let _: () = msg_send![layer, setBackgroundColor: cg_clear];
                    }

                    // 모든 서브뷰(웹뷰 포함)도 투명 layer 강제 — 클릭 시 webview 재렌더링이 회색 노출 방지
                    let subviews: id = msg_send![content_view, subviews];
                    let count: usize = msg_send![subviews, count];
                    for i in 0..count {
                        let subview: id = msg_send![subviews, objectAtIndex: i];
                        let _: () = msg_send![subview, setWantsLayer: YES];
                        let sub_layer: id = msg_send![subview, layer];
                        if !sub_layer.is_null() {
                            let cg_clear2: id = msg_send![clear_color, CGColor];
                            let _: () = msg_send![sub_layer, setBackgroundColor: cg_clear2];
                        }
                        // WKWebView이면 drawsBackground=false 설정
                        let responds: bool = msg_send![subview, respondsToSelector: sel!(setDrawsBackground:)];
                        if responds {
                            let _: () = msg_send![subview, setDrawsBackground: NO];
                        }
                    }

                    // ─── 모든 Spaces 표시 — Mission Control로 Space 바꿔도 따라옴 ───
                    // NSWindowCollectionBehaviorCanJoinAllSpaces = 1 << 0 = 1
                    // NSWindowCollectionBehaviorStationary       = 1 << 4 = 16 (Mission Control에서 안 흔들림)
                    let behavior: u64 = 1 | 16;
                    let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
                }
            }

            // ─── 초기 트레이 메뉴 (기본 이름/상태) ───
            let menu = build_tray_menu(app.handle(), "희희", "👫 같이 있는 중")?;

            // 트레이 아이콘 — 단색 데이지 (template image, macOS 다크/라이트 자동 적응)
            let tray_icon_bytes = include_bytes!("../icons/tray-daisy@2x.png");
            let tray_icon = tauri::image::Image::from_bytes(tray_icon_bytes)
                .unwrap_or_else(|_| app.default_window_icon().unwrap().clone());

            let _tray = TrayIconBuilder::with_id("heehee-tray")
                .icon(tray_icon)
                .icon_as_template(true)
                .tooltip("희희 — 데이지 쿼카 룸메이트")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "rename" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.emit("tray-rename", ());
                        }
                    }
                    "feed" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.emit("tray-feed", ());
                        }
                    }
                    "sleep_in" => {
                        // JS에서 하품(YAWNING) 1초 보여준 뒤 go_to_sleep 커맨드 호출
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.emit("tray-sleep", ());
                        }
                    }
                    "wake_now" => {
                        // 즉시 호출
                        let _ = wake_now(app.clone());
                    }
                    "help" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.emit("tray-help", ());
                        }
                    }
                    "info" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.emit("tray-info", ());
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // ─── 충전 감지 500ms 폴링 (빠른 반응) ───
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let mut last_state: Option<bool> = None;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let charging = check_charging();
                    if last_state != Some(charging) {
                        last_state = Some(charging);
                        let _ = app_handle.emit("power-changed", charging);
                    }
                }
            });

            let app_handle2 = app.handle().clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(500));
                let charging = check_charging();
                let _ = app_handle2.emit("power-changed", charging);
            });

            // ─── 투명 layer 주기적 재적용 (회색창 방지) ───
            // macOS가 click/focus/resize 후 webview layer 배경을 리셋해서 회색 노출
            // 100ms마다 강제 재적용 (저비용 작업, 시각 깜빡임 X)
            let app_handle_t = app.handle().clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(300)); // 초기 webview 로드 대기
                loop {
                    let _ = force_transparent(app_handle_t.clone());
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            });

            // ─── 클릭 패스스루 — 비활성화 (회색창 원인 분리 중) ───
            // modal_open state는 set_modal_open 커맨드 동작용으로만 유지
            let modal_open = Arc::new(AtomicBool::new(false));
            app.manage(modal_open.clone());
            // TODO: 회색창 안 뜨는 패스스루 방식 다시 설계

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, update_tray_menu, go_to_sleep, wake_now, set_modal_open, resize_for_modal, force_transparent])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
