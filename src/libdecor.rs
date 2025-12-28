use core::{
    ffi::{c_char, c_int, c_void},
    marker::{PhantomData, PhantomPinned},
};

use wayland_headers::wayland_client::*;

#[repr(C)]
pub struct xdg_toplevel {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct xdg_surface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct libdecor {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct libdecor_frame {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct libdecor_configuration {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct libdecor_state {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

pub type libdecor_error = c_int;
pub const LIBDECOR_ERROR_COMPOSITOR_INCOMPATIBLE: libdecor_error = 0;
pub const LIBDECOR_ERROR_INVALID_FRAME_CONFIGURATION: libdecor_error = 1;

pub type libdecor_window_state = c_int;
pub const LIBDECOR_WINDOW_STATE_NONE: libdecor_window_state = 0;
pub const LIBDECOR_WINDOW_STATE_ACTIVE: libdecor_window_state = 1 << 0;
pub const LIBDECOR_WINDOW_STATE_MAXIMIZED: libdecor_window_state = 1 << 1;
pub const LIBDECOR_WINDOW_STATE_FULLSCREEN: libdecor_window_state = 1 << 2;
pub const LIBDECOR_WINDOW_STATE_TILED_LEFT: libdecor_window_state = 1 << 3;
pub const LIBDECOR_WINDOW_STATE_TILED_RIGHT: libdecor_window_state = 1 << 4;
pub const LIBDECOR_WINDOW_STATE_TILED_TOP: libdecor_window_state = 1 << 5;
pub const LIBDECOR_WINDOW_STATE_TILED_BOTTOM: libdecor_window_state = 1 << 6;
pub const LIBDECOR_WINDOW_STATE_SUSPENDED: libdecor_window_state = 1 << 7;

pub type libdecor_resize_edge = c_int;
pub const LIBDECOR_RESIZE_EDGE_NONE: libdecor_resize_edge = 0;
pub const LIBDECOR_RESIZE_EDGE_TOP: libdecor_resize_edge = 1;
pub const LIBDECOR_RESIZE_EDGE_BOTTOM: libdecor_resize_edge = 2;
pub const LIBDECOR_RESIZE_EDGE_LEFT: libdecor_resize_edge = 3;
pub const LIBDECOR_RESIZE_EDGE_TOP_LEFT: libdecor_resize_edge = 4;
pub const LIBDECOR_RESIZE_EDGE_BOTTOM_LEFT: libdecor_resize_edge = 5;
pub const LIBDECOR_RESIZE_EDGE_RIGHT: libdecor_resize_edge = 6;
pub const LIBDECOR_RESIZE_EDGE_TOP_RIGHT: libdecor_resize_edge = 7;
pub const LIBDECOR_RESIZE_EDGE_BOTTOM_RIGHT: libdecor_resize_edge = 8;

pub type libdecor_capabilities = c_int;
pub const LIBDECOR_ACTION_MOVE: libdecor_capabilities = 1 << 0;
pub const LIBDECOR_ACTION_RESIZE: libdecor_capabilities = 1 << 1;
pub const LIBDECOR_ACTION_MINIMIZE: libdecor_capabilities = 1 << 2;
pub const LIBDECOR_ACTION_FULLSCREEN: libdecor_capabilities = 1 << 3;
pub const LIBDECOR_ACTION_CLOSE: libdecor_capabilities = 1 << 4;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct libdecor_interface {
    pub error: Option<
        unsafe extern "C" fn(context: *mut libdecor, error: libdecor_error, message: *const c_char),
    >,

    pub reserved0: Option<unsafe extern "C" fn()>,
    pub reserved1: Option<unsafe extern "C" fn()>,
    pub reserved2: Option<unsafe extern "C" fn()>,
    pub reserved3: Option<unsafe extern "C" fn()>,
    pub reserved4: Option<unsafe extern "C" fn()>,
    pub reserved5: Option<unsafe extern "C" fn()>,
    pub reserved6: Option<unsafe extern "C" fn()>,
    pub reserved7: Option<unsafe extern "C" fn()>,
    pub reserved8: Option<unsafe extern "C" fn()>,
    pub reserved9: Option<unsafe extern "C" fn()>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct libdecor_frame_interface {
    pub configure: Option<
        unsafe extern "C" fn(
            frame: *mut libdecor_frame,
            configuration: *mut libdecor_configuration,
            user_data: *mut c_void,
        ),
    >,

    pub close: Option<unsafe extern "C" fn(frame: *mut libdecor_frame, user_data: *mut c_void)>,

    pub commit: Option<unsafe extern "C" fn(frame: *mut libdecor_frame, user_data: *mut c_void)>,

    pub dismiss_popup: Option<
        unsafe extern "C" fn(
            frame: *mut libdecor_frame,
            seat_name: *const c_char,
            user_data: *mut c_void,
        ),
    >,

    pub reserved0: Option<unsafe extern "C" fn()>,
    pub reserved1: Option<unsafe extern "C" fn()>,
    pub reserved2: Option<unsafe extern "C" fn()>,
    pub reserved3: Option<unsafe extern "C" fn()>,
    pub reserved4: Option<unsafe extern "C" fn()>,
    pub reserved5: Option<unsafe extern "C" fn()>,
    pub reserved6: Option<unsafe extern "C" fn()>,
    pub reserved7: Option<unsafe extern "C" fn()>,
    pub reserved8: Option<unsafe extern "C" fn()>,
    pub reserved9: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    pub fn libdecor_unref(context: *mut libdecor);

    pub fn libdecor_new(display: *mut wl_display, iface: *mut libdecor_interface) -> *mut libdecor;

    pub fn libdecor_get_fd(context: *mut libdecor) -> c_int;

    pub fn libdecor_dispatch(context: *mut libdecor, timeout: c_int) -> c_int;

    pub fn libdecor_decorate(
        context: *mut libdecor,
        surface: *mut wl_surface,
        iface: *mut libdecor_frame_interface,
        user_data: *mut c_void,
    ) -> *mut libdecor_frame;

    pub fn libdecor_frame_ref(frame: *mut libdecor_frame);

    pub fn libdecor_frame_unref(frame: *mut libdecor_frame);

    pub fn libdecor_frame_set_visibility(frame: *mut libdecor_frame, visible: bool);

    pub fn libdecor_frame_is_visible(frame: *mut libdecor_frame) -> bool;

    pub fn libdecor_frame_set_parent(frame: *mut libdecor_frame, parent: *mut libdecor_frame);

    pub fn libdecor_frame_set_title(frame: *mut libdecor_frame, title: *const c_char);

    pub fn libdecor_frame_get_title(frame: *mut libdecor_frame) -> *const c_char;

    pub fn libdecor_frame_set_app_id(frame: *mut libdecor_frame, app_id: *const c_char);

    pub fn libdecor_frame_set_capabilities(
        frame: *mut libdecor_frame,
        capabilities: libdecor_capabilities,
    );

    pub fn libdecor_frame_unset_capabilities(
        frame: *mut libdecor_frame,
        capabilities: libdecor_capabilities,
    );

    pub fn libdecor_frame_has_capability(
        frame: *mut libdecor_frame,
        capability: libdecor_capabilities,
    ) -> bool;

    pub fn libdecor_frame_show_window_menu(
        frame: *mut libdecor_frame,
        wl_seat: *mut wl_seat,
        serial: u32,
        x: c_int,
        y: c_int,
    );

    pub fn libdecor_frame_popup_grab(frame: *mut libdecor_frame, seat_name: *const c_char);

    pub fn libdecor_frame_popup_ungrab(frame: *mut libdecor_frame, seat_name: *const c_char);

    pub fn libdecor_frame_translate_coordinate(
        frame: *mut libdecor_frame,
        surface_x: c_int,
        surface_y: c_int,
        frame_x: *mut c_int,
        frame_y: *mut c_int,
    );

    pub fn libdecor_frame_set_min_content_size(
        frame: *mut libdecor_frame,
        content_width: c_int,
        content_height: c_int,
    );

    pub fn libdecor_frame_set_max_content_size(
        frame: *mut libdecor_frame,
        content_width: c_int,
        content_height: c_int,
    );

    pub fn libdecor_frame_get_min_content_size(
        frame: *const libdecor_frame,
        content_width: *mut c_int,
        content_height: *mut c_int,
    );

    pub fn libdecor_frame_get_max_content_size(
        frame: *const libdecor_frame,
        content_width: *mut c_int,
        content_height: *mut c_int,
    );

    pub fn libdecor_frame_resize(
        frame: *mut libdecor_frame,
        wl_seat: *mut wl_seat,
        serial: u32,
        edge: libdecor_resize_edge,
    );

    pub fn libdecor_frame_move(frame: *mut libdecor_frame, wl_seat: *mut wl_seat, serial: u32);

    pub fn libdecor_frame_commit(
        frame: *mut libdecor_frame,
        state: *mut libdecor_state,
        configuration: *mut libdecor_configuration,
    );

    pub fn libdecor_frame_set_minimized(frame: *mut libdecor_frame);

    pub fn libdecor_frame_set_maximized(frame: *mut libdecor_frame);

    pub fn libdecor_frame_unset_maximized(frame: *mut libdecor_frame);

    pub fn libdecor_frame_set_fullscreen(frame: *mut libdecor_frame, output: *mut wl_output);

    pub fn libdecor_frame_unset_fullscreen(frame: *mut libdecor_frame);

    pub fn libdecor_frame_is_floating(frame: *mut libdecor_frame) -> bool;

    pub fn libdecor_frame_close(frame: *mut libdecor_frame);

    pub fn libdecor_frame_map(frame: *mut libdecor_frame);

    pub fn libdecor_frame_get_xdg_surface(frame: *mut libdecor_frame) -> *mut xdg_surface;

    pub fn libdecor_frame_get_xdg_toplevel(frame: *mut libdecor_frame) -> *mut xdg_toplevel;

    pub fn libdecor_state_new(width: c_int, height: c_int) -> *mut libdecor_state;

    pub fn libdecor_state_free(state: *mut libdecor_state);

    pub fn libdecor_configuration_get_content_size(
        configuration: *mut libdecor_configuration,
        frame: *mut libdecor_frame,
        width: *mut c_int,
        height: *mut c_int,
    ) -> bool;

    pub fn libdecor_configuration_get_window_state(
        configuration: *mut libdecor_configuration,
        window_state: *mut libdecor_window_state,
    ) -> bool;
}
