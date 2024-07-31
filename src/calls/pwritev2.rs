/// Write to a file descriptor without changing file offset.
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
/// let ret = unsafe { nc::readv(fd as usize, &mut iov) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
///
/// let path_out = "/tmp/nc-pwritev2";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path_out, nc::O_WRONLY | nc::O_CREAT, 0o644) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let flags = nc::RWF_DSYNC | nc::RWF_APPEND;
/// let ret = unsafe { nc::pwritev2(fd as usize, &iov, 0, iov.len() - 1, flags) };
/// assert!(ret.is_ok());
/// assert_eq!(ret, Ok(capacity as nc::ssize_t));
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// let ret = unsafe { nc::unlinkat(nc::AT_FDCWD, path_out, 0) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn pwritev2(
    fd: usize,
    vec: &[iovec_t],
    pos_l: usize,
    pos_h: usize,
    flags: rwf_t,
) -> Result<ssize_t, Errno> {
    let vec_ptr = vec.as_ptr() as usize;
    let vec_len = vec.len();
    let flags = flags as usize;
    syscall6(SYS_PWRITEV2, fd, vec_ptr, vec_len, pos_l, pos_h, flags).map(|ret| ret as ssize_t)
}
