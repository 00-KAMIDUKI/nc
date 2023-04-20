/// Lock a semaphore.
pub unsafe fn _ksem_wait(id: intptr_t) -> Result<(), Errno> {
    let id = id as usize;
    syscall1(SYS__KSEM_WAIT, id).map(drop)
}
