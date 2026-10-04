// original: 0x0089ee30 audio_voice_dispatch
/// Dispatch this voice by the resolver's verdict.
///
/// No-ops with 0 unless the caller's second word has a clear low byte, and
/// bails with 2 unless the resolver (callee 1/8) reports a live owner. An
/// owner whose `+0xdc` word still holds the `0xffff` marker acquires one
/// through callee 2 (which delivers it through the caller's second word);
/// a null or still-marked answer bails with 2. After a refresh tick
/// (callee 3), the tag word is latched from `+0x54` on first use and a
/// pending retrigger (`+0xe0`) is serviced through callees 4 and 5. The
/// verdict call (callee 6, on the resolver's second answer) selects one of
/// four cases: 0 runs the slot worker (callee 7) and latches `+0xc0` on a
/// 1 answer; 1 clears the 0x38 flag and returns 0; 2 also sets the 0x39
/// bit and returns 2; 3 re-services through callees 4 and 5 and returns 0.
/// An out-of-range verdict returns 2 without setting the bit.
use lf_k2_rt::{callee_cdecl, callee_thiscall, export};

/// Slot selector meaning "slot empty".
const SLOT_EMPTY: u8 = 0xff;

/// Sign-extended resolver key from `+0x3c`.
unsafe fn resolver_key(this: u32) -> u32 {
    *((this + 0x3c) as *const i16) as i32 as u32
}
export!(thiscall, rw_rb39_ee30(this: u32, _a0: u32, a1: u32) -> u32 {
    unsafe {
        if a1 & 0xff != 0 {
            return 0;
        }
        let obj: u32 = callee_cdecl!(1, u32, resolver_key(this));
        if obj == 0 || *((obj + 0x48) as *const u8) == 0 {
            return 2;
        }
        if *((this + 0xdc) as *const u16) == 0xffff {
            let mut acquired: u32 = 0;
            callee_thiscall!(2, u32, this, 0xc, &mut acquired as *mut u32 as u32, 1, 1);
            if acquired == 0 {
                return 2;
            }
            let w = *((acquired + 0xb8) as *const u16);
            *((this + 0xdc) as *mut u16) = w;
            if w == 0xffff {
                return 2;
            }
        }
        callee_thiscall!(3, u32, this);
        let tag: u32;
        if *((this + 0xde) as *const u8) == 0 {
            tag = *((this + 0x54) as *const u32);
            *((this + 0xb8) as *mut u32) = tag;
            *((this + 0xde) as *mut u8) = 1;
        } else {
            tag = *((this + 0xb8) as *const u32);
        }
        if *((this + 0xe0) as *const u8) != 0 {
            let w = *((this + 0xdc) as *const u16) as u32;
            let a: u32 = callee_thiscall!(4, u32, this, w, tag);
            callee_thiscall!(5, u32, a);
            *((this + 0xe0) as *mut u8) = 0;
        }
        let obj2: u32 = callee_cdecl!(8, u32, resolver_key(this));
        let live2 = if obj2 != 0 && *((obj2 + 0x48) as *const u8) != 0 {
            obj2
        } else {
            0
        };
        let w = *((this + 0xdc) as *const u16) as u32;
        let d: u32 = callee_thiscall!(6, u32, live2, w, tag);
        if d > 3 {
            *((this + 0x38) as *mut u8) &= 0x7f;
            return 2;
        }
        match d {
            0 => {
                let r: u32 = callee_thiscall!(7, u32, this, tag);
                if r == 1 {
                    *((this + 0xc0) as *mut u32) = tag;
                    return 1;
                }
                *((this + 0x38) as *mut u8) &= 0x7f;
                r
            }
            1 => {
                *((this + 0x38) as *mut u8) &= 0x7f;
                0
            }
            2 => {
                *((this + 0x39) as *mut u8) |= 1;
                *((this + 0x38) as *mut u8) &= 0x7f;
                2
            }
            _ => {
                let w = *((this + 0xdc) as *const u16) as u32;
                let a: u32 = callee_thiscall!(4, u32, this, w, tag);
                callee_thiscall!(5, u32, a);
                *((this + 0x38) as *mut u8) &= 0x7f;
                0
            }
        }
    }
});
