// original: 0x009f8630 Stats::SetFloatStatCore
use lf_k2_rt::{export, callee_addr, global};

/// Last code stored to the float table.
const F6_FLOAT_LAST: u32 = 0xFC;

/// First code stored to the int table.
const F6_INT_FIRST: u32 = 0xFD;

/// Span of the int-table codes (0xFD..=0x288).
const F6_INT_SPAN: u32 = 0x18B;

/// yield the integer-indefinite value.
fn cvtt_to_i32(x: f32) -> u32 {
    if x >= -2147483648.0 && x < 2147483648.0 {
        (x as i32) as u32
    } else {
        0x8000_0000
    }
}


/// Tunable setter with change notification (cdecl/2 -> void).
///
/// When the validity check passes, stores `val` into the float table (low
/// codes) or its truncation into the int table (middle codes), then fires the
/// two notify steps. High codes skip the store but still notify.
export!(cdecl, rw_s18f6(code: u32, val: f32) -> u32 {
    unsafe {
        let check: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        if check(code) & 0xFF == 0 {
            return 0;
        }
        let n1: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let n2: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        if code > F6_FLOAT_LAST {
            if code.wrapping_sub(F6_INT_FIRST) <= F6_INT_SPAN {
                *global::<u32>(0x12B75D4).add(code as usize) = cvtt_to_i32(val);
            }
        } else {
            *global::<u32>(0x12B75B0).add(code as usize) = val.to_bits();
        }
        n1(code);
        n2(code);
        0
    }
});
