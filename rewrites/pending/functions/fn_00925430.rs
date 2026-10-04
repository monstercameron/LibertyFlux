// original: 0x00925430 render_targets_recreate (proposed)

/// Recreate four cached render targets when the requested size changes.
///
/// Takes three size words `(w, h, d)` on the stack. Always clears the dirty
/// byte at 0x119D094. If the cached sizes at 0x119D09C/0A0/0A4 already equal
/// `(w, h, d)` nothing else happens. Otherwise: releases the old targets
/// (callee 1), builds a 48-byte target descriptor on the stack and runs it
/// through the descriptor initializer (callee 2, thiscall with the descriptor
/// address and 0), then creates four targets through the device object at
/// 0x17F5630 (vtable slot +0x38, thiscall with six stack words each):
///   1. struct 0xE860AC, 3, w*4, w*4, 0x20, descriptor (byte +16 = 0)
///   2. struct 0xE860C4, 3, w*4, w,   0x20, descriptor (byte +16 = 1)
///   3. struct 0xE860E0, 3, w,   w,   0x20, descriptor (dword +44 = 0xE)
///   4. struct 0xE860F8, 3, w,   w,   0x20, descriptor (dword +44 = 8)
/// The descriptor bytes written around the calls are constants: byte +0 = 1,
/// dword +4 = 0, byte +8 = 1, dword +12 = 1, word +17 = 0x101, dword +20 = 0,
/// byte +37 = 1 (the rest comes from the initializer). Each returned handle
/// is stored to its slot (0x119CFF0, 0x119CFF4, 0x119CFEC, 0x119CFE8), and `w`
/// is broadcast to eight table entries at 0x119F104 + k*0x110 and sixteen at
/// 0x11A0B14 + k*0x100.
///
/// No return value (cdecl, plain `ret`).
lf_checker_rt::export!(cdecl, rw_00925430(w: u32, h: u32, d: u32) -> u32 {
    unsafe {
        const DIRTY: u32 = 0x119D094;
        const CW: u32 = 0x119D09C;
        const CH: u32 = 0x119D0A0;
        const CD: u32 = 0x119D0A4;
        const DEVICE: u32 = 0x17F5630;
        const CREATE_SLOT: u32 = 0x38;
        const SLOTS: [u32; 4] = [0x119CFF0, 0x119CFF4, 0x119CFEC, 0x119CFE8];
        const DESCS: [u32; 4] = [0x00E860AC, 0x00E860C4, 0x00E860E0, 0x00E860F8];

        (lf_checker_rt::global::<u8>(DIRTY) as *mut u8).write(0);
        let cw = lf_checker_rt::global::<u32>(CW);
        let ch = lf_checker_rt::global::<u32>(CH);
        let cd = lf_checker_rt::global::<u32>(CD);
        if cw.read_unaligned() == w && ch.read_unaligned() == h && cd.read_unaligned() == d {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(1, u32,);
        // 48-byte target descriptor, stack scratch (callee-owned bytes stay
        // uninitialized exactly like the original's; only the constant
        // patches below are behaviour of this function).
        let mut desc = core::mem::MaybeUninit::<[u8; 48]>::uninit();
        let p = desc.as_mut_ptr() as u32;
        cw.write_unaligned(w);
        ch.write_unaligned(h);
        cd.write_unaligned(d);
        lf_checker_rt::callee_thiscall!(2, u32, p, 0u32);
        let ww = cw.read_unaligned();
        let dev = lf_checker_rt::global::<u32>(DEVICE).read_unaligned();
        let vtable = (dev as *const u32).read_unaligned();
        let create: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute((vtable.wrapping_add(CREATE_SLOT) as *const u32).read_unaligned() as usize);
        // Constant patches shared by all four creations.
        (p as *mut u8).write(1);
        ((p + 4) as *mut u32).write_unaligned(0);
        ((p + 8) as *mut u8).write(1);
        ((p + 12) as *mut u32).write_unaligned(1);
        ((p + 17) as *mut u16).write_unaligned(0x0101);
        ((p + 20) as *mut u32).write_unaligned(0);
        ((p + 37) as *mut u8).write(1);
        // 1: byte +16 = 0, dword +44 = 8.
        ((p + 16) as *mut u8).write(0);
        ((p + 44) as *mut u32).write_unaligned(8);
        let r0 = create(dev, lf_checker_rt::relocated(DESCS[0]), 3, ww.wrapping_mul(4), ww.wrapping_mul(4), 0x20, p);
        lf_checker_rt::global::<u32>(SLOTS[0]).write_unaligned(r0);
        // 2: byte +16 = 1.
        ((p + 16) as *mut u8).write(1);
        let r1 = create(dev, lf_checker_rt::relocated(DESCS[1]), 3, ww.wrapping_mul(4), ww, 0x20, p);
        lf_checker_rt::global::<u32>(SLOTS[1]).write_unaligned(r1);
        // 3: dword +44 = 0xE.
        ((p + 44) as *mut u32).write_unaligned(0x0E);
        let r2 = create(dev, lf_checker_rt::relocated(DESCS[2]), 3, ww, ww, 0x20, p);
        lf_checker_rt::global::<u32>(SLOTS[2]).write_unaligned(r2);
        // 4: dword +44 = 8.
        ((p + 44) as *mut u32).write_unaligned(8);
        let r3 = create(dev, lf_checker_rt::relocated(DESCS[3]), 3, ww, ww, 0x20, p);
        lf_checker_rt::global::<u32>(SLOTS[3]).write_unaligned(r3);
        // Broadcast w to the per-view tables.
        for k in 0..8u32 {
            lf_checker_rt::global::<u32>(0x119F104 + k * 0x110).write_unaligned(ww);
        }
        let mut t = 0x11A0B14u32;
        while t < 0x11A1B14 {
            lf_checker_rt::global::<u32>(t).write_unaligned(ww);
            t = t.wrapping_add(0x100);
        }
        0
    }
});
