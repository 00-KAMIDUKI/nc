/// Attach the System V shared memory segment.
///
/// # Examples
///
/// ```
/// let size = 4 * nc::PAGE_SIZE;
/// let flags = nc::IPC_CREAT | nc::IPC_EXCL | 0o600;
/// let ret = unsafe { nc::shmget(nc::IPC_PRIVATE, size, flags) };
/// assert!(ret.is_ok());
/// let shmid = ret.unwrap();
///
/// let addr: usize = 0;
/// let ret = unsafe { nc::shmat(shmid, addr, 0) };
/// assert!(ret.is_ok());
/// let addr = ret.unwrap();
///
/// let mut buf = nc::shmid_ds_t::default();
/// let ret = unsafe { nc::shmctl(shmid, nc::IPC_STAT, &mut buf) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::shmdt(addr) };
/// assert!(ret.is_ok());
///
/// let ret = unsafe { nc::shmctl(shmid, nc::IPC_RMID, &mut buf) };
/// assert!(ret.is_ok());
/// ```
pub unsafe fn shmat(shmid: i32, shmaddr: usize, shmflg: i32) -> Result<usize, Errno> {
    let shmid = shmid as usize;
    let shmflg = shmflg as usize;
    syscall3(SYS_SHMAT, shmid, shmaddr, shmflg)
}
