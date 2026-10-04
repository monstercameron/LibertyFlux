// original: 0x0099f830 audio_release_slots
/// Release the four voice slots held by an audio object.
///
/// Each slot holds either null (left alone) or a pointer to a voice record.
/// The first two slots may be released through the engine release routine
/// when the global release-all switch reads 2 and the record's tag word is
/// not 2; every other live record is released by setting a bit in a shared
/// flag table indexed by the record's two key bytes. Every consumed slot is
/// cleared to null. Returns nothing.
export!(thiscall, rw_0099f830(this: u32) -> () {
    unsafe {
        const RELEASE_ALL: u32 = 0x11D6FD4;
        const FLAG_STRIDE: u32 = 0x115D968;
        const FLAG_TABLE: u32 = 0x115D988;
        const TABLE_COL_STRIDE: u32 = 0x6F40;
        const TABLE_BIAS: u32 = 0x6F14;
        const FLAG_BYTE_OFF: u32 = 0xE8;
        const FLAG_BIT: u8 = 0x10;
        // Set the shared-table flag for one live voice record.
        let flag_voice = |record: u32| unsafe {
            let row = ((record + 4) as *const u8).read();
            let target = if row == 0xFF {
                0
            } else {
                let stride = global::<u32>(FLAG_STRIDE).read();
                let table = global::<u32>(FLAG_TABLE).read();
                let col = ((record + 0x40) as *const u8).read() as u32;
                let cell = table
                    .wrapping_add(col.wrapping_mul(TABLE_COL_STRIDE))
                    .wrapping_add(TABLE_BIAS);
                stride
                    .wrapping_mul(row as u32)
                    .wrapping_add((cell as *const u32).read())
            };
            let flag = target.wrapping_add(FLAG_BYTE_OFF) as *mut u8;
            flag.write(flag.read() | FLAG_BIT);
        };
        let release_all = global::<u32>(RELEASE_ALL).read() == 2;
        for slot in [0x94u32, 0x98u32] {
            let cell = (this + slot) as *mut u32;
            let record = cell.read();
            if record != 0 {
                if release_all && ((record + 6) as *const u16).read() != 2 {
                    callee_thiscall!(1, u32, record, 0);
                } else {
                    flag_voice(record);
                }
                cell.write(0);
            }
        }
        for slot in [0xB0u32, 0xB4u32] {
            let cell = (this + slot) as *mut u32;
            let record = cell.read();
            if record != 0 {
                flag_voice(record);
                cell.write(0);
            }
        }
    }
});
