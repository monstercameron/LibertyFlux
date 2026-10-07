// original: 0x00895010 create_and_await_ready
/// Creates a handle, stores it on the owner, and polls until the ready byte is set.
/// Returns the handle or last poll value with its low byte forced to one.
/// Proven scope: one owner object; its ready byte cycles through 0 and 1.
/// EAX is pinned to the worker-mapped callback value because the factory stub
/// preserves entry registers. The poll stub writes 1 through ECX and returns
/// 0xabcdef00. Other owner states, readiness values, and callee outcomes are
/// untested.
#[inline(never)]
extern "thiscall" fn poll_ready(owner: u32) -> u32 {
    let answer = lf_k2_rt::callee_cdecl!(6, u32, 0x0A);
    core::hint::black_box(owner);
    answer
}
lf_k2_rt::export!(thiscall, rs17_00895010(this: u32) -> u32 {
    const READY_OFF: u32 = 0x320;
    const TAG_OFF: u32 = 0x324;
    let callback = lf_k2_rt::relocated(0x00895360);
    let params = lf_k2_rt::relocated(0x00E78BA0);
    let handle = lf_k2_rt::callee_cdecl!(5, u32, callback, 0, 0x10000, 0, params, 1, 2);
    unsafe { ((this + TAG_OFF) as *mut u32).write(handle) };
    let mut tag = handle;
    if unsafe { ((this + READY_OFF) as *const u8).read() } == 0 {
        loop {
            tag = poll_ready(this);
            if unsafe { ((this + READY_OFF) as *const u8).read() } != 0 {
                break;
            }
        }
    }
    (tag & !0xFF) | 1
});

