// original: 0x00a4a770 CVehicle::vf42
/// Attach the row's handle object, then run the level gate and tail on.
///
/// When the row word at +8 is nonzero: flag +0x24, resolve the factory
/// (thiscall/0 on the shared handle), build the handle (thiscall/4 on
/// `(factory, 0xE, rowword, [this+0x20], 0)`), store it at +0xDC4, run two
/// direct callees (thiscall/2 each), set bit 3 at +0x60, and run the two
/// virtual hooks (slots +0xA0/+0x70, thiscall/1 and /0 on the handle).
/// Always then: pass 0x100 (or 0) to the level gate (thiscall/1) depending
/// on whether the level word reaches 0.2, and tail to the shared routine
/// with ECX=this (thiscall, no stack arguments; the tail answer returns).
export!(thiscall, rw_00a4a770(this: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let ebx = (this.wrapping_add(0x20) as *const u32).read_unaligned();
        if (row.wrapping_add(8) as *const u32).read_unaligned() != 0 {
            let f24 = (this.wrapping_add(0x24) as *const u32).read_unaligned();
            (this.wrapping_add(0x24) as *mut u32).write_unaligned(f24 | 0x4000000);
            let g: u32 =
                callee_thiscall!(1, u32, (relocated(0x0171c11c) as *const u32).read_unaligned());
            let h = if g == 0 {
                0
            } else {
                let edi8 = (row.wrapping_add(8) as *const u32).read_unaligned();
                callee_thiscall!(2, u32, g, 0xe, edi8, ebx, 0)
            };
            (this.wrapping_add(0xdc4) as *mut u32).write_unaligned(h);
            let _: u32 = callee_thiscall!(3, u32, h, 4, 1);
            let ha = (this.wrapping_add(0xdc4) as *const u32).read_unaligned();
            let w60 = (ha.wrapping_add(0x60) as *const u32).read_unaligned();
            (ha.wrapping_add(0x60) as *mut u32).write_unaligned(w60 | 8);
            let hb = (this.wrapping_add(0xdc4) as *const u32).read_unaligned();
            let _: u32 = callee_thiscall!(4, u32, this, hb, 1);
            let hc = (this.wrapping_add(0xdc4) as *const u32).read_unaligned();
            let vt = (hc as *const u32).read_unaligned();
            let f5: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((vt.wrapping_add(0xa0)) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = f5(hc, 0);
            let hd = (this.wrapping_add(0xdc4) as *const u32).read_unaligned();
            let vt2 = (hd as *const u32).read_unaligned();
            let f6: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt2.wrapping_add(0x70)) as *const u32).read_unaligned() as usize,
            );
            let _: u32 = f6(hd);
        }
        let x = f32::from_bits((relocated(0x012ddeac) as *const u32).read_unaligned());
        let c = f32::from_bits((relocated(0x00fe87d0) as *const u32).read_unaligned());
        let v = if core::hint::black_box(x) >= core::hint::black_box(c) {
            0x100
        } else {
            0
        };
        let _: u32 = callee_thiscall!(7, u32, this, v);
        callee_fastcall!(8, u32, this, 0)
    }
});
