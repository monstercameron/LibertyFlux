// original: 0x00d69590 bind_aux_and_combine
// s16f00: attach an auxiliary record, then combine a counter (thiscall/1).
//
// Looks up the shared table (`lookup(0)`), follows its record link,
// publishes the record on this object, runs an auxiliary step over one
// of the record's fields, then combines
// this object's 16-bit counter with the step's answer through a third call.
// The stack argument is accepted for the signature but never read. Returns
// the combine step's answer.
export!(thiscall, rw_s16f00(this: *mut u8, _arg: u32) -> u32 {
    unsafe {
        let shared = callee_cdecl!(1, u32, 0);
        let record = *((shared.wrapping_add(0xE98)) as *const u32);
        *((this.add(0x30)) as *mut u32) = record;
        let aux_input = *((record.wrapping_add(0x58)) as *const u32);
        let step_answer = callee_cdecl!(2, u32, aux_input);
        *this.add(0x2A) |= 8;
        let counter = *((this.add(0x18)) as *const u16) as u32;
        let total = counter.wrapping_add(step_answer);
        let out = callee_cdecl!(3, u32, this.add(0x18) as u32, total);
        *this.add(0x25) = 3;
        out
    }
});
