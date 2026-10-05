// original: 0x00a47a10 vehicle_model_part_lookup
/// Look up a model part id and hand it to the part loader.
///
/// Zeroes three dwords at `out`, reads the part id at `tab2[index]`
/// (`tab2` is at row offset 0xCC). A -1 id returns 0; otherwise the callee
/// (stdcall/2) runs on `(part, out)` and 1 returns (thiscall, two stack
/// arguments). Only AL is compared.
export!(thiscall, rw_00a47a10(this: u32, index: u32, out: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        (out as *mut u32).write_unaligned(0);
        (out.wrapping_add(4) as *mut u32).write_unaligned(0);
        (out.wrapping_add(8) as *mut u32).write_unaligned(0);
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let tab2 = (row.wrapping_add(0xcc) as *const u32).read_unaligned();
        let part = (tab2.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if part == 0xffffffff {
            return 0;
        }
        let _: u32 = callee_stdcall!(1, u32, part, out);
        1
    }
});
