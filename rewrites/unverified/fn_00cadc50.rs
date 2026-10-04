// original: 0x00cadc50 move_task_position_update (proposed)
/// Refresh the task's output position at `+0x40`, two modes by flag bit.
///
/// `this` (ECX) points at the task; `+0x28` points at a sibling object. When
/// bit 0 of the byte at `+0x58` is set: if the sibling's dword at `+0x20` is
/// zero, runs the fill helper (intercepted callee 1, thiscall/0) on the
/// sibling and the mark helper (intercepted callee 2, thiscall/1) with
/// the sibling's `+0x10` in ECX and that dword as argument, then queries four
/// dwords (intercepted callee 3, cdecl/3: scratch buffer, the dword, pointer
/// `this+0x30`) and copies the answer into `+0x40`..`+0x4C`. When the bit is
/// clear: adds `([this+0x38] + [src+8], [src] + [this+0x30], [this+0x34] +
/// [src+4])`, where `src` is the sibling's `+0x10` when its `+0x20` is zero
/// else that word's value plus `0x30` (it holds a pointer then), into
/// `+0x48`/`+0x40`/`+0x44` in that store order, with the
/// original's operand order pinned. The word at `+0x4C` comes from
/// uninitialized stack in the original; the contract defines that fill as 0 on
/// both sides, so the rewrite stores 0 (see the narrowed note). Original is
/// thiscall(`this`), no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cadc50(this: u32) -> u32 {
    unsafe {
        const FILL: u32 = 1;
        const MARK: u32 = 2;
        const QUERY: u32 = 3;
        const FLAG: u32 = 0x58;
        const SIB: u32 = 0x28;
        const WORD: u32 = 0x20;
        const OUT: u32 = 0x40;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn dw(base: u32, off: u32) -> u32 {
            unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rb(base: u32, off: u32) -> u8 {
            unsafe { ((base.wrapping_add(off)) as *const u8).read() }
        }
        if rb(this, FLAG) & 1 != 0 {
            let sib = dw(this, SIB);
            if dw(sib, WORD) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(FILL, u32, sib);
                let _: u32 = lf_checker_rt::callee_thiscall!(MARK, u32,
                    sib.wrapping_add(0x10), dw(sib, WORD));
            }
            let mut buf = [0u32; 4];
            let p: u32 = lf_checker_rt::callee_cdecl!(QUERY, u32,
                buf.as_mut_ptr() as u32, dw(sib, WORD), this.wrapping_add(0x30));
            for k in 0..4u32 {
                let w = ((p.wrapping_add(k * 4)) as *const u32).read_unaligned();
                ((this.wrapping_add(OUT).wrapping_add(k * 4)) as *mut u32)
                    .write_unaligned(w);
            }
        } else {
            let base = dw(this, SIB);
            let w = dw(base, WORD);
            let src = if w == 0 {
                base.wrapping_add(0x10)
            } else {
                w.wrapping_add(0x30)
            };
            let v48 = add(f32::from_bits(dw(this, 0x38)), f32::from_bits(dw(src, 8)));
            ((this.wrapping_add(0x48)) as *mut u32).write_unaligned(v48.to_bits());
            let v40 = add(f32::from_bits(dw(src, 0)), f32::from_bits(dw(this, 0x30)));
            ((this.wrapping_add(0x40)) as *mut u32).write_unaligned(v40.to_bits());
            let v44 = add(f32::from_bits(dw(this, 0x34)), f32::from_bits(dw(src, 4)));
            ((this.wrapping_add(0x44)) as *mut u32).write_unaligned(v44.to_bits());
            ((this.wrapping_add(0x4C)) as *mut u32).write_unaligned(0);
        }
        0
    }
});
