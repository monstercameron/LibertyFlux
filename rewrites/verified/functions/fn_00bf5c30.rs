// original: 0x00bf5c30 bind_tyre_fire_audio
/// Bind tyre-fire audio (speed/timer) for a vehicle model entry.
///
/// Looks up a wheel slot from a mode field, resolves an effect object for
/// it, then gates on a level value: zero pushes the quiet speed/timer pair,
/// nonzero probes the object and may format a per-object value. The entry's
/// model info is read through the shared table, a helper is picked through
/// two small tables, and when the lookup reports ready a scaled effect
/// value (halved and negated, or zeroed for the flagged mode) is forwarded
/// with a status word copied into the object. Returns nothing.
export!(thiscall, rw_00bf5c30(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const FIRE_NAME: u32 = 0x00EBBD84;
        const SPEED_NAME: u32 = 0x00EBBD94;
        const TIMER_NAME: u32 = 0x00EBBD9C;
        const MANAGER: u32 = 0x01394D60;
        const MODEL_TABLE: u32 = 0x01295CD8;
        const HALF_NEG: f32 = -0.5;
        const UPDATE_SLOT: u32 = 0xEC;
        let get_slot: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let hash_name: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let find_obj: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let set_pair: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(5) as usize);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(6) as usize);
        let fill_tmp: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(7) as usize);
        let use_tmp: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(8) as usize);
        let pick: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(9) as usize);
        let attach: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(10) as usize);
        let register: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(11) as usize);
        let finish: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(12) as usize);
        let release: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(13) as usize);

        if arg1 == 0 {
            return 0;
        }
        let mode = ((*((this.wrapping_add(0x24)) as *const u8) as u32) >> 1) & 7;
        let slot = get_slot(arg1, mode + 9);
        // Scratch mirrors the original frame: the status byte sits just below
        // the saved `this`, so the pre-call word holds this's low bytes.
        let mut out = [this.wrapping_shl(8), 0, 0];
        let hashed = hash_name(
            relocated(FIRE_NAME),
            0,
            out.as_mut_ptr() as u32,
        );
        let obj = find_obj(relocated(MANAGER), slot.wrapping_add(4), hashed);
        if obj == 0 {
            return 0;
        }
        let ready = *(out.as_ptr() as *const u8);
        let deep = core::ptr::read_unaligned(
            (out.as_ptr() as *const u8).wrapping_add(5) as *const u32,
        );
        let vtbl = *((arg1) as *const u32);
        let update: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtbl.wrapping_add(UPDATE_SLOT)) as *const u32) as usize);
        let mut scratch = [0u32; 4];
        let pos = update(arg1, scratch.as_mut_ptr() as u32);
        *((obj.wrapping_add(0x190)) as *mut u32) = *(pos as *const u32);
        *((obj.wrapping_add(0x194)) as *mut u32) = *((pos.wrapping_add(4)) as *const u32);
        *((obj.wrapping_add(0x198)) as *mut u32) = *((pos.wrapping_add(8)) as *const u32);
        if f32::from_bits(arg2) != 0.0 {
            if ready == 0 && (probe(this) as u8) != 0 {
                let mut tmp = 0u32;
                fill_tmp((&mut tmp as *mut u32) as u32, this);
                use_tmp(this, obj, tmp, arg2);
            }
        } else {
            let speed = *((this.wrapping_add(0x18)) as *const u32);
            set_pair(obj, relocated(SPEED_NAME), speed);
            let timer = *((this.wrapping_add(0x1C)) as *const u32);
            set_pair(obj, relocated(TIMER_NAME), timer);
        }
        let idx = *((arg1.wrapping_add(0x2E)) as *const i16) as i32;
        let entry = *((relocated(MODEL_TABLE).wrapping_add((idx as u32).wrapping_mul(4)))
            as *const u32);
        let table = *((entry.wrapping_add(0xCC)) as *const u32);
        let word = *((table
            .wrapping_add(mode.wrapping_mul(4))
            .wrapping_add(0x24)) as *const u32);
        let picked = pick(arg1, word);
        attach(obj, picked);
        register(relocated(MANAGER), obj, arg1, 0);
        if ready == 0 {
            return 0;
        }
        let flagged = *((arg1.wrapping_add(0x1304)) as *const u32) == 1;
        let base = f32::from_bits(*((deep.wrapping_add(0x10)) as *const u32));
        let scaled = if flagged { 0.0f32 } else { base * HALF_NEG };
        finish(obj, scaled.to_bits(), 0, 0);
        let status = *((deep.wrapping_add(8)) as *const u32);
        *((obj.wrapping_add(0x1EC)) as *mut u32) = status;
        release(obj);
        0
    }
});
