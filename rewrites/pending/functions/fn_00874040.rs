// original: 0x00874040 crmt_forward_record_init3
// Forward a record to the three-word initializer as (record, word8,
// word12). (thiscall/1)
export!(thiscall, rw_00874040(this_ptr: u32, record: u32) -> () {
    unsafe {
        let fields = record as *const u32;
        let w8 = fields.add(2).read();
        let w12 = fields.add(3).read();
        callee_thiscall!(1, u32, this_ptr, record, w8, w12);
    }
});
