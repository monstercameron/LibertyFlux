// original: 0x008BA230 float_mix_and_dispatch (proposed)

use lf_checker_rt as rt;

/// Mixer-channel commit: polls a chain of float sources, folds them through
/// one multiply-add ladder, and dispatches the results to a long tail of
/// small setter callees plus a bounded repeat loop.
///
/// The function draws floats from three scripted pollers (two dereferenced,
/// one whose answers are ignored), combines one of them with a loop counter
/// converted to float (`counter * K_A + K_B + slot`), and threads the mixed
/// values plus a few globals through the setters. Two mode bytes select a
/// small mode constant (2 or 7); a polled count minus one bounds the final
/// loop (0 to 6 iterations). Persistent effects are three global stores (a
/// zeroed flag, a callee answer, a set flag).
///
/// Several pollers take pointers to the caller's scratch; the scratch words
/// they point at are mirrored here (zeros under the worker's fill, or the
/// computed floats) and passed by pointer. Two pushed address constants are
/// relocated links, passed through `relocated` like the original's relocated
/// push-immediates.
///
/// Original: cdecl, no arguments, returns the last setter answer observed.
unsafe fn rw_008BA230_inner<const MUTANT: bool>() -> u32 {
    unsafe {
        let _ = MUTANT;
        // Globals (file VAs).
        const GC0C: u32 = 0x1160C0C; // dispatch tag (u32)
        const GC20: u32 = 0x1160C20; // latched answer (u32)
        const GC3A: u32 = 0x1160C3A; // committed flag (byte)
        const GC40: u32 = 0x1160C40; // dispatch mode (u32)
        const GD08: u32 = 0x1160D08; // clear flag (u32)
        const G858: u32 = 0x1161858; // ladder addend (f32)
        const G864: u32 = 0x1161864; // ladder factor (f32)
        const GC250: u32 = 0x116C250; // mode byte 0
        const GC253: u32 = 0x116C253; // mode byte 1
        const G8C8: u32 = 0x11618C8; // dispatch word (u32)
        const G8DC: u32 = 0x11618DC; // dispatch float (f32)
        // Callee ids.
        const C_POLLA: u32 = 1; // float source (deref'd)
        const C_POLLX: u32 = 2; // poller, answer ignored
        const C_POLLW: u32 = 3; // float pair by index (deref'd)
        const C_COUNT: u32 = 4; // repeat count
        const C_MIX5: u32 = 5; // five-word mixer, answer ignored
        const C_LATCH: u32 = 6; // eight-word mixer, answer latched
        const C_SET3: u32 = 7; // three-word setter
        const C_WORD: u32 = 8; // word source by index (deref'd)
        const C_SET7: u32 = 9; // seven-word setter
        const C_SET1A: u32 = 10; // one-word setter
        const C_SET1B: u32 = 11; // one-word setter
        const C_SET3B: u32 = 12; // three-word setter
        const C_SET2A: u32 = 13; // two-word setter
        const C_SET2B: u32 = 14; // two-word setter
        const C_GEN: u32 = 15; // word generator
        const C_SET2C: u32 = 16; // two-word setter
        const C_SET3C: u32 = 17; // three-word setter
        const C_SET3D: u32 = 18; // three-word setter
        const C_REP: u32 = 19; // repeat setter

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn grd(a: u32) -> u32 {
            unsafe { rd32(rt::relocated(a)) }
        }
        #[inline(always)]
        unsafe fn grf(a: u32) -> f32 {
            unsafe { f32::from_bits(grd(a)) }
        }
        #[inline(always)]
        unsafe fn gwr(a: u32, v: u32) {
            unsafe { (rt::relocated(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gr8(a: u32) -> u8 {
            unsafe { (rt::relocated(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn gw8(a: u32, v: u8) {
            unsafe { (rt::relocated(a) as *mut u8).write(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Scratch words the pollers point at (zero-filled like the worker).
        let mut zero2 = [0u32; 2];
        let mut fr24: u32 = 0;
        let mut fr20: u32 = 0;
        let mut fr16: u32 = 0;
        let mut fr56: f32 = 0.0;

        gwr(GD08, 0);
        // Source poll + first ignored poller.
        let r1a: u32 = rt::callee_cdecl!(C_POLLA, u32, zero2.as_mut_ptr() as u32);
        let x0a: f32 = rdf(r1a);
        let mut s1 = [x0a.to_bits(), 0u32];
        rt::callee_cdecl!(C_POLLX, u32, 2u32, 0u32, s1.as_mut_ptr() as u32, 0u32);
        // Indexed pair poll + second ignored poller.
        let r3a: u32 = rt::callee_cdecl!(C_POLLW, u32, zero2.as_mut_ptr() as u32, 0x27u32);
        let x0b: f32 = rdf(r3a);
        let mut s2 = [x0b.to_bits(), 0u32];
        fr16 = x0b.to_bits();
        rt::callee_cdecl!(C_POLLX, u32, 2u32, 0u32, s2.as_mut_ptr() as u32, 0u32);
        // Repeat count and ladder input.
        let cnt: u32 = rt::callee_cdecl!(C_COUNT, u32, grd(GC0C));
        let ebx: i32 = (cnt as i32).wrapping_sub(1);
        rt::callee_cdecl!(C_POLLW, u32, zero2.as_mut_ptr() as u32, 0x25u32);
        let r1b: u32 = rt::callee_cdecl!(C_POLLA, u32, zero2.as_mut_ptr() as u32);
        let mut x0c: f32 = f32::from_bits(fr24);
        x0c = add(x0c, rdf(r1b));
        fr24 = x0c.to_bits();
        rt::callee_cdecl!(C_POLLW, u32, zero2.as_mut_ptr() as u32, 0x26u32);
        let mut x0d: f32 = ebx as f32;
        x0d = mul(x0d, grf(G864));
        x0d = add(x0d, grf(G858));
        x0d = add(x0d, f32::from_bits(fr20));
        fr20 = x0d.to_bits();
        let r3d: u32 = rt::callee_cdecl!(C_POLLW, u32, zero2.as_mut_ptr() as u32, 0x28u32);
        fr56 = rdf(r3d);
        // Mode select: 2 unless both bytes say otherwise.
        let mode: u32 = if gr8(GC250) == 0x6A || gr8(GC253) != 0 { 2 } else { 7 };
        // Five-word mixer over two scratch pairs.
        let mut p1 = [fr24, fr20];
        rt::callee_cdecl!(C_MIX5, u32, 5u32, mode, p1.as_mut_ptr() as u32,
            zero2.as_mut_ptr() as u32, fr56.to_bits());
        let x0e: f32 = f32::from_bits(fr16);
        // Latch mixer; its answer is stored and reused below.
        let a6: u32 = rt::callee_cdecl!(C_LATCH, u32, 0u32, 0u32,
            rt::relocated(0x11618CCu32), x0e.to_bits(), 2u32, 1u32, 0u32, 1u32);
        gwr(GC20, a6);
        fr56 = x0e;
        rt::callee_cdecl!(C_SET3, u32, a6, 0u32, fr16);
        // Trailing pair poll feeding two scratch words.
        let mut p3 = [fr24, fr20];
        let r3e: u32 = rt::callee_cdecl!(C_POLLW, u32, p3.as_mut_ptr() as u32, 0x27u32);
        let x0f: f32 = rdf(r3e.wrapping_add(4));
        fr24 = x0f.to_bits();
        fr16 = x0f.to_bits();
        fr20 = 0;
        // Third ignored poller over the refreshed scratch.
        let mut s3 = [fr16, 0u32];
        rt::callee_cdecl!(C_POLLX, u32, 2u32, 0u32, s3.as_mut_ptr() as u32, 0u32);
        let x0g: f32 = f32::from_bits(fr16);
        fr56 = x0g;
        rt::callee_cdecl!(C_SET3, u32, grd(GC20), 1u32, fr16);
        // Three word polls feeding the dispatch tail.
        let r8a: u32 = rt::callee_cdecl!(C_WORD, u32, zero2.as_mut_ptr() as u32, 0x42u32);
        let edi: u32 = rd32(r8a);
        let r8b: u32 = rt::callee_cdecl!(C_WORD, u32, zero2.as_mut_ptr() as u32, 0x3Eu32);
        let esi: u32 = rd32(r8b);
        let mut s4 = [fr24, 0u32];
        let r8c: u32 = rt::callee_cdecl!(C_WORD, u32, s4.as_mut_ptr() as u32, 0x3Bu32);
        let x0h: f32 = grf(G8DC);
        // Dispatch tail of small setters.
        rt::callee_cdecl!(C_SET7, u32, grd(GC20), grd(G8C8),
            rt::relocated(0x11618D4u32), x0h.to_bits(), rd32(r8c), esi, edi);
        rt::callee_cdecl!(C_SET1A, u32, grd(GC40));
        rt::callee_cdecl!(C_SET1B, u32, 5u32);
        rt::callee_cdecl!(C_SET3B, u32, grd(GC20), 0u32, 1u32);
        rt::callee_cdecl!(C_SET3B, u32, grd(GC20), 1u32, 1u32);
        rt::callee_cdecl!(C_SET2A, u32, 5u32, 0u32);
        if !MUTANT {
            gw8(GC3A, 1);
        }
        rt::callee_cdecl!(C_SET2B, u32, grd(GC20), 1u32);
        rt::callee_cdecl!(C_SET2B, u32, grd(GC0C), 0u32);
        let a15a: u32 = rt::callee_cdecl!(C_GEN, u32, 5u32, 0u32);
        rt::callee_cdecl!(C_SET2C, u32, 5u32, a15a);
        let a15b: u32 = rt::callee_cdecl!(C_GEN, u32, 5u32, 0u32);
        rt::callee_cdecl!(C_SET3C, u32, 5u32, a15b, 1u32);
        let a18: u32 = rt::callee_cdecl!(C_SET3D, u32, grd(GC0C), 8u32, 1u32);
        // Bounded repeat; the return is the last answer seen.
        if ebx > 0 {
            let mut r: u32 = 0;
            let mut i: i32 = 0;
            while i < ebx {
                r = rt::callee_cdecl!(C_REP, u32, grd(GC0C), i as u32, 0u32);
                i += 1;
            }
            r
        } else {
            a18
        }
    }
}

lf_checker_rt::export!(cdecl, rw_008BA230() -> u32 {
    unsafe { rw_008BA230_inner::<false>() }
});
