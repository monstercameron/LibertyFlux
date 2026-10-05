// original: 0x00A4E690 vehicle_accumulate_12f8 (proposed)

/// Accumulates the global step into the float at `this + ACC` when motion is
/// strong, and reports when the total passes a threshold.
///
/// Calls the velocity source through virtual slot `SRC_SLOT` (0xEC) with
/// `this` in `ecx` and a frame scratch pair on the stack (skipped: a frame
/// address; the stub models the callee's fill through out-param writes, like
/// the real callee). When the flag byte at `this + FLAG` (0x12FC) is clear,
/// or the filled pair's sum of squares is at most `LIMIT` (4.0, unordered
/// included), stores 0.0 into `ACC` (0x12F8) and returns. Otherwise clears
/// the flag, adds the global step `DT` to the accumulator, stores it, and
/// when the total is at or above the global at `CAP` (ordered only; below
/// or unordered skips) reports through the second callee (cdecl, `(0x17,
/// obj+0x30, this, 0, 0, 0)` with `obj = [this + OBJ]`, 0x20). Returns
/// nothing defined.
///
/// Original: 0x00A4E690 (thiscall, no stack words), two callees.
lf_checker_rt::export!(thiscall, rw_00A4E690(this: u32) -> u32 {
    unsafe {
        const ACC: u32 = 0x12F8;
        const FLAG: u32 = 0x12FC;
        const OBJ: u32 = 0x20;
        const OBJ_OFF: u32 = 0x30;
        const SRC_SLOT: u32 = 0xEC;
        const DT: u32 = 0x011735BC;
        const CAP: u32 = 0x0103CD00;
        const TAG: u32 = 0x17;
        const LIMIT: f32 = f32::from_bits(0x4080_0000); // 4.0
        const REPORT_CALLEE: u32 = 2;
        let vtable = (this as *const u32).read_unaligned();
        let src: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(
                ((vtable + SRC_SLOT) as *const u32).read_unaligned() as usize,
            );
        let mut buf = [0u32; 4];
        src(this, buf.as_mut_ptr() as u32);
        // The stub filled buf[0..2] on both sides; read them back. The
        // original reads its own frame slots; same values by construction.
        let bx = f32::from_bits(buf[0]);
        let by = f32::from_bits(buf[1]);
        let acc = (this + ACC) as *mut u32;
        if ((this + FLAG) as *const u8).read() == 0 {
            acc.write_unaligned(0);
            return 0;
        }
        // Original order: (by*by) + (bx*bx).
        let sum = core::hint::black_box(by * by)
            + core::hint::black_box(bx * bx);
        // jbe after comiss(sum, LIMIT): taken when sum <= LIMIT or unordered.
        if sum.is_nan() || sum <= LIMIT {
            acc.write_unaligned(0);
            return 0;
        }
        ((this + FLAG) as *mut u8).write(0);
        let dt =
            f32::from_bits(lf_checker_rt::global::<u32>(DT).read_unaligned());
        let total = f32::from_bits(acc.read_unaligned()) + dt;
        acc.write_unaligned(total.to_bits());
        let cap =
            f32::from_bits(lf_checker_rt::global::<u32>(CAP).read_unaligned());
        // jb after comiss(total, cap): below or unordered skips the report.
        if total.is_nan() || cap.is_nan() || total < cap {
            return 0;
        }
        let obj = ((this + OBJ) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(
            REPORT_CALLEE,
            u32,
            TAG,
            obj.wrapping_add(OBJ_OFF),
            this,
            0,
            0,
            0
        );
        0
    }
});
