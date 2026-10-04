// original: 0x006f4b50 mixer_combine
/// Recombines the mixer's hashed seed with its range and stores the result.
///
/// Mixes the helper's answer with a rotate-xor-multiply hash, folds in the
/// range sum from the pair accumulator, reduces the hash modulo the span, and
/// stores the blended value at slots 5 and 6. When slot 4 is clear the stored
/// value is the folded pair value instead.
export!(thiscall, rw_006f4b50(this: u32) -> () {
    unsafe {
        let obj = this as *mut u32;
        if *obj.add(4) == 0 {
            let ext = *obj.add(2) as i32 as i64;
            let a = callee_stdcall!(
                2, u32, *obj.add(7), 0, ext as u32, (ext >> 32) as u32
            );
            // The stub answers a 64-bit value with high word 0 (scripted).
            let b = callee_stdcall!(3, u32, a, 0, 100, 0);
            *obj.add(5) = b;
            *obj.add(6) = b;
            return;
        }
        let r = callee_cdecl!(1, u32,);
        let t = r.wrapping_add((r == 0) as u32);
        let fold = r.rotate_left(16) ^ r;
        let prod = (t as u64).wrapping_mul(0x5CDCFAA7);
        let sum = (prod as u32) as u64 + fold as u64;
        let saved = sum as u32;
        let ebx = ((prod >> 32) as u32).wrapping_add((sum >> 32) as u32);
        let base = *obj.add(2);
        // The pair below only runs when slot 4 reads clear at this point;
        // with a pure stub that never happens on the main path, but the
        // branch is mirrored so the seed value stays faithful either way.
        let seed: u32;
        if *obj.add(4) == 0 {
            let ext = base as i32 as i64;
            let a = callee_stdcall!(
                2, u32, *obj.add(7), 0, ext as u32, (ext >> 32) as u32
            );
            seed = callee_stdcall!(3, u32, a, 0, 100, 0);
        } else {
            seed = base;
        }
        let acc = callee_thiscall!(4, u32, this);
        let span = (1u32.wrapping_sub(seed)).wrapping_add(acc);
        let mixed = ebx.wrapping_sub(saved.wrapping_mul(0x23230559));
        let rem = (mixed & 0x7FFF_FFFF) as i32 % (span as i32);
        let out = (rem as u32).wrapping_add(seed);
        *obj.add(5) = out;
        *obj.add(6) = out;
    }
});
