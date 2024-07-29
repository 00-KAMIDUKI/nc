/// Read from a file descriptor without changing file offset.
///
/// # Examples
///
/// ```
/// use core::ffi::c_void;
///
/// let path = "/etc/passwd";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_RDONLY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [[0_u8; 64]; 4];
/// let capacity = 4 * 64;
/// let mut iov = Vec::with_capacity(buf.len());
/// for ref mut item in (&mut buf).iter() {
///     iov.push(nc::iovec_t {
///         iov_len: item.len(),
///         iov_base: item.as_ptr() as *const c_void,
///     });
/// }
/// let iov_len = iov.len();
/// let flags = 0;
/// let ret = unsafe { nc::preadv2(fd, &mut iov, 0, iov_len - 1, flags) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn preadv2(
    fd: i32,
    vec: &mut [iovec_t],
    pos_l: usize,
    pos_h: usize,
    flags: rwf_t,
) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let vec_ptr = vec.as_mut_ptr() as usize;
    let vec_len = vec.len();
    let flags = flags as usize;
    syscall6(SYS_PREADV2, fd, vec_ptr, vec_len, pos_l, pos_h, flags).map(|ret| ret as ssize_t)
}
