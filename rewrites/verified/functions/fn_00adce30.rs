// original: 0x00adce30 CRenderPhaseInitB
use lf_checker_rt::{callee_thiscall, export, global, relocated};

/// Truncate `f32` to `i32` with exact x86 `CVTTSS2SI` semantics.
///
/// This is the same hardware instruction the original executes, so invalid
/// inputs (NaN, infinities, out-of-range magnitudes) yield `0x80000000`
/// exactly as the original does. A plain Rust `as` cast would saturate
/// instead and is wrong here.
#[inline(always)]
fn cvtt_ss2si(x: f32) -> i32 {
    // SAFETY: pure register operation, no memory access.
    unsafe {
        core::arch::x86::_mm_cvtt_ss2si(core::arch::x86::_mm_set_ss(x))
    }
}

// Same contract roles as the sibling initializer, plus id 7 = second virtual
// predicate at slot 7 on the source object (thiscall/0, planted).
//
// Same shape as the sibling initializer with a different vtable and a
// two-predicate mode tag: the first predicate selects 0 or 3, the second one
// overrides with 5 when it answers true. Parameter block, display-global
// scaling, rect setup and submit are identical.
export!(thiscall, rw_00adce30(obj: u32, src: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj, src);
        let o = obj as *mut u8;
        (o as *mut u32).write_unaligned(relocated(0x00EA6F34));
        (o.add(0x940) as *mut u32).write_unaligned(src);
        let vt = (src as *const u32).read_unaligned();
        let pred1: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (vt.wrapping_add(0x18) as *const u32).read_unaligned() as usize,
        );
        let t1 = pred1(src);
        let mut tag = if t1 & 0xFF != 0 { 0 } else { 3 };
        let pred2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (vt.wrapping_add(0x1c) as *const u32).read_unaligned() as usize,
        );
        let t2 = pred2(src);
        if t2 & 0xFF != 0 {
            tag = 5;
        }
        (o.add(0x8f4) as *mut u32).write_unaligned(tag);
        o.add(0x1c).write_unaligned(1u8);

        let blk = callee_thiscall!(3, u32, src.wrapping_add(0x10)) as *const f32;
        let w0 = blk.add(0).read_unaligned();
        let w1 = blk.add(1).read_unaligned();
        let w2 = blk.add(2).read_unaligned();
        let w3 = blk.add(3).read_unaligned();

        let g880 = global::<u32>(0x0105C880).read_unaligned() as i32;
        let g87c = global::<u32>(0x0105C87C).read_unaligned() as i32;
        let g884 = global::<u32>(0x0105C884).read_unaligned() as i32;
        let g888 = global::<u32>(0x0105C888).read_unaligned() as i32;
        let b1 = callee_thiscall!(4, u32, 0);
        let vb = if b1 & 0xFF != 0 { g87c } else { g880 };
        let b2 = callee_thiscall!(4, u32, 0);
        let vd = if b2 & 0xFF != 0 { g888 } else { g884 };
        let b3 = callee_thiscall!(4, u32, 0);
        let vs = if b3 & 0xFF != 0 { g87c } else { g880 };
        let b4 = callee_thiscall!(4, u32, 0);
        let vc = if b4 & 0xFF != 0 { g888 } else { g884 };

        let i1 = cvtt_ss2si((vb as f32) * w3);
        let i2 = cvtt_ss2si((vd as f32) * w2);
        let i3 = cvtt_ss2si((vs as f32) * w1);
        let i4 = cvtt_ss2si((vc as f32) * w0);
        callee_thiscall!(
            5, u32, obj.wrapping_add(0xb0),
            i4 as u32, i3 as u32, i2 as u32, i1 as u32, 0, 0x3F800000
        );

        let a1 = w0;
        let a2 = w0 + w2;
        let a3 = w1 + w3;
        let a4 = w1;
        callee_thiscall!(
            6, u32, relocated(0x0118D7F0),
            obj.wrapping_add(0xb0),
            a1.to_bits(), a2.to_bits(), a3.to_bits(), a4.to_bits(),
            0, 0x3F800000
        );
        obj
    }
});
