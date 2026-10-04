// original: 0x008a4320 audVariableCurveSound_curve_notify
/// Evaluate the curve and notify about this sound's slot.
///
/// Original 0x008A4320 (`thiscall/1`, stack arg ignored): runs the curve
/// evaluator at `this+0xB8` on the float at `*(this+0xB0)`, storing the
/// double result as float to `*(this+0xB4)`; returns that destination
/// pointer when the selector is 0xFF and 0 when the slot is null;
/// otherwise reports like rw_008a3860 and tail-calls the final callee.
export!(thiscall, rw_008a4320(this: u32, _flag: u32) -> u32 {    let src = unsafe { ((this + 0xb0) as *const u32).read_unaligned() };
    let dst = unsafe { ((this + 0xb4) as *const u32).read_unaligned() };
    let bits = unsafe { (src as *const u32).read_unaligned() };
    let value: f64 = callee_thiscall!(1, f64, this.wrapping_add(0xb8), bits);
    unsafe {
        (dst as *mut f32).write_unaligned(value as f32);
    }
    let id = unsafe { ((this + 0x48) as *const u8).read_unaligned() } as u32;
    if id == 0xFF {
        return dst;
    }
    let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
    let slot = ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(id))
    });
    if slot == 0 {
        return 0;
    }
    let w54 = unsafe { ((this + 0x54) as *const u32).read_unaligned() };
    callee_thiscall!(2, u32, slot, w54, 0);
    let id2 = unsafe { ((this + 0x48) as *const u8).read_unaligned() } as u32;
    let slot2 = if id2 == 0xFF {
        0
    } else {
        let bank2 = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
        ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank2).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(id2))
    })
    };
    callee_thiscall!(3, u32, slot2)
});
