// original: 0x00AFFEC0 viewport_pick_target
/// Viewport target picker: asks the UI gate whether picking is allowed, takes
/// the active camera rig, rejects it when its motion vector is too short,
/// then scans the three pick boxes for one containing the focus point and
/// facing the rig. Writes the winning box index, or -1 when nothing qualifies.
///
/// The function is void; the low byte of the value in EAX on return is the
/// only return channel compared (the rest is callee residue on one path).
/// Every `jbe` after a `comiss` is "not strictly greater", which is true for
/// NaN, so branches use `!(x > y)` rather than `x <= y`.
///
/// Original: 0x00AFFEC0 (cdecl/0).
const PICK_RESULT: u32 = 0x0160_031C;
const MIN_LEN2: u32 = 0x00FE_8B00;
const PICK_TABLE: u32 = 0x0160_0FD8;
const PICK_ROWS: u32 = 3;
const LENS_SLOT: u32 = 0xEC;

export!(cdecl, rw_00affec0() -> u32 {
    unsafe {
        global::<u32>(PICK_RESULT).write(0xFFFF_FFFF);
        let gate: u32 = callee_cdecl!(1, u32,);
        if gate & 0xFF != 0 {
            return gate;
        }
        let rig: u32 = callee_cdecl!(2, u32, 0);
        if rig == 0 {
            return 0;
        }
        let mode = ((rig + 0x1304) as *const u32).read();
        if mode != 0 && mode != 1 {
            return mode;
        }
        // Lens call through the rig's table; the frame slot is an out-slot
        // whose content the caller never reads back.
        let table = (rig as *const u32).read();
        let lens: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((table + LENS_SLOT) as *const u32).read()) as usize);
        let mut slot = 0u32;
        let motion = lens(rig, (&mut slot as *mut u32) as u32);
        let mx = ((motion + 0) as *const f32).read();
        let my = ((motion + 4) as *const f32).read();
        let mz = ((motion + 8) as *const f32).read();
        let len2 = mx * mx + my * my;
        let len2 = len2 + mz * mz;
        let min = (global::<f32>(MIN_LEN2) as *const f32).read();
        if !(len2 > min) {
            return motion;
        }
        let holder = ((rig + 0x20) as *const u32).read();
        let px = ((holder + 0x30) as *const f32).read();
        let py = ((holder + 0x34) as *const f32).read();
        let pz = ((holder + 0x38) as *const f32).read();
        // From here EAX holds the holder pointer until a face call answers.
        let mut last = holder;
        for row in 0..PICK_ROWS {
            let base = PICK_TABLE.wrapping_add(row.wrapping_mul(0x30));
            let rd = |off: u32| (global::<f32>(base.wrapping_sub(off)) as *const f32).read();
            if !(px > rd(0x28)) {
                continue;
            }
            if !(rd(0x18) > px) {
                continue;
            }
            if !(py > rd(0x24)) {
                continue;
            }
            if !(rd(0x14) > py) {
                continue;
            }
            if !(pz > rd(0x20)) {
                continue;
            }
            if !(rd(0x10) > pz) {
                continue;
            }
            let dx = (global::<f32>(base.wrapping_sub(8)) as *const f32).read() - px;
            let dy = (global::<f32>(base.wrapping_sub(4)) as *const f32).read() - py;
            let dz = (global::<f32>(base) as *const f32).read() - pz;
            let face = lens(rig, (&mut slot as *mut u32) as u32);
            last = face;
            let nx = ((face + 0) as *const f32).read();
            let ny = ((face + 4) as *const f32).read();
            let nz = ((face + 8) as *const f32).read();
            let along_x = nx * dx;
            let mut dot = ny * dy;
            dot += along_x;
            dot += nz * dz;
            if dot > 0.0 {
                global::<u32>(PICK_RESULT).write(row);
                return face;
            }
        }
        last
    }
});
