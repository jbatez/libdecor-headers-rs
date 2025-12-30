use std::{
    cell::{Cell, RefCell},
    ffi::{CStr, c_char, c_void},
    os::raw::c_int,
    ptr::{self, null},
};

use wayland_headers::{wayland_client::*, xdg_shell_client_protocol::*};

use crate::libdecor::*;

#[link(name = "wayland-client")]
unsafe extern "C" {}

#[link(name = "wayland-protocol-statics")]
unsafe extern "C" {}

#[link(name = "decor-0")]
unsafe extern "C" {}

const CHK: c_int = 16;

#[derive(Default)]
struct Globals {
    wl_compositor: Cell<*mut wl_compositor>,
    wl_shm: Cell<*mut wl_shm>,
    xdg_wm_base: Cell<*mut xdg_wm_base>,
    seats: RefCell<Vec<Box<Seat>>>,
    outputs: RefCell<Vec<Box<Output>>>,

    has_xrgb8888: Cell<bool>,

    window: Cell<*const Window>,
}

impl Globals {
    fn from_user_data<'a>(user_data: *mut c_void) -> &'a Globals {
        let globals = user_data.cast::<Globals>();
        let globals = unsafe { globals.as_ref() };
        globals.unwrap()
    }
}

struct Seat {
    wl_seat: *mut wl_seat,
}

struct Output {
    wl_output: *mut wl_output,
    scale_factor: Cell<i32>,
}

impl Output {
    fn from_user_data<'a>(user_data: *mut c_void) -> &'a Output {
        let output = user_data.cast::<Output>();
        let output = unsafe { output.as_ref() };
        output.unwrap()
    }
}

#[derive(Default)]
struct Window {
    wl_surface: Cell<*mut wl_surface>,
    scale_factor: Cell<i32>,
    frame: Cell<*mut libdecor_frame>,
}

impl Window {
    fn from_user_data<'a>(user_data: *mut c_void) -> &'a Window {
        let window = user_data.cast::<Window>();
        let window = unsafe { window.as_ref() };
        window.unwrap()
    }
}

#[test]
fn test() {
    let globals = Globals::default();
    let globals = &globals;
    let globals_user_data = ptr::from_ref(globals).cast_mut().cast::<c_void>();

    let wl_display = unsafe { wl_display_connect(null()) };
    assert!(!wl_display.is_null());

    let wl_registry = unsafe { wl_display_get_registry(wl_display) };
    assert!(!wl_registry.is_null());
    unsafe { wl_registry_add_listener(wl_registry, &REGISTRY_LISTENER, globals_user_data) };

    unsafe { wl_display_roundtrip(wl_display) };
    let wl_compositor = globals.wl_compositor.get();
    assert!(!wl_compositor.is_null());

    unsafe { wl_display_roundtrip(wl_display) };
    assert!(globals.has_xrgb8888.get());

    let window = Window::default();
    let window = &window;
    let window_user_data = ptr::from_ref(window).cast_mut().cast::<c_void>();
    globals.window.set(window);

    let wl_surface = unsafe { wl_compositor_create_surface(wl_compositor) };
    assert!(!wl_surface.is_null());
    window.wl_surface.set(wl_surface);
    unsafe { wl_surface_add_listener(wl_surface, &SURFACE_LISTENER, window_user_data) };

    window.scale_factor.set(1);
    for output in &mut *globals.outputs.borrow_mut() {
        let window_scale_factor = window.scale_factor.get();
        let output_scale_factor = output.scale_factor.get();
        let scale_factor = i32::max(window_scale_factor, output_scale_factor);
        window.scale_factor.set(scale_factor);
    }

    let libdecor = unsafe { libdecor_new(wl_display, &raw mut LIBDECOR_INTERFACE) };
    assert!(!libdecor.is_null());
    let frame = unsafe {
        libdecor_decorate(
            libdecor,
            wl_surface,
            &raw mut LIBDECOR_FRAME_INTERFACE,
            window_user_data,
        )
    };
    assert!(!frame.is_null());
    window.frame.set(frame);

    unsafe {
        libdecor_frame_set_app_id(frame, c"libdecor-demo".as_ptr());
        libdecor_frame_set_title(frame, c"libdecor-demo".as_ptr());
        libdecor_frame_map(frame);
        libdecor_frame_set_min_content_size(frame, 15 * CHK, 10 * CHK);
    }

    loop {
        if unsafe { libdecor_dispatch(libdecor, -1) } < 0 {
            break;
        }
    }
}

