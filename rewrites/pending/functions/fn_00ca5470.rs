// original: 0x00ca5470 CEventHandler::vf15
// ---------------------------------------------------------------------------
// fn2 0x00CA5470 CEventHandler::vf15: proximity-gated event switch.
// When the tag flag is set the handler first requires the subject to be
// within range (squared distance below a limit) and three readiness polls
// to agree; otherwise, and when the flag is clear, it dispatches on the
// event id: fixed ids build results through the factory, anything else is
// forwarded to a virtual handler.
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

#[inline(never)]
unsafe fn vf15_forward(this: u32, event: u32, subject: u32) -> u32 {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(0x12c));
        let forward: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        forward(this, event, subject);
        0
    }
}

export!(thiscall, rw_b08_f2(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        let subject = load_u32(a0.wrapping_add(0x18));
        if subject == 0 {
            return 0;
        }
        let owner = load_u32(this.wrapping_add(4));
        let other = load_u32(subject.wrapping_add(0x20));
        let here = load_u32(owner.wrapping_add(0x20));
        let dx = f32::from_bits(load_u32(here.wrapping_add(0x30)))
            - f32::from_bits(load_u32(other.wrapping_add(0x30)));
        let dy = f32::from_bits(load_u32(here.wrapping_add(0x34)))
            - f32::from_bits(load_u32(other.wrapping_add(0x34)));
        if *(owner.wrapping_add(0x219) as *const u8) != 0 {
            let dist2 = dy * dy + dx * dx;
            if !(f32::from_bits(*global::<u32>(0xfe8a4c)) > dist2) {
                return 0;
            }
            let gate = load_u32(owner.wrapping_add(0x224));
            if callee_thiscall!(1, u32, gate,) != 0 {
                return 0;
            }
            let gate = load_u32(load_u32(this.wrapping_add(4)).wrapping_add(0x224));
            if callee_thiscall!(2, u32, gate,) != 0 {
                return 0;
            }
            if (callee_thiscall!(3, u32, load_u32(this.wrapping_add(4)).wrapping_add(0xbb0),) & 0xff) != 0 {
                return 0;
            }
            callee_thiscall!(4, u32, lf_k2_rt::relocated(0x128e310), 9, 5, 0, 0);
            return 0;
        }
        let event = load_u32(a0.wrapping_add(0x10));
        if event > 0x258 {
            if event != 0x38f {
                return vf15_forward(this, event, subject);
            }
            let factory = callee_thiscall!(7, u32, *global::<u32>(0x167e2a0),);
            let mut product: u32 = 0;
            if factory != 0 {
                product = callee_thiscall!(10, u32, factory, subject, 0);
            }
            store_u32(this.wrapping_add(0xc), product);
            let slot = product.wrapping_add(0x60) as *mut u32;
            *slot |= 8;
            let gate = load_u32(load_u32(this.wrapping_add(4)).wrapping_add(0x224));
            callee_thiscall!(11, u32, gate, subject, 1);
            return 0;
        }
        if event == 0x258 {
            let rand16 = callee_cdecl!(6, u32,) & 0xffff;
            let unit = (rand16 as f32) * f32::from_bits(*global::<u32>(0xfe8680));
            let factory = callee_thiscall!(8, u32, *global::<u32>(0x167e2a0),);
            if factory == 0 {
                store_u32(this.wrapping_add(0xc), 0);
                return 0;
            }
            let scaled = unit * f32::from_bits(*global::<u32>(0xed7cac));
            let steps = cvttss2si(scaled);
            let param = 0x3a98u32.wrapping_sub(steps as u32);
            let product = callee_thiscall!(9, u32, factory, subject, param);
            store_u32(this.wrapping_add(0xc), product);
            return 0;
        }
        if event == 0xc8 {
            store_u32(this.wrapping_add(0xc), 0);
            return 0;
        }
        if event != 0x1ab {
            return vf15_forward(this, event, subject);
        }
        let factory = callee_thiscall!(5, u32, *global::<u32>(0x167e2a0),);
        if factory == 0 {
            store_u32(this.wrapping_add(0xc), 0);
            return 0;
        }
        let product = callee_thiscall!(12, u32, factory, 0, 0x5f5e0ff, 0xffffffff);
        store_u32(this.wrapping_add(0xc), product);
        0
    }
});
