//! Proof scope: complete straight-line wrapper under a valid pointer and float-edge fixture.
//! The global callback slot is a scripted recorder; its real implementation and effects are untested.
//! All three scalar callback arguments and its EAX return are compared; no pointer snapshots are required.
//! Malformed pointers and broader engine contexts are excluded; one call shape is not branch instrumentation.
//! The positive-only projection removes only the separate mutant module.
//! Specification: read the f32 pointer at byte offset 8 in the input record,
//! call the relocated debug-print function slot with (0, 0, value), and return
//! its EAX result unchanged.

pub const HANDLER_SLOT_FILE_VA: u32 = 0x0110_B714;

#[repr(C)]
pub struct PrintFloatInput {
    pub reserved: [u32; 2],
    pub value: *const f32,
}

type PrintFloatHandler = unsafe extern "cdecl" fn(u32, u32, f32) -> u32;

pub(crate) unsafe fn invoke(input: *const PrintFloatInput, second: u32) -> u32 {
    let value = (*input).value.read();
    let handler = lf_checker_rt::global::<PrintFloatHandler>(HANDLER_SLOT_FILE_VA).read();
    handler(0, second, value)
}

lf_checker_rt::export!(cdecl, rw_0086f0e0(input: *const PrintFloatInput) -> u32 {
    unsafe { invoke(input, 0) }
});

