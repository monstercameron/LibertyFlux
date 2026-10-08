/// Proof scope: Entry kinds zero through ten and six nonzero divisors are covered.
/// Allocation failure and divisor zero are untested; helper bodies are scripted.
/// Full-data snapshot is disabled. Earlier build-contract identity differs for an unknown reason; actual run receipts govern this proof.
use lf_checker_rt as lf_k2_rt;
// original: 0x00894800 create_and_start_entry
/// Allocate and initialize an entry, store its value byte, then dispatch its
/// virtual start method and return the allocated object.
lf_k2_rt::export!(thiscall, rw_a_q136_00894800(
    _this: u32,
    value: u32,
    entry: u32,
    method_arg: u32,
) -> u32 {
    let entry_kind = unsafe { *((entry + 4) as *const u8) as u32 };
    let object = lf_k2_rt::callee_thiscall!(
        1, u32, lf_k2_rt::relocated(0x0115D8A0), 0xF0u32, entry_kind, 0u32
    );
    if object == 0 {
        return 0;
    }

    let _constructor_result = lf_k2_rt::callee_thiscall!(2, u32, object);
    let stored_value = if value == 0 {
        0xFFu8
    } else {
        const ROW_STRIDE: u32 = 0x6F40;
        const TABLE_BIAS: u32 = 0x6F10;
        let table_base = unsafe { lf_k2_rt::global::<u32>(0x0115D988).read() };
        let row_offset = entry_kind
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(TABLE_BIAS);
        let table_value = unsafe { *((table_base.wrapping_add(row_offset)) as *const u32) };
        let divisor = unsafe { lf_k2_rt::global::<u32>(0x0115D964).read() };
        ((value.wrapping_sub(table_value) / divisor) & 0xFF) as u8
    };

    unsafe { *((object + 5) as *mut u8) = stored_value; }
    let vtable = unsafe { *(object as *const u32) };
    let method_address = unsafe { *((vtable + 0x1C) as *const u32) };
    let method: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(method_address as usize) };
    let _method_result = method(object, 0, entry, method_arg);
    unsafe { *((object + 0x38) as *mut u8) |= 1; }
    object
});
