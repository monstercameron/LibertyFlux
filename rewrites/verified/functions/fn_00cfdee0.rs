// original: 0x00cfdee0 ped_task_aim_update (proposed)

/// Update an aim-task record toward a target point, then advance its driver.
///
/// `obj` is the task owner (`+0x20` position block with x/y at `+0x30`/`+0x34`,
/// driver object at `+0xd68`). `pos` holds the target x/y. `out` receives a
/// 16-byte record, `tgt_out` (the `[ebp+0x14]` pointer) a driven position.
/// `this` (entry ECX) holds three accumulated floats at `+0x60`/`+0x64`/`+0x68`.
///
/// The record holds the normalised 2D direction (target minus mover position,
/// scaled by 1/length unless the squared length is <= 0, in which case 0;
/// z is 0) followed by a word the original reads from its own uninitialised
/// stack frame (0 under the checker's defined stack fill). If the driver
/// object is null, or the advance call (cdecl, five args) answers false, the
/// function returns 0 with that record in place. Otherwise the three `this`
/// floats are added into `tgt_out`, the driver call (thiscall, two args: a
/// frame slot holding 0 and `out`) answers a 16-byte result that replaces
/// the whole record, and the return is that result's last word with its low
/// byte set to 1. The float operation order is the original's.
///
/// Original: 0x00CFDEE0 (thiscall, ECX plus four stack words).
lf_checker_rt::export!(thiscall, rw_00cfdee0(this: u32, obj: u32, pos: u32, out: u32, tgt_out: u32) -> u32 {
    unsafe {
        const POS: u32 = 0x20;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const DRV: u32 = 0xd68;
        const ONE: f32 = 1.0; // file bytes at the 0xFE88E8 constant
        const ADVANCE: u32 = 1;
        const DRIVER_CALL: u32 = 2;

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

        let objpos = rd32(obj + POS);
        let dx = sub(rdf(pos), rdf(objpos + PX));
        let dy = sub(rdf(pos + 4), rdf(objpos + PY));
        // The original copies one word from its own uninitialised frame
        // slot here; the contract sets a defined stack fill of 0.
        wr32(out + 0x0c, 0);
        wrf(out, dx);
        wrf(out + 4, dy);
        wr32(out + 8, 0);
        let len2 = add(mul(dy, dy), mul(dx, dx));
        let inv = if len2 <= 0.0 {
            0.0
        } else {
            core::hint::black_box(ONE) / core::hint::black_box(core::hint::black_box(len2).sqrt())
        };
        let d68 = rd32(obj + DRV);
        wrf(out, mul(inv, dx));
        wrf(out + 4, mul(dy, inv));
        wrf(out + 8, mul(inv, 0.0));
        if d68 == 0 {
            return 0;
        }
        let go: u32 = lf_checker_rt::callee_cdecl!(ADVANCE, u32, d68, obj, out, tgt_out, 0);
        if go & 0xff == 0 {
            return 0;
        }
        wrf(tgt_out, add(rdf(tgt_out), rdf(this + 0x60)));
        wrf(tgt_out + 4, add(rdf(tgt_out + 4), rdf(this + 0x64)));
        wrf(tgt_out + 8, add(rdf(tgt_out + 8), rdf(this + 0x68)));
        let mut frame_slot: u32 = 0;
        let res: u32 = lf_checker_rt::callee_thiscall!(
            DRIVER_CALL, u32, d68, &frame_slot as *const u32 as u32, out);
        wr32(out, rd32(res));
        wr32(out + 4, rd32(res + 4));
        wr32(out + 8, rd32(res + 8));
        let last = rd32(res + 0x0c);
        wr32(out + 0x0c, last);
        (last & 0xffff_ff00) | 1
    }
});
