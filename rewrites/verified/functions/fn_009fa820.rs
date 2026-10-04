// original: 0x009fa820 network_socket_connect
/// Open a stream socket and connect it to the given address.
///
/// Creates a TCP socket for the family in the argument's leading word,
/// probes it, and connects. Any failure closes the socket and returns -1;
/// success returns the open socket.
export!(cdecl, rw_009fa820(addr: u32) -> u32 {
    unsafe {
        const SOCK_STREAM: u32 = 1;
        const IPPROTO_TCP: u32 = 6;
        const INVALID_SOCKET: u32 = 0xFFFFFFFF;
        const READY_TICK: u32 = 0x2733;
        const TUNE_OP: u32 = 0x1b58;
        const ADDR_LEN: u32 = 0x10;
        type SocketFn = extern "stdcall" fn(u32, u32, u32) -> u32;
        type CloseFn = extern "stdcall" fn(u32) -> u32;
        let family = (addr as *const i16).read() as i32 as u32;
        let socket: SocketFn = core::mem::transmute(global::<u32>(0x00E734B4).read());
        let s = socket(family, SOCK_STREAM, IPPROTO_TCP);
        if s == INVALID_SOCKET {
            return INVALID_SOCKET;
        }
        if callee_cdecl!(4, u32, s, 0) != 0 {
            callee_cdecl!(5, u32,);
            let close: CloseFn = core::mem::transmute(global::<u32>(0x00E734CC).read());
            close(s);
            return INVALID_SOCKET;
        }
        let connect: SocketFn = core::mem::transmute(global::<u32>(0x00E734E8).read());
        if connect(s, addr, ADDR_LEN) == 0 {
            return s;
        }
        if callee_cdecl!(5, u32,) != READY_TICK {
            let close: CloseFn = core::mem::transmute(global::<u32>(0x00E734CC).read());
            close(s);
            return INVALID_SOCKET;
        }
        let tuned = callee_cdecl!(6, u32, s, TUNE_OP);
        if tuned == 0 {
            let close: CloseFn = core::mem::transmute(global::<u32>(0x00E734CC).read());
            close(s);
            return INVALID_SOCKET;
        }
        if (tuned as i32) >= 0 {
            return s;
        }
        callee_cdecl!(5, u32,);
        let close: CloseFn = core::mem::transmute(global::<u32>(0x00E734CC).read());
        close(s);
        INVALID_SOCKET
    }
});
