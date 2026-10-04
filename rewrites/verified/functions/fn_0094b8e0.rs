// original: 0x0094b8e0 Script_RegisterHandle
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Object address the type-5 slot lookup runs against.
const POOL_THIS: u32 = 0x12E2420;

#[inline(always)]
unsafe fn rd_u16(file_va: u32) -> u16 {
    unsafe { (relocated(file_va) as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn wr_u16(file_va: u32, v: u16) {
    unsafe { (relocated(file_va) as *mut u16).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wr_u8(file_va: u32, v: u8) {
    unsafe { (relocated(file_va) as *mut u8).write_unaligned(v) }
}

/// Next generation counter: increments, wrapping 0xFFFE and 0xFFFF back to 1,
/// so the counter is never 0 or 0xFFFF after an allocation.
#[inline(always)]
fn bump_generation(g: u16) -> u16 {
    if g >= 0xFFFE { 1 } else { g.wrapping_add(1) }
}

/// Allocate the next handle for a slot of the given type tag.
///
/// Bumps the slot's stored generation counter (wrapping to 1 past 0xFFFE)
/// and returns the counter packed with the slot index in the low 16 bits.
/// Dead tags return the all-ones handle. Tag 4 also marks the slot live.
export!(cdecl, rw_94b8e0(index: u32, kind: u32) -> u32 {
    let pack = |fresh: u16| ((fresh as u32) << 16) | index;
    let this = relocated(POOL_THIS);
    match (kind & 0xFF) as u8 {
        0 => {
            let at = 0x11EE2B2 + index.wrapping_mul(48);
            let fresh = bump_generation(unsafe { rd_u16(at) });
            unsafe { wr_u16(at, fresh) };
            pack(fresh)
        }
        3 => {
            let at = 0x1291DE2 + index.wrapping_mul(48);
            let fresh = bump_generation(unsafe { rd_u16(at) });
            unsafe { wr_u16(at, fresh) };
            pack(fresh)
        }
        4 => {
            let cell = index.wrapping_mul(4);
            let fresh = bump_generation(unsafe { rd_u16(0x11E6252 + cell) });
            unsafe { wr_u16(0x11E6252 + cell, fresh) };
            unsafe { wr_u8(0x11E6250 + cell, 1) };
            pack(fresh)
        }
        5 => {
            let slot = callee_thiscall!(1, u32, this, index);
            let fresh = bump_generation(unsafe {
                ((slot.wrapping_add(0x60)) as *const u16).read_unaligned()
            });
            let slot = callee_thiscall!(1, u32, this, index);
            unsafe { ((slot.wrapping_add(0x60)) as *mut u16).write_unaligned(fresh) };
            let slot = callee_thiscall!(1, u32, this, index);
            let back = unsafe {
                ((slot.wrapping_add(0x60)) as *const u16).read_unaligned()
            };
            pack(back)
        }
        6 => {
            let at = 0x167CD20 + index.wrapping_mul(2);
            let fresh = bump_generation(unsafe { rd_u16(at) });
            unsafe { wr_u16(at, fresh) };
            pack(fresh)
        }
        7 => {
            let _: u32 = callee_cdecl!(2, u32,);
            let at = 0x167E310 + index.wrapping_mul(2);
            let fresh = bump_generation(unsafe { rd_u16(at) });
            let _: u32 = callee_cdecl!(2, u32,);
            unsafe { wr_u16(at, fresh) };
            let _: u32 = callee_cdecl!(2, u32,);
            pack(unsafe { rd_u16(at) })
        }
        8 => {
            let at = 0x1666C8C + index.wrapping_mul(0x5C);
            let fresh = bump_generation(unsafe { rd_u16(at) });
            unsafe { wr_u16(at, fresh) };
            pack(fresh)
        }
        9 => {
            let at = 0x11D95F2 + index.wrapping_mul(0x20);
            let fresh = bump_generation(unsafe { rd_u16(at) });
            unsafe { wr_u16(at, fresh) };
            pack(fresh)
        }
        10 => {
            let at = 0x11D97F0 + index.wrapping_mul(0x30);
            let fresh = bump_generation(unsafe { rd_u16(at) });
            unsafe { wr_u16(at, fresh) };
            pack(fresh)
        }
        _ => 0xFFFF_FFFF,
    }
});
