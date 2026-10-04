// original: 0x008e63f0 gated_dual_channel_update
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
#[inline]
unsafe fn rd8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}

#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read() }
}

#[inline]
unsafe fn wr8(base: u32, off: u32, v: u8) {
    unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
}

#[inline]
unsafe fn wr16(base: u32, off: u32, v: u16) {
    unsafe { ((base.wrapping_add(off)) as *mut u16).write(v) }
}

#[inline]
unsafe fn wr32(base: u32, off: u32, v: u32) {
    unsafe { ((base.wrapping_add(off)) as *mut u32).write(v) }
}

/// Pinned entry-EAX value, returned on paths where no call ran.
const ENTRY_EAX: u32 = 0x1234_5678;
/// Gated dual-channel level update.
///
/// Returns the caller's entry EAX unchanged when the enable byte at +0x43E
/// is clear. Otherwise it asks the engine audio object for two input levels
/// through out-params, transforms each through the channel object at +0x54
/// (float in on the stack, float out on the x87 stack), and compares both
/// against the -100.0 silence limit: an above-limit channel with no handle
/// yet gets one created through the 17-argument creator and then has its
/// level set, while an at-or-below-limit channel with a live handle is shut
/// down through the stop helper. The returned EAX is the last callee answer
/// on the path taken (a float answer leaves its bits in EAX), or the entry
/// EAX when no call ran.

/// Pinned entry-EAX value, returned on the early path. Must equal regs[0].
const ENTRY_EAX: u32 = 0x1234_5678;
/// Engine audio object the level query runs against (constant address).
const ENGINE_OBJ: u32 = 0x0116_5880;
/// Silence-limit float (-100.0), read from the image like the original.
const LIMIT_F: u32 = 0x00FE_8DF8;
/// Per-channel creator parameter globals.
const CH0_PARAM: u32 = 0x0117_6880;
const CH1_PARAM: u32 = 0x0117_6884;

export!(thiscall, rw_008E63F0(this: u32) -> u32 {
    let mut eax = ENTRY_EAX;
    if unsafe { rd8(this, 0x43E) } == 0 {
        return eax;
    }
    let mut level_b: u32 = 0;
    let mut level_a: u32 = 0;
    // (A's EAX answer is overwritten by B2 before anything reads it.)
    let _a = callee_thiscall!(
        1, u32, relocated(ENGINE_OBJ),
        core::ptr::addr_of_mut!(level_b) as u32,
        core::ptr::addr_of_mut!(level_a) as u32
    );
    let f1 = callee_thiscall!(2, f32, this.wrapping_add(0x54), level_b);
    // (B1's EAX residue is overwritten by B2 before anything reads it.)
    let f2 = callee_thiscall!(3, f32, this.wrapping_add(0x54), level_a);
    eax = f2.to_bits();
    let limit = f32::from_bits(unsafe { global::<u32>(LIMIT_F).read() });
    // Channel 0, handle slot at +0x48.
    if f1 > limit {
        if unsafe { rd32(this, 0x48) } == 0 {
            let p = unsafe { global::<u32>(CH0_PARAM).read() };
            eax = callee_thiscall!(
                4, u32, this, p, this.wrapping_add(0x48),
                0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0xFFFF_FFFF, 0, 0
            );
        }
        let h = unsafe { rd32(this, 0x48) };
        if h != 0 {
            eax = callee_thiscall!(5, u32, h, f1.to_bits());
        }
    } else {
        let h = unsafe { rd32(this, 0x48) };
        if h != 0 {
            eax = callee_thiscall!(6, u32, h, 0);
        }
    }
    // Channel 1, handle slot at +0x4C.
    if f2 > limit {
        if unsafe { rd32(this, 0x4C) } == 0 {
            let p = unsafe { global::<u32>(CH1_PARAM).read() };
            eax = callee_thiscall!(
                7, u32, this, p, this.wrapping_add(0x4C),
                0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0xFFFF_FFFF, 0, 0
            );
        }
        let h = unsafe { rd32(this, 0x4C) };
        if h != 0 {
            eax = callee_thiscall!(5, u32, h, f2.to_bits());
        }
    } else {
        let h = unsafe { rd32(this, 0x4C) };
        if h != 0 {
            eax = callee_thiscall!(6, u32, h, 0);
        }
    }
    eax
});
