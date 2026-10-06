// original: 0x00ca7340 CEventHandler::vf71
/// Allocate and construct the response object for this event code and store
/// it at `this+0x0c`.
///
/// `this` is the handler (thiscall: `this` in ecx, three unread stack words).
/// The response is allocated through callee id 1 (fed the shared global at
/// `0x167e2a0`) and constructed through callee id 2 (fed `-1`); a failed
/// allocation stores zero. Returns whatever the constructor (or the
/// allocator on the failure path) left in eax.
lf_checker_rt::export!(thiscall, rw_00ca7340(this: u32, _a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const RESPONSE: u32 = 0x0c;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const ALLOC: u32 = 1;
        const BUILD: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
        if obj == 0 {
            wr32(this + RESPONSE, 0);
            return 0;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(BUILD, u32, obj, 0xFFFF_FFFFu32);
        wr32(this + RESPONSE, r);
        r
    }
});

include!("../../mut_00ca7340.rs");
include!("../../rw_00ca8280.rs");
include!("../../rw_00ca8330.rs");
include!("../../mut_00ca8280.rs");
include!("../../mut_00ca8330.rs");
