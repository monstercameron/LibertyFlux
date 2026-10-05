// original: 0x00CFD810 ped_task_seek_target_setup (proposed)

/// Fill a seek-target task record from the mover's position and a target.
///
/// `obj` is the task owner: `+0x18` points at a sub-object whose `+0x20`
/// points at the position block (floats x/y/z at `+0x30`/`+0x34`/`+0x38`,
/// a word at `+0x3c`). `target` has the same `+0x20` position block holding
/// the destination x/y. `out` receives a 0x40-byte record, `flag_out` a mode
/// word.
///
/// The record holds: the mover's position (x/y/z copied, `+0x3c` copied),
/// the constants 8.0, 64.0, 10000.0, 10000.0, then the position minus the
/// normalised 2D seek direction scaled by 8.0 (z slot: z minus 0.0), a word
/// the original reads from its own uninitialised stack frame (0 under the
/// checker's defined stack fill), and the four constants again. The mode
/// word is set to 2. The return value is the mover's position-block pointer.
///
/// Direction: dx/dy are the mover-minus-target x/y differences;
/// inv = 1/sqrt(dx*dx+dy*dy) unless that sum is <= 0 (or unordered-clean
/// zero), in which case 0; the step is (dx*inv*8, dy*inv*8). The float
/// operation order is the original's. Null `[obj+0x18]` takes an early exit
/// returning the entry EAX; the contract pins it non-null (see narrowed).
///
/// Original: 0x00CFD810 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00cfd810(obj: u32, out: u32, flag_out: u32, target: u32) -> u32 {
    unsafe {
        const SUB: u32 = 0x18;
        const POS: u32 = 0x20;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const PZ: u32 = 0x38;
        const PW: u32 = 0x3c;
        const ONE: f32 = 1.0; // file bytes at the 0xFE88E8 constant
        const SPEED: f32 = 8.0; // file bytes at the 0xFE8AFC constant
        const C8: u32 = 0x4100_0000;
        const C64: u32 = 0x4280_0000;
        const C10K: u32 = 0x461c_3c00; // 10000.0

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let subo = rd32(obj + SUB);
        let here = rd32(subo + POS);
        let want = rd32(target + POS);
        let dx = sub(rdf(here + PX), rdf(want + PX));
        let dy = sub(rdf(here + PY), rdf(want + PY));
        let len2 = add(mul(dx, dx), mul(dy, dy));
        let inv = if len2 <= 0.0 {
            0.0
        } else {
            core::hint::black_box(ONE) / core::hint::black_box(core::hint::black_box(len2).sqrt())
        };
        wr32(flag_out, 2);
        let dirx = mul(dx, inv);
        let diry = mul(dy, inv);
        let pz = rdf(here + PZ);
        let px_bits = rd32(here + PX);
        let dirz = mul(inv, 0.0);
        let stepx = mul(dirx, SPEED);
        let stepy = mul(diry, SPEED);
        let stepz = mul(dirz, SPEED);
        let py = rdf(here + PY);
        wr32(out, px_bits);
        wrf(out + 4, py);
        wrf(out + 8, pz);
        wr32(out + 0x0c, rd32(here + PW));
        wr32(out + 0x10, C8);
        wr32(out + 0x14, C64);
        wr32(out + 0x18, C10K);
        wr32(out + 0x1c, C10K);
        let sub2 = rd32(obj + SUB);
        let pos2 = rd32(sub2 + POS);
        let ox = sub(rdf(pos2 + PZ), stepz);
        let oy = sub(rdf(pos2 + PX), stepx);
        let oz = sub(rdf(pos2 + PY), stepy);
        wrf(out + 0x28, ox);
        wrf(out + 0x20, oy);
        wrf(out + 0x24, oz);
        // The original copies one word from its own uninitialised frame
        // slot here; the contract sets a defined stack fill of 0.
        wr32(out + 0x2c, 0);
        wr32(out + 0x30, C8);
        wr32(out + 0x34, C64);
        wr32(out + 0x38, C10K);
        wr32(out + 0x3c, C10K);
        pos2
    }
});
