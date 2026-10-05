// original: 0x00d29260 CPedTargetting__vf13
/// Validate the 8 targeting slots against sight and range.
///
/// Walks the 8 slots at `this+0x48` (stride 64). A slot is processed only
/// when its flag byte `+5` is set, its subject pointer at `this+0x34` (same
/// stride) is live, and the subject's type word `& 0x3c0` reads `0xc0`. Each
/// processed slot calls the sight helper (cdecl, 8 words: owner, subject, 0,
/// 1, a near global, 0x7d0, and the owner/subject position vecs): an answer
/// of 1 marks the slot `([slot] & ~6) | 1` and clears bit 0 of `slot+4`; an
/// answer of 0 marks it `([slot] & ~5) | 2` and sets bit 0 of `slot+4` to
/// whether a limit global exceeds the squared owner-subject distance (in the
/// original's operand order); any other answer leaves the slot alone and
/// keeps its flag byte, while the 0/1 paths clear it. Returns the last value
/// written (slot 0 is pinned live so the entry EAX never survives).
///
/// Original: thiscall, ECX only.
lf_checker_rt::export!(thiscall, rw_00d29260(this: u32) -> u32 {
    unsafe {
        const SIGHT: u32 = 1;
        const NEAR_VA: u32 = 0x0103cd30;
        const LIMIT_VA: u32 = 0x00fe8ab8;
        let owner = ((this + 0x24c) as *const u32).read_unaligned();
        let omat = ((owner + 0x20) as *const u32).read_unaligned();
        let ox = f32::from_bits(((omat + 0x30) as *const u32).read_unaligned());
        let oy = f32::from_bits(((omat + 0x34) as *const u32).read_unaligned());
        let oz = f32::from_bits(((omat + 0x38) as *const u32).read_unaligned());
        let mut eax = 0u32; // unreachable: slot 0 is pinned live
        let mut i = 0u32;
        while i < 8 {
            let slot = this + 0x48 + i * 0x40;
            let flag = ((slot + 5) as *const u8).read() != 0;
            let d = ((this + 0x34 + i * 0x40) as *const u32).read_unaligned();
            let mut live = flag && d != 0;
            if live {
                live = (((d + 0x28) as *const u32).read_unaligned() & 0x3c0) == 0xc0;
            }
            if live {
                let dmat = ((d + 0x20) as *const u32).read_unaligned();
                let ex = f32::from_bits(((dmat + 0x30) as *const u32).read_unaligned());
                let ey = f32::from_bits(((dmat + 0x34) as *const u32).read_unaligned());
                let ez = f32::from_bits(((dmat + 0x38) as *const u32).read_unaligned());
                let mut obuf = [ox.to_bits(), oy.to_bits(), oz.to_bits()];
                let mut ebuf = [ex.to_bits(), ey.to_bits(), ez.to_bits()];
                let g = lf_checker_rt::global::<u32>(NEAR_VA).read_unaligned();
                let r: u32 = lf_checker_rt::callee_cdecl!(
                    SIGHT, u32, owner, d, 0, 1, g, 0x7d0,
                    obuf.as_mut_ptr() as u32, ebuf.as_mut_ptr() as u32);
                if r == 1 {
                    let f = (slot as *const u32).read_unaligned();
                    let nf = (f & 0xfffffff9) | 1;
                    (slot as *mut u32).write_unaligned(nf);
                    let b4 = ((slot + 4) as *const u8).read();
                    ((slot + 4) as *mut u8).write(b4 & 0xfe);
                    eax = nf;
                } else if r == 0 {
                    let dy = core::hint::black_box(oy) - core::hint::black_box(ey);
                    let dx = core::hint::black_box(ox) - core::hint::black_box(ex);
                    let dz = core::hint::black_box(oz) - core::hint::black_box(ez);
                    let dyy = core::hint::black_box(dy) * core::hint::black_box(dy);
                    let dxx = core::hint::black_box(dx) * core::hint::black_box(dx);
                    let dzz = core::hint::black_box(dz) * core::hint::black_box(dz);
                    let mut len2 = core::hint::black_box(dyy) + core::hint::black_box(dxx);
                    let f = (slot as *const u32).read_unaligned();
                    let nf = (f & 0xfffffffa) | 2;
                    (slot as *mut u32).write_unaligned(nf);
                    len2 = core::hint::black_box(len2) + core::hint::black_box(dzz);
                    let lim = f32::from_bits(
                        lf_checker_rt::global::<u32>(LIMIT_VA).read_unaligned());
                    let above = if lim > len2 { 1u8 } else { 0u8 };
                    let b = ((slot + 4) as *const u8).read();
                    let flip = (above ^ b) & 1;
                    eax = (nf & 0xFFFFFF00) | (flip as u32);
                    ((slot + 4) as *mut u8).write(b ^ flip);
                } else {
                    eax = r;
                    i += 1;
                    continue;
                }
                ((slot + 5) as *mut u8).write(0);
            }
            i += 1;
        }
        eax
    }
});