static REGISTRY_LISTENER: wl_registry_listener = wl_registry_listener {
    global: Some(on_registry_global),
    global_remove: Some(on_registry_global_remove),
};

unsafe extern "C" fn on_registry_global(
    globals_user_data: *mut c_void,
    wl_registry: *mut wl_registry,
    global_id: u32,
    interface_name: *const c_char,
    interface_version: u32,
) {
    let globals = Globals::from_user_data(globals_user_data);
    let interface_name = unsafe { CStr::from_ptr(interface_name) };

    match interface_name.to_bytes() {
        b"wl_compositor" => {
            assert!(interface_version >= 4);
            let wl_compositor =
                unsafe { wl_registry_bind(wl_registry, global_id, &wl_compositor_interface, 4) };
            let wl_compositor = wl_compositor.cast::<wl_compositor>();
            assert!(!wl_compositor.is_null());

            globals.wl_compositor.set(wl_compositor);
        }
        b"wl_shm" => {
            assert!(interface_version >= 1);
            let wl_shm = unsafe { wl_registry_bind(wl_registry, global_id, &wl_shm_interface, 1) };
            let wl_shm = wl_shm.cast::<wl_shm>();
            assert!(!wl_shm.is_null());

            globals.wl_shm.set(wl_shm);
            unsafe { wl_shm_add_listener(wl_shm, &SHM_LISTENER, globals_user_data) };
        }
        b"wl_seat" => {
            assert!(interface_version >= 3);
            let wl_seat =
                unsafe { wl_registry_bind(wl_registry, global_id, &wl_seat_interface, 3) };
            let wl_seat = wl_seat.cast::<wl_seat>();
            assert!(!wl_seat.is_null());

            let seat = Box::new(Seat { wl_seat });
            let seat_user_data = ptr::from_ref(seat.as_ref()).cast_mut().cast::<c_void>();

            globals.seats.borrow_mut().push(seat);
            unsafe { wl_seat_add_listener(wl_seat, &SEAT_LISTENER, seat_user_data) };
        }
        b"wl_output" => {
            assert!(interface_version >= 2);
            let wl_output =
                unsafe { wl_registry_bind(wl_registry, global_id, &wl_output_interface, 2) };
            let wl_output = wl_output.cast::<wl_output>();
            assert!(!wl_output.is_null());

            let output = Box::new(Output {
                wl_output,
                scale_factor: Cell::new(1),
            });
            let output_user_data = ptr::from_ref(output.as_ref()).cast_mut().cast::<c_void>();

            globals.outputs.borrow_mut().push(output);
            unsafe { wl_output_add_listener(wl_output, &OUTPUT_LISTENER, output_user_data) };
        }
        b"xdg_wm_base" => {
            assert!(interface_version >= 1);
            let xdg_wm_base =
                unsafe { wl_registry_bind(wl_registry, global_id, &xdg_wm_base_interface, 1) };
            let xdg_wm_base = xdg_wm_base.cast::<xdg_wm_base>();
            assert!(!xdg_wm_base.is_null());

            globals.xdg_wm_base.set(xdg_wm_base);
            unsafe { xdg_wm_base_add_listener(xdg_wm_base, &WM_BASE_LISTENER, globals_user_data) };
        }
        _ => (),
    }
}

unsafe extern "C" fn on_registry_global_remove(
    _globals_user_data: *mut c_void,
    _wl_registry: *mut wl_registry,
    _global_id: u32,
) {
}

static SHM_LISTENER: wl_shm_listener = wl_shm_listener {
    format: Some(on_shm_format),
};

unsafe extern "C" fn on_shm_format(
    globals_user_data: *mut c_void,
    _wl_shm: *mut wl_shm,
    format: u32,
) {
    let globals = Globals::from_user_data(globals_user_data);
    if format == WL_SHM_FORMAT_XRGB8888 {
        globals.has_xrgb8888.set(true);
    }
}

static SEAT_LISTENER: wl_seat_listener = wl_seat_listener {
    capabilities: Some(on_seat_capabilities),
    name: Some(on_seat_name),
};

