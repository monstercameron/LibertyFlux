// original: 0x00bf5700 bind_train_spark_audio
/// Bind train spark/squeal audio for a vehicle model.
///
/// Resolves a spark-effect object for the given model entry, gates on a
/// level value (zero takes the quiet path, nonzero probes the object), casts
/// the entry's model info to vehicle model info, picks a helper through two
/// small tables, and when the lookup reports ready, fetches a position
/// triple through the entry's update method and forwards it with the
/// negated effect height. Returns nothing; all effects go through the
/// scripted helpers and the resolved object.
export!(thiscall, rw_00bf5700(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const SPARK_NAME: u32 = 0x00EBBDF0;
        const SQUEAL_NAME: u32 = 0x00EBBE04;
        const MANAGER: u32 = 0x01394D60;
        const MODEL_TABLE: u32 = 0x01295CD8;
        const SRC_TYPE: u32 = 0x0103B028;
        const DST_TYPE: u32 = 0x0103B0AC;
        const NEG_MASK: u32 = 0x8000_0000;
        const UPDATE_SLOT: u32 = 0xEC;
        let hash_name: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let find_obj: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let set_quiet: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(4) as usize);
        let fill_tmp: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(5) as usize);
        let use_tmp: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(6) as usize);
        let dyn_cast: extern "cdecl" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(7) as usize);
        let pick: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(8) as usize);
        let attach: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(9) as usize);
        let register: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(10) as usize);
        let finish: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(12) as usize);
        let release: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(13) as usize);

        if arg1 == 0 {
            return 0;
        }
        let tune = *((this.wrapping_add(0x1E)) as *const u8);
        let key = arg1.wrapping_add(0x0B).wrapping_add(tune as u32);
        let mut out = [0u32; 1];
        let hashed = hash_name(
            arg1,
            relocated(SPARK_NAME),
            0,
            out.as_mut_ptr() as u32,
        );
        let obj = find_obj(relocated(MANAGER), key, hashed);
        if obj == 0 {
            return 0;
        }
        let ready = *(out.as_ptr() as *const u8);
        if f32::from_bits(arg2) != 0.0 {
            if ready == 0 && (probe(this) as u8) != 0 {
                let mut slot = 0u32;
                fill_tmp((&mut slot as *mut u32) as u32, this);
                use_tmp(this, obj, slot, arg2);
            }
        } else {
            let level = *((this.wrapping_add(0x18)) as *const u32);
            set_quiet(obj, relocated(SQUEAL_NAME), level);
        }
        let idx = *((arg1.wrapping_add(0x2E)) as *const i16) as i32;
        let entry = *((relocated(MODEL_TABLE).wrapping_add((idx as u32).wrapping_mul(4)))
            as *const u32);
        let casted =
            dyn_cast(entry, 0, relocated(SRC_TYPE), relocated(DST_TYPE), 0);
        let c1 = *((this.wrapping_add(0x1D)) as *const u8) as u32;
        let c2 = *((this.wrapping_add(0x1E)) as *const u8) as u32;
        let table = *((casted.wrapping_add(0xCC)) as *const u32);
        let word = *((table.wrapping_add((c1 + c2 * 2).wrapping_mul(4))) as *const u32);
        let picked = if (word as i32) >= 0 { pick(arg1, word) } else { 0 };
        attach(obj, picked);
        register(relocated(MANAGER), obj, arg1, 0);
        if ready == 0 {
            return 0;
        }
        if *((this.wrapping_add(0x1C)) as *const u8) != 0 {
            *((obj.wrapping_add(0x1E2)) as *mut u8) = 1;
        }
        let height = f32::from_bits(*((casted.wrapping_add(0x7C)) as *const u32));
        let neg = height.to_bits() ^ NEG_MASK;
        let vtbl = *((arg1) as *const u32);
        let update: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtbl.wrapping_add(UPDATE_SLOT)) as *const u32) as usize);
        let mut scratch = [0u32; 4];
        let pos = update(arg1, scratch.as_mut_ptr() as u32);
        *((obj.wrapping_add(0x190)) as *mut u32) = *(pos as *const u32);
        *((obj.wrapping_add(0x194)) as *mut u32) = *((pos.wrapping_add(4)) as *const u32);
        *((obj.wrapping_add(0x198)) as *mut u32) = *((pos.wrapping_add(8)) as *const u32);
        let triple = [0u32, 0u32, neg];
        finish(obj, triple.as_ptr() as u32);
        release(obj);
        0
    }
});
