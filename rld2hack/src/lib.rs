#[cfg(not(target_os = "windows"))]
compile_error!("This only works on Windows.");

#[cfg(not(target_arch = "x86"))]
compile_error!("This only works on 32bit.");

pub mod d2;
pub mod plugincore;
pub mod plugins;

use d2::{d2core::D2Core, packets::PacketFromServer};
use plugincore::plugin_info::PluginInfo;

static THREAD_RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
static mut THREAD_HANDLE: winapi::um::winnt::HANDLE = std::ptr::null_mut();

struct RLD2Hack {
    plugins: Vec<PluginInfo>,
}

impl RLD2Hack {
    pub fn new() -> Self {
        RLD2Hack {
            plugins: vec![plugins::playground::get_info(), plugins::plugin2::get_info()],
        }
    }

    fn on_game_packet_received(&self, packet: &PacketFromServer) {
        for plugin_info in &self.plugins {
            plugin_info.plugin.on_game_packet_received(packet);
        }
    }
}

fn dll_attach() {
    println!("Attach!");

    let rld2hack = RLD2Hack::new();
    let mut d2core = D2Core::new();

    d2core.set_callback(Box::new(|packet: &PacketFromServer| {
        rld2hack.on_game_packet_received(packet);
    }));

    for plugin_info in &rld2hack.plugins {
        println!(
            "Loading Plugin: {} ({}) by {}",
            plugin_info.name, plugin_info.version, plugin_info.author
        );
        plugin_info.plugin.on_load();
    }

    while THREAD_RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
        println!("Running!");

        for plugin_info in &rld2hack.plugins {
            plugin_info.plugin.on_tick();
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    for plugin in &rld2hack.plugins {
        println!("Unloading Plugin: {}", plugin.name);
        plugin.plugin.on_unload();
    }
}

unsafe extern "system" fn dll_attach_wrapper(
    base: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::DWORD {
    match std::panic::catch_unwind(|| dll_attach()) {
        Err(e) => {
            println!("`dll_attach` has panicked: {:#?}", e);
        }
        Ok(_) => {}
    }

    println!("Post attach!");

    rlwindows::free_library_and_exit_thread(base as _, 0);

    1
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    hinst_dll: winapi::shared::minwindef::HINSTANCE,
    fdw_reason: winapi::shared::minwindef::DWORD,
    _lpv_reserved: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::BOOL {
    match fdw_reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => {
            println!("DllMain: DLL_PROCESS_ATTACH {:?}", hinst_dll);

            rlwindows::disable_thread_library_calls(hinst_dll as _);

            let mut thread_id = winapi::shared::minwindef::DWORD::default();

            unsafe {
                THREAD_HANDLE = rlwindows::create_thread(
                    std::ptr::null_mut(),
                    0,
                    Some(dll_attach_wrapper),
                    hinst_dll as _,
                    0,
                    &mut thread_id,
                );
            }
        }
        winapi::um::winnt::DLL_THREAD_ATTACH => {
            println!("DllMain: DLL_THREAD_ATTACH {:?}", hinst_dll);
        }
        winapi::um::winnt::DLL_THREAD_DETACH => {
            println!("DllMain: DLL_THREAD_DETACH {:?}", hinst_dll);
        }
        winapi::um::winnt::DLL_PROCESS_DETACH => {
            println!("DllMain: DLL_PROCESS_DETACH {:?}", hinst_dll);
        }
        reason => {
            println!("DllMain: Unexpected reason! {}", reason);
        }
    }

    return winapi::shared::minwindef::TRUE;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn UnloadModule(_base: winapi::shared::minwindef::LPVOID) -> winapi::shared::minwindef::DWORD {
    println!("UnloadModule");

    THREAD_RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);

    return winapi::shared::minwindef::TRUE as _;
}
