use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use log::{debug, error};
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::ui::AppState;

#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{HWND, RECT, POINT, LPARAM, WPARAM, LRESULT},
    UI::WindowsAndMessaging::*,
    Graphics::Gdi::*,
    System::LibraryLoader::GetModuleHandleW,
};

// Global state for overlay management
static OVERLAY_THREAD: Lazy<Mutex<Option<OverlayThread>>> = Lazy::new(|| Mutex::new(None));
static OVERLAY_ENABLED: AtomicBool = AtomicBool::new(true);

// Window class name
const WINDOW_CLASS_NAME: &str = "PushToWhisperOverlay\0";

/// Overlay notification thread
struct OverlayThread {
    should_stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl OverlayThread {
    fn new(state: AppState) -> Result<Self, Box<dyn std::error::Error>> {
        let should_stop = Arc::new(AtomicBool::new(false));
        let should_stop_clone = should_stop.clone();
        
        let handle = thread::spawn(move || {
            if let Err(e) = create_overlay_window(state, should_stop_clone) {
                error!("Failed to create overlay window: {}", e);
            }
        });

        Ok(OverlayThread {
            should_stop,
            handle: Some(handle),
        })
    }

    fn stop(mut self) {
        self.should_stop.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(windows)]
fn create_overlay_window(state: AppState, should_stop: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    unsafe {
        // Register window class
        let class_name_wide: Vec<u16> = OsStr::new(WINDOW_CLASS_NAME)
            .encode_wide()
            .collect();
        
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(std::ptr::null()),
            hIcon: std::ptr::null_mut(),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as _,
            lpszMenuName: std::ptr::null(),
            lpszClassName: class_name_wide.as_ptr(),
            hIconSm: std::ptr::null_mut(),
        };
        
        if RegisterClassExW(&wc) == 0 {
            return Err("Failed to register window class".into());
        }
        
        // Get screen dimensions
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        
        // Window dimensions and position (bottom-right corner)
        let window_width = 250;
        let window_height = 60;
        let window_x = screen_width - window_width - 20;
        let window_y = screen_height - window_height - 80; // Above taskbar
        
        // Create window with WS_EX_NOACTIVATE to prevent focus stealing
        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW | WS_EX_LAYERED,
            class_name_wide.as_ptr(),
            std::ptr::null(),
            WS_POPUP | WS_VISIBLE,
            window_x,
            window_y,
            window_width,
            window_height,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            GetModuleHandleW(std::ptr::null()),
            std::ptr::null(),
        );
        
        if hwnd == std::ptr::null_mut() {
            return Err("Failed to create window".into());
        }
        
        // Set transparency
        SetLayeredWindowAttributes(hwnd, 0, 230, LWA_ALPHA);
        
        // Store state in window data
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, state as i32 as isize);
        
        // Show window without activating it
        ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        UpdateWindow(hwnd);
        
        // Message loop
        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        
        while !should_stop.load(Ordering::SeqCst) {
            if PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT {
                    break;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
        
        // Destroy window
        DestroyWindow(hwnd);
        UnregisterClassW(class_name_wide.as_ptr(), GetModuleHandleW(std::ptr::null()));
    }
    
    Ok(())
}

#[cfg(windows)]
unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT {
                hdc: std::ptr::null_mut(),
                fErase: 0,
                rcPaint: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                fRestore: 0,
                fIncUpdate: 0,
                rgbReserved: [0; 32],
            };
            
            let hdc = BeginPaint(hwnd, &mut ps);
            
            // Get window rect
            let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
            GetClientRect(hwnd, &mut rect);
            
            // Get state from window data
            let state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as i32;
            let app_state: AppState = unsafe { std::mem::transmute(state as u8) };
            
            // Set background color based on state
            let brush = match app_state {
                AppState::Recording => CreateSolidBrush(0x2020DC), // Red (BGR format)
                AppState::Transcribing => CreateSolidBrush(0x00BFFF), // Orange (BGR format)
                _ => CreateSolidBrush(0x808080), // Grey
            };
            
            FillRect(hdc, &rect, brush);
            DeleteObject(brush);
            
            // Draw text
            let text = match app_state {
                AppState::Recording => "🔴 Recording...\0",
                AppState::Transcribing => "🟠 Transcribing...\0",
                _ => "Ready\0",
            };
            
            SetBkMode(hdc, 1); // TRANSPARENT = 1
            SetTextColor(hdc, 0xFFFFFF); // White text
            
            let text_wide: Vec<u16> = text.encode_utf16().collect();
            DrawTextW(
                hdc,
                text_wide.as_ptr() as *mut u16,
                text_wide.len() as i32 - 1, // Exclude null terminator
                &mut rect,
                DT_CENTER | DT_VCENTER | DT_SINGLELINE,
            );
            
            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

#[cfg(not(windows))]
fn create_overlay_window(_state: AppState, _should_stop: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    // Non-Windows placeholder
    Ok(())
}

#[cfg(not(windows))]
unsafe extern "system" fn window_proc(_hwnd: usize, _msg: u32, _wparam: usize, _lparam: isize) -> isize {
    0
}

/// Show an overlay notification for the given state
pub fn show_overlay(state: AppState) {
    // Check if visual feedback is disabled
    let config = crate::utils::get_config();
    if config.disable_visual {
        debug!("Visual feedback is disabled in config, skipping overlay");
        return;
    }
    
    if !OVERLAY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    
    let mut overlay_thread = OVERLAY_THREAD.lock();
    
    // Close any existing overlay
    if let Some(thread) = overlay_thread.take() {
        thread.stop();
    }
    
    match state {
        AppState::Normal => {
            // Just close any existing overlay (already done above)
            debug!("Closed overlay (Normal state)");
        }
        AppState::Recording | AppState::Transcribing => {
            // Create new overlay
            match OverlayThread::new(state) {
                Ok(thread) => {
                    debug!("Created overlay for state: {:?}", state);
                    *overlay_thread = Some(thread);
                }
                Err(e) => {
                    error!("Failed to create overlay: {}", e);
                }
            }
        }
    }
} 