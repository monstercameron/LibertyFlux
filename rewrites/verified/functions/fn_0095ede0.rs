// original: 0x0095EDE0 emit_gated_float_record_32

/// Emit a gated 32-byte float-bearing record. Its first output byte controls
/// the length returned by the intercepted helper and therefore the cursor move.
lf_checker_rt::export!(cdecl, rw_0095ede0(a0: u32, a1: u32, a2: u32, float_bits: u32) -> u32 {
    unsafe {
const READY: u32 = 1;
const LOCK: u32 = 2;
const LOCK_OBJECT: u32 = 0x0120_f270;
const CURSOR: u32 = 0x011f_a008;
const BUFFER: u32 = 0x011f_a00c;

#[inline(always)]
unsafe fn read_global_u32(address: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(address).read_unaligned() }
}

#[inline(always)]
unsafe fn write_global_u32(address: u32, value: u32) {
    unsafe { lf_checker_rt::global::<u32>(address).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_memory_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn table_output_pointer() -> u32 {
    unsafe { read_global_u32(0x0120_f294).wrapping_add(read_global_u32(0x0120_f290)) }
}

#[inline(always)]
unsafe fn advance_table_cursor(bytes: u32) {
    unsafe {
        let cursor = read_global_u32(0x0120_f290).wrapping_add(bytes);
        write_global_u32(0x0120_f290, cursor);
    }
}

#[inline(always)]
unsafe fn ready() -> bool {
    unsafe { lf_checker_rt::callee_cdecl!(READY, u32,) as u8 != 0 }
}

#[inline(always)]
unsafe fn lock() {
    let _ = unsafe { lf_checker_rt::callee_thiscall!(LOCK, u32, lf_checker_rt::relocated(LOCK_OBJECT)) };
}

#[inline(always)]
unsafe fn unlock_with(id: u32) {
    let _ = unsafe { lf_checker_rt::callee_thiscall!(id, u32, lf_checker_rt::relocated(LOCK_OBJECT)) };
}

#[inline(always)]
unsafe fn reserve(bytes: u32, id: u32) -> bool {
    unsafe { lf_checker_rt::callee_cdecl!(id, u32, bytes) as u8 != 0 }
}

#[inline(always)]
unsafe fn output_pointer() -> u32 {
    unsafe { read_global_u32(BUFFER).wrapping_add(read_global_u32(CURSOR)) }
}

#[inline(always)]
unsafe fn finish_fixed(bytes: u32) {
    unsafe {
        let cursor = read_global_u32(CURSOR).wrapping_add(bytes);
        write_global_u32(CURSOR, cursor);
        (read_global_u32(BUFFER).wrapping_add(cursor) as *mut u8).write(0);
    }
}

#[inline(always)]
unsafe fn begin_stateful_cursor(state_id: u32) -> bool {
    unsafe {
        if (lf_checker_rt::callee_cdecl!(state_id, u32,) as u8) == 0 { return false; }
        lock();
        true
    }
}

#[inline(always)]
unsafe fn state_gate(state_id: u32) -> bool {
    unsafe {
        const FIRST_FLAG: u32 = 0x0103_76e9;
        const SECOND_FLAG: u32 = 0x011f_7076;
        const OPTIONAL_FLAG: u32 = 0x0103_7868;
        const OPTIONAL_VALUE: u32 = 0x011f_6ffa;
        const MODE: u32 = 0x0120_88b4;
        const EXPECTED_MODE: u32 = 0x00f1_c040;
        if lf_checker_rt::global::<u8>(FIRST_FLAG).read() == 0 { return false; }
        if lf_checker_rt::global::<u8>(SECOND_FLAG).read() != 0 { return false; }
        if (lf_checker_rt::callee_cdecl!(state_id, u32,) as u8) == 0 { return false; }
        if lf_checker_rt::global::<u8>(OPTIONAL_FLAG).read() != 0
            && lf_checker_rt::global::<u8>(OPTIONAL_VALUE).read() != 2 { return false; }
        lf_checker_rt::global::<u32>(MODE).read_unaligned()
            == lf_checker_rt::global::<u32>(EXPECTED_MODE).read_unaligned()
    }
}

#[inline(always)]
unsafe fn within_cursor_limit(size: u32, overflow_id: u32) -> bool {
    unsafe {
        const LIMIT: u32 = 0x0089_5430;
        let (end, carry) = read_global_u32(CURSOR).overflowing_add(size);
        carry || end <= LIMIT || (lf_checker_rt::callee_cdecl!(overflow_id, u32,) as u8 != 0)
    }
}

#[inline(always)]
unsafe fn finish_dynamic(length_id: u32, output: u32) {
    unsafe {
        let first_byte = (output as *const u8).read();
        let length = lf_checker_rt::callee_cdecl!(length_id, u32, u32::from(first_byte));
        let cursor = read_global_u32(CURSOR).wrapping_add(length);
        write_global_u32(CURSOR, cursor);
        (read_global_u32(BUFFER).wrapping_add(cursor) as *mut u8).write(0);
    }
}

#[inline(always)]
unsafe fn initialize_entity_tag(entity: u32, hash_id: u32) {
    unsafe {
        const ENTITY_TAG_OFFSET: u32 = 0x64;
        let field = entity.wrapping_add(ENTITY_TAG_OFFSET) as *mut u32;
        if field.read_unaligned() == u32::MAX {
            field.write_unaligned(lf_checker_rt::callee_cdecl!(hash_id, u32, entity));
        }
    }
}

        if !state_gate(1) { return 0; }
        lock();
        if !within_cursor_limit(0x20, 3) { unlock_with(6); return 0; }
        let out = output_pointer();
        let _ = lf_checker_rt::callee_thiscall!(4, u32, out, a0, a1, a2, float_bits);
        finish_dynamic(5, out);
        unlock_with(6);
        0
    }
});
