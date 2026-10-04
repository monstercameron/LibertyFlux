// original: 0x00875b60 rage::crmtNodeExpression::vf4
/// Scan an expression's operands and notify the sink on the first live one.
///
/// thiscall/1 (`rage::crmtNodeExpression::vf4`). Walks the operand table
/// attached to the node; the first operand whose flag byte is set is
/// reported to the sink through slot 17 of the sink's vtable, passing the
/// table. An empty or all-clear table notifies nothing.
///
/// Note: the count word and the second operand slot are the same address
/// in the original, and this rewrite reads them the same way, so a table
/// with a count of two and a clear first operand faults exactly like the
/// original does.
export!(thiscall, rw_00875b60(this: *mut u8, sink: u32) -> () {
    unsafe {
        let table = *(this.add(0x20) as *const u32);
        if table == 0 {
            return;
        }
        let count = *((table as *const u8).add(0x0c) as *const i32);
        if count <= 0 {
            return;
        }
        let mut index = 0i32;
        while index < count {
            let operand = *((table + 8) as *const u32).add(index as usize);
            if *(operand as *const u8) != 0 {
                let vtable = *(sink as *const u32);
                let notify: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(
                        *((vtable as *const u8).add(0x44) as *const u32) as usize,
                    );
                notify(sink, table);
                return;
            }
            index += 1;
        }
    }
});
