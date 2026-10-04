// original: 0x0094b640 ScriptHandle_ToIndex
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Returned when a handle is stale, dead, or of unknown type.
const INVALID: i32 = -1;

/// Object address the type-5 slot lookup runs against.
const POOL_THIS: u32 = 0x12E2420;

#[inline(always)]
unsafe fn rd_u16(file_va: u32) -> u16 {
    unsafe { (relocated(file_va) as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn rd_u8(file_va: u32) -> u8 {
    unsafe { (relocated(file_va) as *const u8).read_unaligned() }
}

/// Validate a pooled handle of the given type tag.
///
/// The handle packs a 16-bit slot index (low) with a 16-bit generation
/// counter (high). Returns the slot index when the stored generation matches
/// the handle and the slot's live flag is set, otherwise -1. Type tags 1 and
/// 2 and anything above 10 are dead and always fail; the all-ones handle
/// fails up front. Tags 9 and 10 check the generation only (no live flag).
export!(cdecl, rw_94b640(handle: u32, kind: u32) -> i32 {
    if handle == 0xFFFF_FFFF {
        return INVALID;
    }
    let index = handle & 0xFFFF;
    let generation = (handle >> 16) & 0xFFFF;
    let index_i = index as i32;
    match (kind & 0xFF) as u8 {
        0 => {
            let cell = index.wrapping_mul(48);
            if unsafe { rd_u16(0x11EE2B2 + cell) } as u32 != generation {
                return INVALID;
            }
            if unsafe { rd_u8(0x11EE2B0 + cell) } != 0 {
                index_i
            } else {
                INVALID
            }
        }
        3 => {
            let cell = index.wrapping_mul(48);
            if unsafe { rd_u16(0x1291DE2 + cell) } as u32 != generation {
                return INVALID;
            }
            if unsafe { rd_u8(0x1291DE0 + cell) } != 0 {
                index_i
            } else {
                INVALID
            }
        }
        4 => {
            let cell = index.wrapping_mul(4);
            if unsafe { rd_u16(0x11E6252 + cell) } as u32 != generation {
                return INVALID;
            }
            if unsafe { rd_u8(0x11E6250 + cell) } != 0 {
                index_i
            } else {
                INVALID
            }
        }
        5 => {
            let slot = callee_thiscall!(1, u32, relocated(POOL_THIS), index);
            if unsafe { ((slot.wrapping_add(0x60)) as *const u16).read_unaligned() } as u32
                != generation
            {
                return INVALID;
            }
            if unsafe { rd_u8(POOL_THIS + index) } & 2 != 0 {
                index_i
            } else {
                INVALID
            }
        }
        6 => {
            if unsafe { rd_u16(0x167CD20 + index.wrapping_mul(2)) } as u32 != generation {
                return INVALID;
            }
            if unsafe { rd_u8(0x167CCE0 + index) } != 0 {
                index_i
            } else {
                INVALID
            }
        }
        7 => {
            let _: u32 = callee_cdecl!(2, u32,);
            if unsafe { rd_u16(0x167E310 + index.wrapping_mul(2)) } as u32 != generation {
                return INVALID;
            }
            let _: u32 = callee_cdecl!(2, u32,);
            if unsafe { rd_u8(0x167E2D8 + index) } != 0 {
                index_i
            } else {
                INVALID
            }
        }
        8 => {
            let cell = index.wrapping_mul(0x5C);
            if unsafe { rd_u16(0x1666C8C + cell) } as u32 != generation {
                return INVALID;
            }
            if unsafe { rd_u8(0x1666CDE + cell) } & 1 != 0 {
                index_i
            } else {
                INVALID
            }
        }
        9 => {
            let cell = index.wrapping_mul(0x20);
            if unsafe { rd_u16(0x11D95F2 + cell) } as u32 == generation {
                index_i
            } else {
                INVALID
            }
        }
        10 => {
            let cell = index.wrapping_mul(0x30);
            if unsafe { rd_u16(0x11D97F0 + cell) } as u32 == generation {
                index_i
            } else {
                INVALID
            }
        }
        _ => INVALID,
    }
});
