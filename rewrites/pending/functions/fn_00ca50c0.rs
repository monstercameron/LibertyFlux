// original: 0x00ca50c0 CEventHandler::vf50
// ---------------------------------------------------------------------------
// fn1 0x00CA50C0 CEventHandler::vf50: gated handler dispatch.
// Reads the owner's state flags and the subject pointer, derives a random
// float parameter, asks the factory for a context, and when the enable flag
// computed from the subject chain is set, builds a parameter block through
// two more factory products and records the result on the handler.
// ---------------------------------------------------------------------------
use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

#[inline(always)]
unsafe fn load_u32(addr: u32) -> u32 {
    *(addr as *const u32)
}

#[inline(always)]
unsafe fn store_u32(addr: u32, v: u32) {
    *(addr as *mut u32) = v;
}

/// Exact emulation of `cvttss2si`: truncate toward zero; NaN, +2^31 and
/// above, and below -2^31 yield the indefinite value 0x80000000.
/// (Rust's `as` saturates instead, which differs on those inputs.)
#[inline(always)]
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

export!(thiscall, rw_b08_f1(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        let owner = load_u32(this.wrapping_add(4));
        let subject = load_u32(a0.wrapping_add(0xc));
        if (*(owner.wrapping_add(0x26c) as *const u8) & 4) == 0 {
            return 0;
        }
        let gate = load_u32(owner.wrapping_add(0xb30));
        if gate == 0 || gate != subject {
            return 0;
        }
        let enabled: u8 = if *(owner.wrapping_add(0xa60) as *const u8) != 1 {
            0
        } else {
            let aux = load_u32(owner.wrapping_add(0x21c));
            if load_u32(aux.wrapping_add(0x12c)) == 2 {
                0
            } else {
                let ok = callee_thiscall!(1, u32, subject, owner);
                if (ok & 0xff) != 0 {
                    0
                } else {
                    let inner = load_u32(subject.wrapping_add(0xf50));
                    if inner == 0 {
                        0
                    } else if *(inner.wrapping_add(0x211) as *const u8) != 0 {
                        1
                    } else {
                        0
                    }
                }
            }
        };
        let rand16 = callee_cdecl!(2, u32,) & 0xffff;
        let unit = (rand16 as f32) * f32::from_bits(*global::<u32>(0xfe8680));
        let factory = callee_thiscall!(3, u32, *global::<u32>(0x167e2a0),);
        let primary: u32 = if factory == 0 {
            0
        } else {
            let scaled = unit * f32::from_bits(*global::<u32>(0xed7ca8));
            let steps = cvttss2si(scaled);
            let rest = 0x64u32.wrapping_sub(steps as u32);
            let amount = (rest as i32 as f32) * f32::from_bits(*global::<u32>(0xfe86b4));
            callee_thiscall!(6, u32, factory, gate, 0x180, amount.to_bits(), 0)
        };
        if enabled == 0 {
            store_u32(this.wrapping_add(0xc), primary);
            return 0;
        }
        let mut secondary = callee_thiscall!(4, u32, *global::<u32>(0x167e2a0),);
        if secondary != 0 {
            secondary = callee_thiscall!(7, u32, secondary,);
        }
        callee_thiscall!(8, u32, secondary, primary);
        let producer = callee_thiscall!(5, u32, *global::<u32>(0x167e2a0),);
        if producer == 0 {
            callee_thiscall!(8, u32, secondary, 0);
        } else {
            let param = callee_thiscall!(
                9, u32, producer,
                load_u32(subject.wrapping_add(0xf50)),
                1,
                *global::<u32>(0xeef940),
                *global::<u32>(0xeef944),
                *global::<u32>(0xeef948),
                *global::<u32>(0xeef94c),
                0
            );
            callee_thiscall!(8, u32, secondary, param);
        }
        store_u32(this.wrapping_add(0xc), secondary);
        0
    }
});
