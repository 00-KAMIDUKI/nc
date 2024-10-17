/// Get directory entries.
///
/// # Examples
///
/// ```rust
/// use std::mem::offset_of;
///
/// const BUF_SIZE: usize = 1 * 1024;
///
/// let path = "/etc";
/// let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_DIRECTORY, 0) };
/// assert!(ret.is_ok());
/// let fd = ret.unwrap();
/// let mut buf = [0; BUF_SIZE];
///
/// loop {
///     let ret = unsafe { nc::getdents(fd, &mut buf) };
///     assert!(ret.is_ok());
///     let nread = ret.unwrap() as usize;
///     if nread == 0 {
///         break;
///     }
///
///     let buf_ptr: *const u8 = buf.as_ptr();
///     let mut bpos: usize = 0;
///
///     println!("--------------- nread={nread} ---------------");
///     println!("inode#    file type  d_reclen  d_off   d_name");
///     while bpos < nread {
///         let d = buf_ptr.wrapping_add(bpos) as *mut nc::linux_dirent_t;
///         let d_ref: &nc::linux_dirent_t = unsafe { &(*d) };
///         let d_type = match d_ref.d_type() {
///             nc::DT_REG => "regular",
///             nc::DT_DIR => "directory",
///             nc::DT_FIFO => "FIFO",
///             nc::DT_SOCK => "socket",
///             nc::DT_LNK => "symlink",
///             nc::DT_BLK => "block-dev",
///             nc::DT_CHR => "char-dev",
///             _ => "unknown",
///         };
///
///         if let Ok(name) = std::str::from_utf8(d_ref.name()) {
///             println!(
///                 "{: >8}  {:<10} {: >4} {: >12}  {}",
///                 d_ref.d_ino, d_type, d_ref.d_reclen, d_ref.d_off as u32, name
///             );
///         } else {
///             eprintln!("Invalid name: {:?}", d_ref.name());
///         }
///
///         bpos += d_ref.d_reclen as usize;
///     }
///     break;
/// }
/// let offset = offset_of!(nc::linux_dirent_t, d_name);
/// println!("offset of d_name is: {}", offset);
/// println!(
///     "offset of d_reclen is: {}",
///     offset_of!(nc::linux_dirent_t, d_reclen)
/// );
///
/// let ret = unsafe { nc::close(fd) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn getdents(fd: i32, dir_buf: &mut [u8]) -> Result<ssize_t, Errno> {
    let fd = fd as usize;
    let count = dir_buf.len();
    let dir_buf_ptr = dir_buf.as_mut_ptr() as usize;
    syscall3(SYS_GETDENTS, fd, dir_buf_ptr, count).map(|ret| ret as ssize_t)
}
