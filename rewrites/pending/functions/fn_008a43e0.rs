// original: 0x008a43e0 audVariableCurveSound_curve_query
/// Evaluate the curve and query about this sound's slot.
///
/// Original 0x008A43E0 (`thiscall/1`, stack arg ignored): returns 0 when
/// the selector is 0xFF or the slot is null; else runs the curve
/// evaluator like rw_008a4320 and tail-calls the query callee with the
/// re-resolved slot (0 when the selector is 0xFF).
export!(thiscall, rw_008a43e0(this: u32, _flag: u32) -> u32 {    let id = unsafe { ((this + 0x48) as *const u8).read_unaligned() } as u32;
    if id == 0xFF {
        return 0;
    }
    let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
    if ({
        let __stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let __base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let __entry = unsafe {
            (__base
                .wrapping_add((bank).wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        __entry.wrapping_add(__stride.wrapping_mul(id))
    }) == 0 {
        return 0;
    }
    let src = unsafe { ((this + 0xb0) as *const u32).read_unaligned() };
    let dst = unsafe { ((this + 0xb4) as *const u32).read_unaligned() };
    let bits = unsafe { (src as *const u32).read_unaligned() };
    let value: f64 = callee_thiscall!(1, f64, this.wrapping_add(0xb8), bits);
    unsafe {
        (dst as *mut f32).write_unaligned(value as f32);
    }
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
    callee_thiscall!(2, u32, slot2)
});