unsafe extern "C" fn on_seat_capabilities(
    _seat_user_data: *mut c_void,
    _wl_seat: *mut wl_seat,
    _capabilities: u32,
) {
}

unsafe extern "C" fn on_seat_name(
    _seat_user_data: *mut c_void,
    _wl_seat: *mut wl_seat,
    _name: *const c_char,
) {
}

static OUTPUT_LISTENER: wl_output_listener = wl_output_listener {
    geometry: Some(on_output_geometry),
    mode: Some(on_output_mode),
    done: Some(on_output_done),
    scale: Some(on_output_scale),
};

unsafe extern "C" fn on_output_geometry(
    _output_user_data: *mut c_void,
    _wl_output: *mut wl_output,
    _x: i32,
    _y: i32,
    _physical_width: i32,
    _physical_height: i32,
    _subpixel: i32,
    _make: *const c_char,
    _model: *const c_char,
    _transform: i32,
) {
}

unsafe extern "C" fn on_output_mode(
    _output_user_data: *mut c_void,
    _wl_output: *mut wl_output,
    _flags: u32,
    _width: i32,
    _height: i32,
    _refresh: i32,
) {
}

unsafe extern "C" fn on_output_done(_output_user_data: *mut c_void, _wl_output: *mut wl_output) {}

unsafe extern "C" fn on_output_scale(
    output_user_data: *mut c_void,
    _wl_output: *mut wl_output,
    factor: i32,
) {
    let output = Output::from_user_data(output_user_data);
    output.scale_factor.set(factor);
}

static WM_BASE_LISTENER: xdg_wm_base_listener = xdg_wm_base_listener {
    ping: Some(on_wm_base_ping),
};

unsafe extern "C" fn on_wm_base_ping(
    _globals_user_data: *mut c_void,
    xdg_wm_base: *mut xdg_wm_base,
    serial: u32,
) {
    unsafe { xdg_wm_base_pong(xdg_wm_base, serial) };
}

static SURFACE_LISTENER: wl_surface_listener = wl_surface_listener {
    enter: Some(on_surface_enter),
    leave: Some(on_surface_leave),
};

unsafe extern "C" fn on_surface_enter(
    _window_user_data: *mut c_void,
    _wl_surface: *mut wl_surface,
    _wl_output: *mut wl_output,
) {
}

unsafe extern "C" fn on_surface_leave(
    _window_user_data: *mut c_void,
    _wl_surface: *mut wl_surface,
    _wl_output: *mut wl_output,
) {
}

static mut LIBDECOR_INTERFACE: libdecor_interface = libdecor_interface {
    error: Some(on_libdecor_error),
    reserved0: None,
    reserved1: None,
    reserved2: None,
    reserved3: None,
    reserved4: None,
    reserved5: None,
    reserved6: None,
    reserved7: None,
    reserved8: None,
    reserved9: None,
};

unsafe extern "C" fn on_libdecor_error(
    _libdecor: *mut libdecor,
    _error: libdecor_error,
    _message: *const c_char,
) {
}

static mut LIBDECOR_FRAME_INTERFACE: libdecor_frame_interface = libdecor_frame_interface {
    configure: Some(on_libdecor_frame_configure),
    close: Some(on_libdecor_frame_close),
    commit: Some(on_libdecor_frame_commit),
    dismiss_popup: Some(on_libdecor_frame_dismiss_popup),
    reserved0: None,
    reserved1: None,
    reserved2: None,
    reserved3: None,
    reserved4: None,
    reserved5: None,
    reserved6: None,
    reserved7: None,
    reserved8: None,
    reserved9: None,
};

unsafe extern "C" fn on_libdecor_frame_configure(
    _frame: *mut libdecor_frame,
    _configuration: *mut libdecor_configuration,
    _window_user_data: *mut c_void,
) {
}

unsafe extern "C" fn on_libdecor_frame_close(
    _frame: *mut libdecor_frame,
    _window_user_data: *mut c_void,
) {
}

unsafe extern "C" fn on_libdecor_frame_commit(
    _frame: *mut libdecor_frame,
    _window_user_data: *mut c_void,
) {
}

unsafe extern "C" fn on_libdecor_frame_dismiss_popup(
    _frame: *mut libdecor_frame,
    _seat_name: *const c_char,
    _window_user_data: *mut c_void,
) {
}
