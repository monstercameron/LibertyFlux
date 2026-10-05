// original: 0x00D459B0 peds_task_react_dispatch (proposed)

/// Dispatch a ped reaction: pick one of three handlers or decline.
///
/// `this` is the reaction task (`+0x5d` flag byte, `+0x60` a four-float slot);
/// `ped` is the ped. Returns 1 (low byte) when a handler runs, else 0; the
/// upper return bytes keep stale eax.
///
/// Behaviour: decline unless the gate table is live and the ped passes its
/// probe. When the ped carries the reaction bit, resolves a live target whose
/// packed id is valid and names a live row, run handler one: flag the task,
/// seed the slot through the pose helper, lazily resolve the shared style id
/// (once-flagged globals), and issue the ten-word reaction request. When the
/// indexed lookup succeeds and its out-vector validates, run handler two:
/// flag the task, lift the vector's height by 1.5, store the four words into
/// the slot, and issue the request with its own style. When the sequential
/// lookup succeeds, its pose validates, the slot sits over four units from
/// the ped anchor, and the facing dot lands strictly inside (0, 0.9659), run
/// handler three after raising the slot's third word by 1.0. Anything else
/// declines. Float operation order is the original's.
///
/// Original: 0x00D459B0 (thiscall, one stack word, returns u32, al significant).
lf_checker_rt::export!(thiscall, rw_00D459B0(this: u32, ped: u32) -> u32 {
    unsafe {
        const GATE: u32 = 0x0118_D818;
        const ROWS: u32 = 0x0117_8284;
        const ONCE_FLAG: u32 = 0x0172_0F80;
        const STYLE_ID: u32 = 0x0172_0F7C;
        const PED_PROBE_BIT: u32 = 0x26c;
        const PED_TARGET: u32 = 0xb30;
        const TARGET_PACKED: u32 = 0xde8;
        const PED_SEQ: u32 = 0x224;
        const PED_ANCHOR: u32 = 0x20;
        const PED_REQ: u32 = 0xbb0;
        const TASK_FLAG: u32 = 0x5d;
        const SLOT: u32 = 0x60;
        const STYLE_A: u32 = 0x00EE_3F30;
        const REQ_A: u32 = 0x00EE_3F4C;
        const REQ_B: u32 = 0x00EE_3F68;
        const REQ_C: u32 = 0x00EE_3F80;
        const LIFT: f32 = f32::from_bits(0x3FC0_0000);
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        const FAR2: f32 = f32::from_bits(0x4080_0000);
        const DOT_LIM: f32 = f32::from_bits(0x3F77_4539);
        const PROBE_CALLEE: u32 = 1;
        const TARGET_CALLEE: u32 = 2;
        const POSE_CALLEE: u32 = 3;
        const STYLE_CALLEE: u32 = 4;
        const REQ_CALLEE: u32 = 5;
        const IDX_CALLEE: u32 = 6;
        const SEQ_CALLEE: u32 = 7;
        const VEC_CALLEE: u32 = 8;
        const SEQPOSE_CALLEE: u32 = 9;
        const FACE_CALLEE: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn flag_task(this: u32) {
            unsafe {
                let p = this.wrapping_add(TASK_FLAG) as *mut u8;
                p.write(p.read() | 1);
            }
        }
        #[inline(always)]
        unsafe fn request(req_this: u32, style: u32, id: u32, slot: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    REQ_CALLEE, u32, req_this, style, id, 0u32, 0x64u32, 0xFFFF_FFFFu32,
                    slot, 0u32, 0x1F4u32, 0x1F4u32, 1u32
                );
            }
        }

        let gate = lf_checker_rt::global::<u32>(GATE) as u32;
        if rd32(gate.wrapping_add(rd32(gate).wrapping_mul(4))) == 0 {
            return 0;
        }
        if lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, ped, 0u32) & 0xFF == 0 {
            return 0;
        }
        if rd8(ped.wrapping_add(PED_PROBE_BIT)) & 4 != 0 {
            let target = rd32(ped.wrapping_add(PED_TARGET));
            if target != 0
                && lf_checker_rt::callee_thiscall!(TARGET_CALLEE, u32, target, ped) & 0xFF != 0
            {
                let packed = rd32(rd32(ped.wrapping_add(PED_TARGET)).wrapping_add(TARGET_PACKED));
                let idx = packed & 0xFFFF;
                if idx != 0xFFFF {
                    let rows = lf_checker_rt::global::<u32>(ROWS) as u32;
                    let row = rd32(rows.wrapping_add(idx.wrapping_mul(4)));
                    if row != 0 {
                        flag_task(this);
                        let posed = row.wrapping_add((packed >> 16).wrapping_mul(32));
                        let slot = this.wrapping_add(SLOT);
                        lf_checker_rt::callee_thiscall!(POSE_CALLEE, u32, posed, slot);
                        let flagp = lf_checker_rt::global::<u32>(ONCE_FLAG);
                        let mut f = flagp.read_unaligned();
                        let id = if f & 1 == 0 {
                            f |= 1;
                            flagp.write_unaligned(f);
                            let got = lf_checker_rt::callee_cdecl!(
                                STYLE_CALLEE, u32,
                                lf_checker_rt::relocated(STYLE_A), 0u32
                            );
                            lf_checker_rt::global::<u32>(STYLE_ID).write_unaligned(got);
                            got
                        } else {
                            lf_checker_rt::global::<u32>(STYLE_ID).read_unaligned()
                        };
                        request(
                            ped.wrapping_add(PED_REQ),
                            lf_checker_rt::relocated(REQ_A),
                            id,
                            this.wrapping_add(SLOT),
                        );
                        return 1;
                    }
                }
            }
        }
        let seq_base = rd32(ped.wrapping_add(PED_SEQ)).wrapping_add(0x44);
        let found = lf_checker_rt::callee_thiscall!(IDX_CALLEE, u32, seq_base, 0x3AEu32);
        if found != 0 {
            let mut out_a = [0u32; 4];
            let mut out_b = [0f32; 3];
            let ok = lf_checker_rt::callee_thiscall!(
                VEC_CALLEE, u32, found, ped, out_a.as_mut_ptr() as u32,
                out_b.as_mut_ptr() as u32, 1u32
            );
            if ok & 0xFF == 0 {
                return 0;
            }
            flag_task(this);
            let raised = add(out_b[2], LIFT);
            let slot = this.wrapping_add(SLOT);
            wrf(slot, out_b[0]);
            wrf(slot.wrapping_add(4), out_b[1]);
            wrf(slot.wrapping_add(8), raised);
            wrf(slot.wrapping_add(12), f32::from_bits(out_a[3]));
            request(
                ped.wrapping_add(PED_REQ),
                lf_checker_rt::relocated(REQ_B),
                0,
                slot,
            );
            return 1;
        }
        let found2 = lf_checker_rt::callee_thiscall!(SEQ_CALLEE, u32, seq_base, 0x3B5u32);
        if found2 == 0 {
            return 0;
        }
        let slot = this.wrapping_add(SLOT);
        if lf_checker_rt::callee_thiscall!(SEQPOSE_CALLEE, u32, found2, slot) & 0xFF == 0 {
            return 0;
        }
        wrf(slot.wrapping_add(8), add(rdf(slot.wrapping_add(8)), ONE));
        let anchor = rd32(ped.wrapping_add(PED_ANCHOR));
        let dx = sub(rdf(slot), rdf(anchor.wrapping_add(0x30)));
        let dy = sub(rdf(slot.wrapping_add(4)), rdf(anchor.wrapping_add(0x34)));
        let dz = sub(rdf(slot.wrapping_add(8)), rdf(anchor.wrapping_add(0x38)));
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        if !(dist2 > FAR2) {
            return 0;
        }
        let mut face = [0f32; 3];
        face[0] = sub(rdf(slot), rdf(anchor.wrapping_add(0x30)));
        face[1] = sub(rdf(slot.wrapping_add(4)), rdf(anchor.wrapping_add(0x34)));
        face[2] = 0.0;
        lf_checker_rt::callee_thiscall!(FACE_CALLEE, u32, face.as_mut_ptr() as u32);
        let dot = add(
            add(
                mul(rdf(anchor.wrapping_add(0x14)), face[1]),
                mul(rdf(anchor.wrapping_add(0x10)), face[0]),
            ),
            mul(rdf(anchor.wrapping_add(0x18)), face[2]),
        );
        if !(DOT_LIM > dot) {
            return 0;
        }
        if !(dot > 0.0) {
            return 0;
        }
        flag_task(this);
        request(ped.wrapping_add(PED_REQ), lf_checker_rt::relocated(REQ_C), 0, slot);
        1
    }
});
