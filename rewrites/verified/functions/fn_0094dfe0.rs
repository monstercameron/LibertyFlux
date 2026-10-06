// original: 0x0094DFE0 NativeImpl_REMOVE_TXD (symbols)

/// Detach and evict one 32-byte record selected through callee 1.
///
/// Calls callee 1 with (`key`, `KIND`). Its answer is a SIGNED index: a
/// negative answer (including 0x80000000 and -1) is returned unchanged and
/// nothing else happens. Otherwise the record at `this + index * 32` is
/// read: its dword at +4 is likewise SIGNED, and a negative cell is returned
/// unchanged. A live record (flag byte at +0 non-zero) is detached through
/// callee 2 (the cell value) and evicted through callee 3 (thiscall, the
/// constant `CTX` in ECX, the cell value on the stack). The record is then
/// cleared (byte +0, dword +4 = -1, byte +8) and callee 3's answer — or the
/// cell value when the flag byte was already zero — is returned.
///
/// Original: 0x0094DFE0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0094DFE0(this: u32, key: u32) -> u32 {
    unsafe {
        const KIND: u32 = 9;
        const CTX: u32 = 0x11DAAB0;
        const FIND: u32 = 1;
        const DETACH: u32 = 2;
        const EVICT: u32 = 3;
        const EMPTY: u32 = 0xFFFF_FFFF;
        let r = lf_checker_rt::callee_cdecl!(FIND, u32, key, KIND);
        if (r as i32) < 0 {
            return r;
        }
        let rec = this.wrapping_add(r.wrapping_mul(32));
        let cell = (rec.wrapping_add(4) as *const u32).read_unaligned();
        if (cell as i32) < 0 {
            return cell;
        }
        let live = (rec as *const u8).read_unaligned();
        let ans = if live != 0 {
            lf_checker_rt::callee_cdecl!(DETACH, u32, cell);
            let cell2 = (rec.wrapping_add(4) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(EVICT, u32, lf_checker_rt::relocated(CTX), cell2)
        } else {
            cell
        };
        (rec as *mut u8).write(0);
        (rec.wrapping_add(4) as *mut u32).write_unaligned(EMPTY);
        (rec.wrapping_add(8) as *mut u8).write(0);
        ans
    }
});
