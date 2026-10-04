// original: 0x00b29010 object_record_update
/// Refresh an object from a packed record and run its update stages.
///
/// Decodes the record into a scratch frame through the record decoder,
/// folds a caller-supplied triple into the decoded tail when any of its
/// components is nonzero, then runs three update stages on the object
/// (prepare, setup with the decoded frame, refresh) followed by a
/// virtual notification. Scales three signed record bytes into object
/// fields, mirrors a record flag bit into an object flag, and either
/// clears two scaled fields and finishes with a zero block (when the
/// mode byte is set) or scales three record words into the finish block.
/// Clears two status bits when the object state word equals 1. Returns
/// the finish stage's answer.
export!(cdecl, rw_00b29010(obj: u32, rec: u32, flag: u32, triple: u32) -> u32 {
    unsafe {
        const SCALE_A: u32 = 0x00FE876C;
        const SCALE_B: u32 = 0x00FE870C;
        const SCALE_W: u32 = 0x00EAC688;
        const VT_NOTIFY: u32 = 0xB4;
        let ka: f32 = *global::<f32>(SCALE_A);
        let kb: f32 = *global::<f32>(SCALE_B);
        let kw: f32 = *global::<f32>(SCALE_W);
        let decode: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let prepare: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let setup: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let refresh: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(4) as usize);
        let finish: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(6) as usize);

        let mut frame = [0.0f32; 15];
        decode(frame.as_mut_ptr() as u32, rec);
        let cx = *(triple as *const f32);
        let cy = *(triple.wrapping_add(4) as *const f32);
        let cz = *(triple.wrapping_add(8) as *const f32);
        if cx != 0.0 || cy != 0.0 || cz != 0.0 {
            frame[12] += (frame[4] * cy + frame[0] * cx) + frame[8] * cz;
            frame[13] += (frame[5] * cy + frame[1] * cx) + frame[9] * cz;
            frame[14] += (frame[6] * cy + frame[2] * cx) + frame[10] * cz;
        }
        let mut small = [0.0f32; 4];
        prepare(obj, small.as_mut_ptr() as u32);
        setup(obj, frame.as_mut_ptr() as u32, 0, 0);
        refresh(obj);
        let vt = *(obj as *const u32);
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(VT_NOTIFY)) as *const u32) as usize);
        notify(obj, 1);

        let b0 = *(rec.wrapping_add(0x10) as *const i8) as f32;
        let b1 = *(rec.wrapping_add(0x11) as *const i8) as f32;
        let b2 = *(rec.wrapping_add(0x12) as *const i8) as f32;
        *(obj.wrapping_add(0x1088) as *mut f32) = b0 * ka;
        *(obj.wrapping_add(0x1078) as *mut f32) = b1 * kb;
        *(obj.wrapping_add(0x107C) as *mut f32) = b2 * kb;
        if *(rec.wrapping_add(0x13) as *const u8) != 0 {
            *(obj.wrapping_add(0xF14) as *mut u8) |= 0x80;
        } else {
            *(obj.wrapping_add(0xF14) as *mut u8) &= 0x7F;
        }

        let r = if (flag & 0xFF) != 0 {
            *(obj.wrapping_add(0x1078) as *mut u32) = 0;
            *(obj.wrapping_add(0x107C) as *mut u32) = 0;
            small[0] = 0.0;
            small[1] = 0.0;
            small[2] = 0.0;
            let r = finish(obj, small.as_mut_ptr() as u32);
            *(obj.wrapping_add(0xF14) as *mut u8) &= 0x7F;
            r
        } else {
            let w0 = *(rec.wrapping_add(4) as *const i16) as f32;
            let w1 = *(rec.wrapping_add(6) as *const i16) as f32;
            let w2 = *(rec.wrapping_add(8) as *const i16) as f32;
            small[0] = w0 * kw;
            small[1] = w1 * kw;
            small[2] = w2 * kw;
            finish(obj, small.as_mut_ptr() as u32)
        };
        if *(obj.wrapping_add(0x1300) as *const u32) == 1 {
            *(obj.wrapping_add(0x1310) as *mut u8) &= 0xE7;
        }
        r
    }
});