/// Wrong version of rw_00d29260: the flag byte is set, not cleared.
lf_checker_rt::export!(thiscall, mut_00d29260(this: u32) -> u32 {
    unsafe {
        const SIGHT: u32 = 1;
        const NEAR_VA: u32 = 0x0103cd30;
        const LIMIT_VA: u32 = 0x00fe8ab8;
        let owner = ((this + 0x24c) as *const u32).read_unaligned();
        let omat = ((owner + 0x20) as *const u32).read_unaligned();
        let ox = f32::from_bits(((omat + 0x30) as *const u32).read_unaligned());
        let oy = f32::from_bits(((omat + 0x34) as *const u32).read_unaligned());
        let oz = f32::from_bits(((omat + 0x38) as *const u32).read_unaligned());
        let mut eax = 0u32; // unreachable: slot 0 is pinned live
        let mut i = 0u32;
        while i < 8 {
            let slot = this + 0x48 + i * 0x40;
            let flag = ((slot + 5) as *const u8).read() != 0;
            let d = ((this + 0x34 + i * 0x40) as *const u32).read_unaligned();
            let mut live = flag && d != 0;
            if live {
                live = (((d + 0x28) as *const u32).read_unaligned() & 0x3c0) == 0xc0;
            }
            if live {
                let dmat = ((d + 0x20) as *const u32).read_unaligned();
                let ex = f32::from_bits(((dmat + 0x30) as *const u32).read_unaligned());
                let ey = f32::from_bits(((dmat + 0x34) as *const u32).read_unaligned());
                let ez = f32::from_bits(((dmat + 0x38) as *const u32).read_unaligned());
                let mut obuf = [ox.to_bits(), oy.to_bits(), oz.to_bits()];
                let mut ebuf = [ex.to_bits(), ey.to_bits(), ez.to_bits()];
                let g = lf_checker_rt::global::<u32>(NEAR_VA).read_unaligned();
                let r: u32 = lf_checker_rt::callee_cdecl!(
                    SIGHT, u32, owner, d, 0, 1, g, 0x7d0,
                    obuf.as_mut_ptr() as u32, ebuf.as_mut_ptr() as u32);
                if r == 1 {
                    let f = (slot as *const u32).read_unaligned();
                    let nf = (f & 0xfffffff9) | 1;
                    (slot as *mut u32).write_unaligned(nf);
                    let b4 = ((slot + 4) as *const u8).read();
                    ((slot + 4) as *mut u8).write(b4 & 0xfe);
                    eax = nf;
                } else if r == 0 {
                    let dy = core::hint::black_box(oy) - core::hint::black_box(ey);
                    let dx = core::hint::black_box(ox) - core::hint::black_box(ex);
                    let dz = core::hint::black_box(oz) - core::hint::black_box(ez);
                    let dyy = core::hint::black_box(dy) * core::hint::black_box(dy);
                    let dxx = core::hint::black_box(dx) * core::hint::black_box(dx);
                    let dzz = core::hint::black_box(dz) * core::hint::black_box(dz);
                    let mut len2 = core::hint::black_box(dyy) + core::hint::black_box(dxx);
                    let f = (slot as *const u32).read_unaligned();
                    let nf = (f & 0xfffffffa) | 2;
                    (slot as *mut u32).write_unaligned(nf);
                    len2 = core::hint::black_box(len2) + core::hint::black_box(dzz);
                    let lim = f32::from_bits(
                        lf_checker_rt::global::<u32>(LIMIT_VA).read_unaligned());
                    let above = if lim > len2 { 1u8 } else { 0u8 };
                    let b = ((slot + 4) as *const u8).read();
                    let flip = (above ^ b) & 1;
                    eax = (nf & 0xFFFFFF00) | (flip as u32);
                    ((slot + 4) as *mut u8).write(b ^ flip);
                } else {
                    eax = r;
                    i += 1;
                    continue;
                }
                // MUTANT: flag byte set instead of cleared.
                ((slot + 5) as *mut u8).write(1);
            }
            i += 1;
        }
        eax
    }
});
