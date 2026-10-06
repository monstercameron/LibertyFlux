// original: 0x009bb6e0 guarded_input_dispatch_a
/// Bind the active input record unless the holder chain is already empty.
///
/// `this` points to a holder whose first word links to an inner record.
/// When the link is null the function answers 0 at once. Otherwise control
/// continues into the shared tail routine (inlined here, since a rewrite
/// cannot jump into original code): when the inner record's first word is
/// zero, or its flag byte at offset 0x64 is zero, it also answers 0.
/// Otherwise it fires helper id 1 (thiscall, three stack words) four times,
/// once per slot of the record's argument array, then picks a selector word
/// from offset 0x7C (when the last array word is zero) or 0x80, and answers
/// 0 when the selector is zero. A non-zero selector addresses a table row;
/// the routine raises the global flag byte at 0x017ED94B, publishes the row
/// address at 0x017F58E0, and answers 0 when the row's tag word at offset
/// 0xC is zero. Otherwise it publishes the row's second word at 0x017F58E4,
/// fires helper id 2 (thiscall, four stack words), clears 0x017F58E8 and
/// answers 1.
///
/// Edge cases: a null link, a zero head word, a zero flag byte, a zero
/// selector and a zero tag word each answer 0 with no writes and, except
/// for the four helper calls that precede the selector test, no calls.
///
/// Original: thiscall, `this` in ECX, no stack arguments, answer in AL.
lf_checker_rt::export!(thiscall, rw_009bb6e0(this: u32) -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x017ed94b;
        const OUT_PTR: u32 = 0x017f58e0;
        const OUT_SEL: u32 = 0x017f58e4;
        const OUT_ZERO: u32 = 0x017f58e8;
        let inner = (this as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        let edi = inner;
        if (edi as *const u32).read_unaligned() == 0 {
            return 0;
        }
        if ((edi.wrapping_add(0x64)) as *const u8).read() == 0 {
            return 0;
        }
        let idx_src = ((edi.wrapping_add(4)) as *const u32).read_unaligned();
        let esi = ((idx_src.wrapping_add(0x14)) as *const u32).read_unaligned() << 2;
        let mut k: u32 = 0;
        while k < 4 {
            let arr = ((edi.wrapping_add(8)) as *const u32).read_unaligned();
            let cx68 = ((edi.wrapping_add(0x68)) as *const u32).read_unaligned();
            let a_arr = ((arr.wrapping_add(esi).wrapping_add(k.wrapping_mul(4))) as *const u32).read_unaligned();
            let a_slot = ((edi.wrapping_add(0x6c).wrapping_add(k.wrapping_mul(4))) as *const u32).read_unaligned();
            let _ans: u32 = lf_checker_rt::callee_thiscall!(1, u32, cx68, cx68.wrapping_add(0x30), a_slot, a_arr);
            k += 1;
        }
        let arr = ((edi.wrapping_add(8)) as *const u32).read_unaligned();
        let last = ((arr.wrapping_add(esi).wrapping_add(12)) as *const u32).read_unaligned();
        let sel = if last == 0 {
            ((edi.wrapping_add(0x7c)) as *const u32).read_unaligned()
        } else {
            ((edi.wrapping_add(0x80)) as *const u32).read_unaligned()
        };
        if sel == 0 {
            return 0;
        }
        let edx = ((edi.wrapping_add(0x68)) as *const u32).read_unaligned();
        let base = ((edx as *const u32).read_unaligned()).wrapping_sub(0x10);
        let row = base.wrapping_add(sel << 4);
        lf_checker_rt::global::<u8>(FLAG_BYTE).write(1);
        let tag = ((row.wrapping_add(0x0c)) as *const u16).read_unaligned();
        lf_checker_rt::global::<u32>(OUT_PTR).write_unaligned(row);
        if tag == 0 {
            return 0;
        }
        let sel2 = ((row.wrapping_add(8)) as *const u32).read_unaligned();
        lf_checker_rt::global::<u32>(OUT_SEL).write_unaligned(sel2);
        let _ans2: u32 = lf_checker_rt::callee_thiscall!(2, u32, sel2, edx.wrapping_add(0x10), edx.wrapping_add(0x18), edx.wrapping_add(0x30), edx.wrapping_add(8));
        lf_checker_rt::global::<u32>(OUT_ZERO).write_unaligned(0);
        1
    }
});
