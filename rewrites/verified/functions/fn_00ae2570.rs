// original: 0x00ae2570 ui_node_attach_once
/// Attach the helper node to the object once flag `0x10` is present.
///
/// Does nothing unless bit `0x10` of the flag word at `+0x28` is set while
/// bit `0x20` is clear; then it resolves a node through the node source,
/// stores the object into it and sets bit `0x20`. Returns the flag word, or
/// the node on the attach path.
export!(cdecl, rw_00ae2570(obj: u32) -> u32 {
    unsafe {
        let fp = (obj + 0x28) as *mut u32;
        let v = fp.read();
        if v & 0x10 == 0 || v & 0x20 != 0 {
            return v;
        }
        let node = callee_thiscall!(1, u32, relocated(0x1615470), 0x10);
        (node as *mut u32).write(obj);
        fp.write(v | 0x20);
        node
    }
});
