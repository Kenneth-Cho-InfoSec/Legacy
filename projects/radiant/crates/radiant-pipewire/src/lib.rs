//! Small safe wrapper around Radiant's native `PipeWire` stream adapter.

use std::ffi::c_void;
use std::ptr::NonNull;

type ProcessCallback = extern "C" fn(*mut c_void, *mut f32, u32, u32);
type ErrorCallback = extern "C" fn(*mut c_void);
type ProcessHandler = dyn FnMut(&mut [f32], usize) + Send;
type ErrorHandler = dyn FnMut() + Send;

#[repr(C)]
struct NativeStream {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn radiant_pw_start(
        rate: u32,
        channels: u32,
        process: ProcessCallback,
        error: ErrorCallback,
        userdata: *mut c_void,
    ) -> *mut NativeStream;
    fn radiant_pw_stop(stream: *mut NativeStream);
}

#[derive(Debug)]
pub struct PipeWireError;

impl std::fmt::Display for PipeWireError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("failed to create native PipeWire stream")
    }
}

impl std::error::Error for PipeWireError {}

pub struct PipeWireStream {
    handle: NonNull<NativeStream>,
    _callbacks: Box<CallbackState>,
}

struct CallbackState {
    process: Box<ProcessHandler>,
    error: Box<ErrorHandler>,
}

extern "C" fn process_trampoline(
    userdata: *mut c_void,
    output: *mut f32,
    frames: u32,
    channels: u32,
) {
    // SAFETY: the C adapter calls this only while CallbackState and its mapped buffer are live.
    let state = unsafe { &mut *userdata.cast::<CallbackState>() };
    let sample_count = usize::try_from(frames)
        .unwrap_or(usize::MAX)
        .saturating_mul(usize::try_from(channels).unwrap_or(usize::MAX));
    // SAFETY: PipeWire supplies a mapped f32 buffer of frames * channels samples.
    let output = unsafe { std::slice::from_raw_parts_mut(output, sample_count) };
    (state.process)(output, usize::try_from(channels).unwrap_or(usize::MAX));
}

extern "C" fn error_trampoline(userdata: *mut c_void) {
    // SAFETY: the C adapter calls this only while CallbackState is live.
    let state = unsafe { &mut *userdata.cast::<CallbackState>() };
    (state.error)();
}

impl PipeWireStream {
    pub fn start(
        rate: u32,
        channels: u32,
        process: impl FnMut(&mut [f32], usize) + Send + 'static,
        error: impl FnMut() + Send + 'static,
    ) -> Result<Self, PipeWireError> {
        let mut callbacks = Box::new(CallbackState {
            process: Box::new(process),
            error: Box::new(error),
        });
        // SAFETY: callbacks and userdata remain owned by the caller until this stream is dropped.
        let handle = unsafe {
            radiant_pw_start(
                rate,
                channels,
                process_trampoline,
                error_trampoline,
                std::ptr::from_mut(callbacks.as_mut()).cast(),
            )
        };
        NonNull::new(handle)
            .map(|handle| Self {
                handle,
                _callbacks: callbacks,
            })
            .ok_or(PipeWireError)
    }
}

impl Drop for PipeWireStream {
    fn drop(&mut self) {
        // SAFETY: the handle is unique, live, and stopped exactly once here.
        unsafe { radiant_pw_stop(self.handle.as_ptr()) };
    }
}

// The native stream callback and its boxed closure are transferred to PipeWire's dedicated thread.
unsafe impl Send for PipeWireStream {}
