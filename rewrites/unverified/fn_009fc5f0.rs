// original: 0x009fc5f0 socket_ioctl_mode

/// Applies the inverse of the low byte of `requested_mode` through the
/// Winsock FIONBIO option. The original stores that normalized dword in its
/// second incoming stack slot and passes its address to `ioctlsocket`; this
/// rewrite uses a local dword, with the contract comparing its contents at
/// the intercepted call. The return value is the scripted API result.
lf_checker_rt::export!(cdecl, rw_009fc5f0(socket_handle: u32, requested_mode: u32) -> u32 {
    const FIONBIO: u32 = 0x8004_667e;
    let nonblocking_flag = u32::from((requested_mode as u8) == 0);
    let value_ptr = &nonblocking_flag as *const u32 as u32;
    lf_checker_rt::callee_stdcall!(0, u32, socket_handle, FIONBIO, value_ptr)
});
