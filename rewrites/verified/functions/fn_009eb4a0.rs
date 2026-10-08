//! Proof scope: complete target body within the declared bounded fixture.
//! The window probe and direct helper are recorder stubs; their native effects are untested.
//! All declared comparisons passed in 1000 completed original trials. The pointed-store
//! mutant failed three times over 26 completed originals under the same contract.
//! General engine states, native API behavior, and unobserved stack writes are unproven.
//! Acceptance binds the preserved tested source and byte-identical owned DLL copy.
//! This unbuilt projection removes only the mutant export; positive helper bytes are unchanged.
//! The ownership-rejected continuation produced zero trials and is excluded.
use lf_checker_rt::{callee_stdcall, callee_thiscall, export, global};

const SHARED_WINDOW_HANDLE: u32 = 0x017A_CCD8;
const FLAG_A: u32 = 0x0105_B48F;
const FLAG_B: u32 = 0x017E_D8D1;
const FLAG_C: u32 = 0x0117_3590;
const FLAG_D: u32 = 0x0117_3591;
const MODE_WORD: u32 = 0x011F_7060;
const COUNTER: u32 = 0x0120_88B4;
const COUNTER_MIRROR: u32 = 0x00F1_C040;
const STATE_WORD: u32 = 0x0103_7720;

/// Applies the gated ped update and returns either its status, counter, or adjusted handle.
///
/// The imported window probe is a stdcall with one HWND word. The helper at the
/// link field is intercepted by the contract; its side effects are not exercised here.
fn run_vf73(this_ptr: u32, mutant_store: bool) -> u32 {
    unsafe {
        let shared_window = *global::<u32>(SHARED_WINDOW_HANDLE);
        let is_iconic = callee_stdcall!(2, u32, shared_window);

        let base = if is_iconic != 0 {
            1u8
        } else {
            let a = *global::<u8>(FLAG_A);
            let b = *global::<u8>(FLAG_B);
            u8::from(a != 0 && b != 0)
        };
        let low = base | *global::<u8>(FLAG_C) | *global::<u8>(FLAG_D);
        if low != 0 {
            return u32::from(low);
        }
        if *global::<u32>(MODE_WORD) == 1 {
            return u32::from(low);
        }

        let counter = *global::<u32>(COUNTER);
        if counter != *global::<u32>(COUNTER_MIRROR) {
            return counter;
        }
        if *global::<u32>(STATE_WORD) == 0x12 {
            return counter;
        }

        let link = *((this_ptr.wrapping_add(0x224)) as *const u32);
        let _: u32 = callee_thiscall!(3, u32, link);
        let handle = *((this_ptr.wrapping_add(0x228)) as *const u32);
        if handle == 0 {
            return 0;
        }
        let adjusted = handle.wrapping_add(0x70);
        if adjusted == 0 {
            return 0;
        }
        let value = if mutant_store { 1 } else { 0 };
        *((adjusted.wrapping_add(0x0C)) as *mut u32) = value;
        adjusted
    }
}

export!(thiscall, rw_009eb4a0(this_ptr: u32) -> u32 {
    run_vf73(this_ptr, false)
});

