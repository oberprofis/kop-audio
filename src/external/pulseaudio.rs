#[repr(C)]
pub struct PaSimple {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct PaSampleSpec {
    pub format: u32,
    pub rate: u32,
    pub channels: u8,
}

#[link(name = "pulse-simple")]
unsafe extern "C" {
    pub fn pa_simple_new(
        server: *const i8,
        name: *const i8,
        dir: i32,
        dev: *const i8,
        stream_name: *const i8,
        ss: *const PaSampleSpec,
        map: *const i8,
        attr: *const i8,
        error: *mut i32,
    ) -> *mut PaSimple;
    pub fn pa_simple_free(s: *mut PaSimple);
    pub fn pa_simple_write(
        s: *mut PaSimple,
        data: *const std::ffi::c_void,
        bytes: usize,
        error: *mut i32,
    ) -> i32;
    pub fn pa_simple_drain(s: *mut PaSimple, error: *mut i32) -> i32;
}
