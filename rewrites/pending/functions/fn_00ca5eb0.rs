// original: 0x00ca5eb0 CEventHandler::vf48
// ---------------------------------------------------------------------------
// fn4 0x00CA5EB0 CEventHandler::vf48: event switch with two factory paths.
// The 0x76c path builds through the factory and finishes with a shared
// tail call; the 0x38f path splits on whether the inner object exists,
// either building a parameter block or tagging the factory product;
// 0xc8 clears the result and anything else forwards to a virtual handler.
// ---------------------------------------------------------------------------
use lf_k2_rt::{callee_thiscall, export, global};

#[inline(always)]
unsafe fn load_u32(addr: u32) -> u32 {
    *(addr as *const u32)
}

#[inline(always)]
unsafe fn store_u32(addr: u32, v: u32) {
    *(addr as *mut u32) = v;
}

export!(thiscall, rw_b08_f4(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        let target = load_u32(a0.wrapping_add(0x18));
        if target == 0 {
            return 0;
        }
        let event = load_u32(a0.wrapping_add(0x10));
        let inner = load_u32(target.wrapping_add(0xf50));
        if event == 0xc8 {
            store_u32(this.wrapping_add(0xc), 0);
            return 0;
        }
        if event == 0x38f {
            let f = callee_thiscall!(3, u32, *global::<u32>(0x167e2a0),);
            if inner == 0 {
                if f == 0 {
                    store_u32(this.wrapping_add(0xc), 0);
                    *(0x39 as *mut u8) = 1;
                    return 0;
                }
                let r = callee_thiscall!(
                    6, u32, f, target, 0,
                    *global::<u32>(0xeef940),
                    *global::<u32>(0xeef944),
                    *global::<u32>(0xeef948),
                    *global::<u32>(0xeef94c),
                    0
                );
                store_u32(this.wrapping_add(0xc), r);
                *((r.wrapping_add(0x39)) as *mut u8) = 1;
                return 0;
            }
            let r = if f == 0 {
                0
            } else {
                callee_thiscall!(4, u32, f, inner, 0)
            };
            store_u32(this.wrapping_add(0xc), r);
            *(r.wrapping_add(0x60) as *mut u32) |= 8;
        } else if event == 0x76c {
            let f = callee_thiscall!(1, u32, *global::<u32>(0x167e2a0),);
            if f == 0 {
                store_u32(this.wrapping_add(0xc), 0);
            } else {
                store_u32(this.wrapping_add(0xc), callee_thiscall!(2, u32, f, inner, 0));
            }
        } else {
            let vtable = load_u32(this);
            let ftarget = load_u32(vtable.wrapping_add(0x140));
            let forward: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(ftarget as usize);
            forward(this, event, inner, target);
            return 0;
        }
        let gate = load_u32(load_u32(this.wrapping_add(4)).wrapping_add(0x224));
        callee_thiscall!(5, u32, gate, inner, 1);
        0
    }
});
