use core::arch::asm;
use crate::SignalAction;

const SYSCALL_DUP: usize = 24;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_SHUTDOWN: usize = 130;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGPROCMASK: usize = 135;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

pub fn sys_dup(fd: usize) -> isize {
    syscall(SYSCALL_DUP, [fd, 0, 0])
}

pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(SYSCALL_OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd, 0, 0])
}

pub fn sys_pipe(pipe_fd: &mut [usize; 2]) -> isize {
    syscall(SYSCALL_PIPE, [pipe_fd.as_mut_ptr() as usize, 0, 0])
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

// system call used for giving up CPU, always return 0
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

// system call used for sending signal to other processes
pub fn sys_kill(pid: usize, signum: i32) -> isize {
    syscall(SYSCALL_KILL, [pid, signum as usize, 0])
}

// system call for set the action when signal is received
pub fn sys_sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *mut SignalAction,
) -> isize {
    syscall(
        SYSCALL_SIGACTION, 
        [signum as usize, action as usize, old_action as usize]
    )
}

// system call for shutdown machine
pub fn sys_shutdown(failure: usize) -> ! {
    syscall(SYSCALL_SHUTDOWN, [failure, 0, 0]);
    unreachable!();
}

// system call used for mask/unmask signals
pub fn sys_sigprocmask(mask: u32) -> isize {
    syscall(SYSCALL_SIGPROCMASK, [mask as usize, 0, 0])
}

// system call used for returning from signal handler
pub fn sys_sigreturn() -> isize {
    syscall(SYSCALL_SIGRETURN, [0, 0, 0])
}

// system call used for getting time in milliseconds
pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

// system call used for getting the pid of the process
pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

// system call used for fork current process
pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

// system call used for exec a new program
pub fn sys_exec(path: &str, args: &[*const u8]) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, args.as_ptr() as usize, 0])
}

// system call used for wait child process
pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
}