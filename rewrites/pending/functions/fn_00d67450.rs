// original: 0x00d67450 resolve_and_publish_handles
//
// Stages three key records from a global material block into frame slots,
// resolves each through a lookup callee, publishes the successful handles
// with a registry value, dispatches once through a virtual slot, runs a
// flag-gated one-time state initialization, and returns a finalizer answer.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

// Callee ids (see contract).
const R1_RESOLVE: u32 = 1; // 0x9DC680 cdecl/1: handle lookup over a key record
const R2_PUBLISH: u32 = 2; // 0x8D8730 cdecl/2: publish (handle, registry)
const R4_INIT: u32 = 4; // 0x434A90 thiscall/0: state initializer
const R5_NOTIFY: u32 = 5; // 0xDF9006 cdecl/1: notify with a code address
const R6_FINAL: u32 = 6; // 0xD640E0 thiscall/0: finalizer, answers the return
const R7_COOKIE: u32 = 7; // 0xDF8AAE: stack-cookie check (preserves registers)
// Id 3 is the planted [ecx+8] virtual dispatch, called through the
// fabricated object rather than the stub table.

// Globals touched (file VAs).
const G_KEYS: u32 = 0xEE9F2C; // key material block (52 bytes)
const G_REGISTRY: u32 = 0x12B4138; // registry value published with handles
const G_OBJECT: u32 = 0x17475DC; // dispatch object slot, cleared after use
const G_STATE: u32 = 0x1797608; // state block base (this for R4/R6 nearby)
const G_FLAG: u32 = 0x1797628; // init flag, bit 0 = already initialized
const G_THIS_A: u32 = 0x1797610; // this pointer for the initializer call
const G_NOTIFY_ARG: u32 = 0xE72DE0; // code address passed to the notify call

const NO_HANDLE: u32 = 0xFFFF_FFFF;

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

/// Stage the three key records into the frame exactly as the original lays
/// them out (byte offsets from the frame base): record A at +0x2C (15
/// bytes), record B at +0x4C (17 bytes), record C at +0x0C (15 bytes).
/// The frame starts zeroed, matching the checker's zero stack fill, so the
/// padding bytes inside snapshotted words read as zero on both sides.
unsafe fn stage_keys(fr: &mut [u32; 28]) {
    unsafe {
        let k = relocated(G_KEYS);
        // Record C first in layout order (lowest offset), then A, then B.
        fr[0x0C / 4] = rd32(k + 0x24);
        fr[0x10 / 4] = rd32(k + 0x28);
        fr[0x14 / 4] = rd32(k + 0x2C);
        fr[0x18 / 4] = rd16(k + 0x30) as u32 | ((rd8(k + 0x32) as u32) << 16);
        fr[0x2C / 4] = rd32(k + 0x00);
        fr[0x30 / 4] = rd32(k + 0x04);
        fr[0x34 / 4] = rd32(k + 0x08);
        fr[0x38 / 4] = rd16(k + 0x0C) as u32 | ((rd8(k + 0x0E) as u32) << 16);
        fr[0x4C / 4] = rd32(k + 0x10);
        fr[0x50 / 4] = rd32(k + 0x14);
        fr[0x54 / 4] = rd32(k + 0x18);
        fr[0x58 / 4] = rd32(k + 0x1C);
        fr[0x5C / 4] = rd8(k + 0x20) as u32;
    }
}

/// Rewrite of the original function at 0x00D67450 (cdecl/0 -> u32).
///
/// Resolves three key records to handles, publishes each valid handle with
/// the registry value, dispatches the stored object through virtual slot 2,
/// clears the object slot, runs the one-time state initialization when the
/// flag says it has not run yet, and returns the finalizer's answer.
export!(cdecl, rw_d67450() -> u32 {
    unsafe {
        let mut fr = [0u32; 28];
        stage_keys(&mut fr);
        let base = fr.as_mut_ptr() as u32;
        let h1 = callee_cdecl!(R1_RESOLVE, u32, base + 0x2C);
        let h2 = callee_cdecl!(R1_RESOLVE, u32, base + 0x4C);
        let h3 = callee_cdecl!(R1_RESOLVE, u32, base + 0x0C);
        let reg = rd32(relocated(G_REGISTRY));
        if h1 != NO_HANDLE {
            callee_cdecl!(R2_PUBLISH, u32, h1, reg);
        }
        if h2 != NO_HANDLE {
            callee_cdecl!(R2_PUBLISH, u32, h2, reg);
        }
        if h3 != NO_HANDLE {
            callee_cdecl!(R2_PUBLISH, u32, h3, reg);
        }
        // Virtual dispatch exactly like the original: the object comes from
        // the global slot, ECX is the table pointer stored in its first
        // word, the slot address is loaded from table+8, and the object
        // itself is the stack argument.
        let obj = rd32(relocated(G_OBJECT));
        let vtab = (obj as *const u32).read();
        let slot = ((vtab.wrapping_add(8)) as *const u32).read();
        let dispatch: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        dispatch(vtab, obj);
        let flag = rd32(relocated(G_FLAG));
        wr32(relocated(G_OBJECT), 0);
        if flag & 1 == 0 {
            wr32(relocated(G_FLAG), flag | 1);
            wr32(relocated(G_STATE), 0);
            wr32(relocated(G_STATE + 4), 0);
            callee_thiscall!(R4_INIT, u32, relocated(G_THIS_A));
            wr32(relocated(G_STATE + 0x18), 0);
            callee_cdecl!(R5_NOTIFY, u32, relocated(G_NOTIFY_ARG));
        }
        let out = callee_thiscall!(R6_FINAL, u32, relocated(G_STATE));
        // Stack-cookie check: the stub preserves every register, so the
        // finalizer answer in EAX survives into the return value.
        callee_cdecl!(R7_COOKIE, u32,);
        out
    }
});
