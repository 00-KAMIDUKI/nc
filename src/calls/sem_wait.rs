/// Decrement (lock) a semaphore.
pub unsafe fn sem_wait(sem: &mut sem_t) -> Result<(), Errno> {
    let sem_ptr = sem as *mut sem_t as usize;
    syscall1(SYS_SEM_WAIT, sem_ptr).map(drop)
}
