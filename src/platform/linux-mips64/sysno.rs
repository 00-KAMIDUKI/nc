// Code generated by mksysnum_linux.py; DO NOT EDIT.

use crate::syscalls::Sysno;

pub const SYS_READ: Sysno = 5000;
pub const SYS_WRITE: Sysno = 5001;
pub const SYS_OPEN: Sysno = 5002;
pub const SYS_CLOSE: Sysno = 5003;
pub const SYS_STAT: Sysno = 5004;
pub const SYS_FSTAT: Sysno = 5005;
pub const SYS_LSTAT: Sysno = 5006;
pub const SYS_POLL: Sysno = 5007;
pub const SYS_LSEEK: Sysno = 5008;
pub const SYS_MMAP: Sysno = 5009;
pub const SYS_MPROTECT: Sysno = 5010;
pub const SYS_MUNMAP: Sysno = 5011;
pub const SYS_BRK: Sysno = 5012;
pub const SYS_RT_SIGACTION: Sysno = 5013;
pub const SYS_RT_SIGPROCMASK: Sysno = 5014;
pub const SYS_IOCTL: Sysno = 5015;
pub const SYS_PREAD64: Sysno = 5016;
pub const SYS_PWRITE64: Sysno = 5017;
pub const SYS_READV: Sysno = 5018;
pub const SYS_WRITEV: Sysno = 5019;
pub const SYS_ACCESS: Sysno = 5020;
pub const SYS_PIPE: Sysno = 5021;
pub const SYS__NEWSELECT: Sysno = 5022;
pub const SYS_SCHED_YIELD: Sysno = 5023;
pub const SYS_MREMAP: Sysno = 5024;
pub const SYS_MSYNC: Sysno = 5025;
pub const SYS_MINCORE: Sysno = 5026;
pub const SYS_MADVISE: Sysno = 5027;
pub const SYS_SHMGET: Sysno = 5028;
pub const SYS_SHMAT: Sysno = 5029;
pub const SYS_SHMCTL: Sysno = 5030;
pub const SYS_DUP: Sysno = 5031;
pub const SYS_DUP2: Sysno = 5032;
pub const SYS_PAUSE: Sysno = 5033;
pub const SYS_NANOSLEEP: Sysno = 5034;
pub const SYS_GETITIMER: Sysno = 5035;
pub const SYS_SETITIMER: Sysno = 5036;
pub const SYS_ALARM: Sysno = 5037;
pub const SYS_GETPID: Sysno = 5038;
pub const SYS_SENDFILE: Sysno = 5039;
pub const SYS_SOCKET: Sysno = 5040;
pub const SYS_CONNECT: Sysno = 5041;
pub const SYS_ACCEPT: Sysno = 5042;
pub const SYS_SENDTO: Sysno = 5043;
pub const SYS_RECVFROM: Sysno = 5044;
pub const SYS_SENDMSG: Sysno = 5045;
pub const SYS_RECVMSG: Sysno = 5046;
pub const SYS_SHUTDOWN: Sysno = 5047;
pub const SYS_BIND: Sysno = 5048;
pub const SYS_LISTEN: Sysno = 5049;
pub const SYS_GETSOCKNAME: Sysno = 5050;
pub const SYS_GETPEERNAME: Sysno = 5051;
pub const SYS_SOCKETPAIR: Sysno = 5052;
pub const SYS_SETSOCKOPT: Sysno = 5053;
pub const SYS_GETSOCKOPT: Sysno = 5054;
pub const SYS_CLONE: Sysno = 5055;
pub const SYS_FORK: Sysno = 5056;
pub const SYS_EXECVE: Sysno = 5057;
pub const SYS_EXIT: Sysno = 5058;
pub const SYS_WAIT4: Sysno = 5059;
pub const SYS_KILL: Sysno = 5060;
pub const SYS_UNAME: Sysno = 5061;
pub const SYS_SEMGET: Sysno = 5062;
pub const SYS_SEMOP: Sysno = 5063;
pub const SYS_SEMCTL: Sysno = 5064;
pub const SYS_SHMDT: Sysno = 5065;
pub const SYS_MSGGET: Sysno = 5066;
pub const SYS_MSGSND: Sysno = 5067;
pub const SYS_MSGRCV: Sysno = 5068;
pub const SYS_MSGCTL: Sysno = 5069;
pub const SYS_FCNTL: Sysno = 5070;
pub const SYS_FLOCK: Sysno = 5071;
pub const SYS_FSYNC: Sysno = 5072;
pub const SYS_FDATASYNC: Sysno = 5073;
pub const SYS_TRUNCATE: Sysno = 5074;
pub const SYS_FTRUNCATE: Sysno = 5075;
pub const SYS_GETDENTS: Sysno = 5076;
pub const SYS_GETCWD: Sysno = 5077;
pub const SYS_CHDIR: Sysno = 5078;
pub const SYS_FCHDIR: Sysno = 5079;
pub const SYS_RENAME: Sysno = 5080;
pub const SYS_MKDIR: Sysno = 5081;
pub const SYS_RMDIR: Sysno = 5082;
pub const SYS_CREAT: Sysno = 5083;
pub const SYS_LINK: Sysno = 5084;
pub const SYS_UNLINK: Sysno = 5085;
pub const SYS_SYMLINK: Sysno = 5086;
pub const SYS_READLINK: Sysno = 5087;
pub const SYS_CHMOD: Sysno = 5088;
pub const SYS_FCHMOD: Sysno = 5089;
pub const SYS_CHOWN: Sysno = 5090;
pub const SYS_FCHOWN: Sysno = 5091;
pub const SYS_LCHOWN: Sysno = 5092;
pub const SYS_UMASK: Sysno = 5093;
pub const SYS_GETTIMEOFDAY: Sysno = 5094;
pub const SYS_GETRLIMIT: Sysno = 5095;
pub const SYS_GETRUSAGE: Sysno = 5096;
pub const SYS_SYSINFO: Sysno = 5097;
pub const SYS_TIMES: Sysno = 5098;
pub const SYS_PTRACE: Sysno = 5099;
pub const SYS_GETUID: Sysno = 5100;
pub const SYS_SYSLOG: Sysno = 5101;
pub const SYS_GETGID: Sysno = 5102;
pub const SYS_SETUID: Sysno = 5103;
pub const SYS_SETGID: Sysno = 5104;
pub const SYS_GETEUID: Sysno = 5105;
pub const SYS_GETEGID: Sysno = 5106;
pub const SYS_SETPGID: Sysno = 5107;
pub const SYS_GETPPID: Sysno = 5108;
pub const SYS_GETPGRP: Sysno = 5109;
pub const SYS_SETSID: Sysno = 5110;
pub const SYS_SETREUID: Sysno = 5111;
pub const SYS_SETREGID: Sysno = 5112;
pub const SYS_GETGROUPS: Sysno = 5113;
pub const SYS_SETGROUPS: Sysno = 5114;
pub const SYS_SETRESUID: Sysno = 5115;
pub const SYS_GETRESUID: Sysno = 5116;
pub const SYS_SETRESGID: Sysno = 5117;
pub const SYS_GETRESGID: Sysno = 5118;
pub const SYS_GETPGID: Sysno = 5119;
pub const SYS_SETFSUID: Sysno = 5120;
pub const SYS_SETFSGID: Sysno = 5121;
pub const SYS_GETSID: Sysno = 5122;
pub const SYS_CAPGET: Sysno = 5123;
pub const SYS_CAPSET: Sysno = 5124;
pub const SYS_RT_SIGPENDING: Sysno = 5125;
pub const SYS_RT_SIGTIMEDWAIT: Sysno = 5126;
pub const SYS_RT_SIGQUEUEINFO: Sysno = 5127;
pub const SYS_RT_SIGSUSPEND: Sysno = 5128;
pub const SYS_SIGALTSTACK: Sysno = 5129;
pub const SYS_UTIME: Sysno = 5130;
pub const SYS_MKNOD: Sysno = 5131;
pub const SYS_PERSONALITY: Sysno = 5132;
pub const SYS_USTAT: Sysno = 5133;
pub const SYS_STATFS: Sysno = 5134;
pub const SYS_FSTATFS: Sysno = 5135;
pub const SYS_SYSFS: Sysno = 5136;
pub const SYS_GETPRIORITY: Sysno = 5137;
pub const SYS_SETPRIORITY: Sysno = 5138;
pub const SYS_SCHED_SETPARAM: Sysno = 5139;
pub const SYS_SCHED_GETPARAM: Sysno = 5140;
pub const SYS_SCHED_SETSCHEDULER: Sysno = 5141;
pub const SYS_SCHED_GETSCHEDULER: Sysno = 5142;
pub const SYS_SCHED_GET_PRIORITY_MAX: Sysno = 5143;
pub const SYS_SCHED_GET_PRIORITY_MIN: Sysno = 5144;
pub const SYS_SCHED_RR_GET_INTERVAL: Sysno = 5145;
pub const SYS_MLOCK: Sysno = 5146;
pub const SYS_MUNLOCK: Sysno = 5147;
pub const SYS_MLOCKALL: Sysno = 5148;
pub const SYS_MUNLOCKALL: Sysno = 5149;
pub const SYS_VHANGUP: Sysno = 5150;
pub const SYS_PIVOT_ROOT: Sysno = 5151;
pub const SYS__SYSCTL: Sysno = 5152;
pub const SYS_PRCTL: Sysno = 5153;
pub const SYS_ADJTIMEX: Sysno = 5154;
pub const SYS_SETRLIMIT: Sysno = 5155;
pub const SYS_CHROOT: Sysno = 5156;
pub const SYS_SYNC: Sysno = 5157;
pub const SYS_ACCT: Sysno = 5158;
pub const SYS_SETTIMEOFDAY: Sysno = 5159;
pub const SYS_MOUNT: Sysno = 5160;
pub const SYS_UMOUNT2: Sysno = 5161;
pub const SYS_SWAPON: Sysno = 5162;
pub const SYS_SWAPOFF: Sysno = 5163;
pub const SYS_REBOOT: Sysno = 5164;
pub const SYS_SETHOSTNAME: Sysno = 5165;
pub const SYS_SETDOMAINNAME: Sysno = 5166;
pub const SYS_CREATE_MODULE: Sysno = 5167;
pub const SYS_INIT_MODULE: Sysno = 5168;
pub const SYS_DELETE_MODULE: Sysno = 5169;
pub const SYS_GET_KERNEL_SYMS: Sysno = 5170;
pub const SYS_QUERY_MODULE: Sysno = 5171;
pub const SYS_QUOTACTL: Sysno = 5172;
pub const SYS_NFSSERVCTL: Sysno = 5173;
pub const SYS_GETPMSG: Sysno = 5174;
pub const SYS_PUTPMSG: Sysno = 5175;
pub const SYS_AFS_SYSCALL: Sysno = 5176;
pub const SYS_RESERVED177: Sysno = 5177;
pub const SYS_GETTID: Sysno = 5178;
pub const SYS_READAHEAD: Sysno = 5179;
pub const SYS_SETXATTR: Sysno = 5180;
pub const SYS_LSETXATTR: Sysno = 5181;
pub const SYS_FSETXATTR: Sysno = 5182;
pub const SYS_GETXATTR: Sysno = 5183;
pub const SYS_LGETXATTR: Sysno = 5184;
pub const SYS_FGETXATTR: Sysno = 5185;
pub const SYS_LISTXATTR: Sysno = 5186;
pub const SYS_LLISTXATTR: Sysno = 5187;
pub const SYS_FLISTXATTR: Sysno = 5188;
pub const SYS_REMOVEXATTR: Sysno = 5189;
pub const SYS_LREMOVEXATTR: Sysno = 5190;
pub const SYS_FREMOVEXATTR: Sysno = 5191;
pub const SYS_TKILL: Sysno = 5192;
pub const SYS_RESERVED193: Sysno = 5193;
pub const SYS_FUTEX: Sysno = 5194;
pub const SYS_SCHED_SETAFFINITY: Sysno = 5195;
pub const SYS_SCHED_GETAFFINITY: Sysno = 5196;
pub const SYS_CACHEFLUSH: Sysno = 5197;
pub const SYS_CACHECTL: Sysno = 5198;
pub const SYS_SYSMIPS: Sysno = 5199;
pub const SYS_IO_SETUP: Sysno = 5200;
pub const SYS_IO_DESTROY: Sysno = 5201;
pub const SYS_IO_GETEVENTS: Sysno = 5202;
pub const SYS_IO_SUBMIT: Sysno = 5203;
pub const SYS_IO_CANCEL: Sysno = 5204;
pub const SYS_EXIT_GROUP: Sysno = 5205;
pub const SYS_LOOKUP_DCOOKIE: Sysno = 5206;
pub const SYS_EPOLL_CREATE: Sysno = 5207;
pub const SYS_EPOLL_CTL: Sysno = 5208;
pub const SYS_EPOLL_WAIT: Sysno = 5209;
pub const SYS_REMAP_FILE_PAGES: Sysno = 5210;
pub const SYS_RT_SIGRETURN: Sysno = 5211;
pub const SYS_SET_TID_ADDRESS: Sysno = 5212;
pub const SYS_RESTART_SYSCALL: Sysno = 5213;
pub const SYS_SEMTIMEDOP: Sysno = 5214;
pub const SYS_FADVISE64: Sysno = 5215;
pub const SYS_TIMER_CREATE: Sysno = 5216;
pub const SYS_TIMER_SETTIME: Sysno = 5217;
pub const SYS_TIMER_GETTIME: Sysno = 5218;
pub const SYS_TIMER_GETOVERRUN: Sysno = 5219;
pub const SYS_TIMER_DELETE: Sysno = 5220;
pub const SYS_CLOCK_SETTIME: Sysno = 5221;
pub const SYS_CLOCK_GETTIME: Sysno = 5222;
pub const SYS_CLOCK_GETRES: Sysno = 5223;
pub const SYS_CLOCK_NANOSLEEP: Sysno = 5224;
pub const SYS_TGKILL: Sysno = 5225;
pub const SYS_UTIMES: Sysno = 5226;
pub const SYS_MBIND: Sysno = 5227;
pub const SYS_GET_MEMPOLICY: Sysno = 5228;
pub const SYS_SET_MEMPOLICY: Sysno = 5229;
pub const SYS_MQ_OPEN: Sysno = 5230;
pub const SYS_MQ_UNLINK: Sysno = 5231;
pub const SYS_MQ_TIMEDSEND: Sysno = 5232;
pub const SYS_MQ_TIMEDRECEIVE: Sysno = 5233;
pub const SYS_MQ_NOTIFY: Sysno = 5234;
pub const SYS_MQ_GETSETATTR: Sysno = 5235;
pub const SYS_VSERVER: Sysno = 5236;
pub const SYS_WAITID: Sysno = 5237;
pub const SYS_ADD_KEY: Sysno = 5239;
pub const SYS_REQUEST_KEY: Sysno = 5240;
pub const SYS_KEYCTL: Sysno = 5241;
pub const SYS_SET_THREAD_AREA: Sysno = 5242;
pub const SYS_INOTIFY_INIT: Sysno = 5243;
pub const SYS_INOTIFY_ADD_WATCH: Sysno = 5244;
pub const SYS_INOTIFY_RM_WATCH: Sysno = 5245;
pub const SYS_MIGRATE_PAGES: Sysno = 5246;
pub const SYS_OPENAT: Sysno = 5247;
pub const SYS_MKDIRAT: Sysno = 5248;
pub const SYS_MKNODAT: Sysno = 5249;
pub const SYS_FCHOWNAT: Sysno = 5250;
pub const SYS_FUTIMESAT: Sysno = 5251;
pub const SYS_NEWFSTATAT: Sysno = 5252;
pub const SYS_UNLINKAT: Sysno = 5253;
pub const SYS_RENAMEAT: Sysno = 5254;
pub const SYS_LINKAT: Sysno = 5255;
pub const SYS_SYMLINKAT: Sysno = 5256;
pub const SYS_READLINKAT: Sysno = 5257;
pub const SYS_FCHMODAT: Sysno = 5258;
pub const SYS_FACCESSAT: Sysno = 5259;
pub const SYS_PSELECT6: Sysno = 5260;
pub const SYS_PPOLL: Sysno = 5261;
pub const SYS_UNSHARE: Sysno = 5262;
pub const SYS_SPLICE: Sysno = 5263;
pub const SYS_SYNC_FILE_RANGE: Sysno = 5264;
pub const SYS_TEE: Sysno = 5265;
pub const SYS_VMSPLICE: Sysno = 5266;
pub const SYS_MOVE_PAGES: Sysno = 5267;
pub const SYS_SET_ROBUST_LIST: Sysno = 5268;
pub const SYS_GET_ROBUST_LIST: Sysno = 5269;
pub const SYS_KEXEC_LOAD: Sysno = 5270;
pub const SYS_GETCPU: Sysno = 5271;
pub const SYS_EPOLL_PWAIT: Sysno = 5272;
pub const SYS_IOPRIO_SET: Sysno = 5273;
pub const SYS_IOPRIO_GET: Sysno = 5274;
pub const SYS_UTIMENSAT: Sysno = 5275;
pub const SYS_SIGNALFD: Sysno = 5276;
pub const SYS_TIMERFD: Sysno = 5277;
pub const SYS_EVENTFD: Sysno = 5278;
pub const SYS_FALLOCATE: Sysno = 5279;
pub const SYS_TIMERFD_CREATE: Sysno = 5280;
pub const SYS_TIMERFD_GETTIME: Sysno = 5281;
pub const SYS_TIMERFD_SETTIME: Sysno = 5282;
pub const SYS_SIGNALFD4: Sysno = 5283;
pub const SYS_EVENTFD2: Sysno = 5284;
pub const SYS_EPOLL_CREATE1: Sysno = 5285;
pub const SYS_DUP3: Sysno = 5286;
pub const SYS_PIPE2: Sysno = 5287;
pub const SYS_INOTIFY_INIT1: Sysno = 5288;
pub const SYS_PREADV: Sysno = 5289;
pub const SYS_PWRITEV: Sysno = 5290;
pub const SYS_RT_TGSIGQUEUEINFO: Sysno = 5291;
pub const SYS_PERF_EVENT_OPEN: Sysno = 5292;
pub const SYS_ACCEPT4: Sysno = 5293;
pub const SYS_RECVMMSG: Sysno = 5294;
pub const SYS_FANOTIFY_INIT: Sysno = 5295;
pub const SYS_FANOTIFY_MARK: Sysno = 5296;
pub const SYS_PRLIMIT64: Sysno = 5297;
pub const SYS_NAME_TO_HANDLE_AT: Sysno = 5298;
pub const SYS_OPEN_BY_HANDLE_AT: Sysno = 5299;
pub const SYS_CLOCK_ADJTIME: Sysno = 5300;
pub const SYS_SYNCFS: Sysno = 5301;
pub const SYS_SENDMMSG: Sysno = 5302;
pub const SYS_SETNS: Sysno = 5303;
pub const SYS_PROCESS_VM_READV: Sysno = 5304;
pub const SYS_PROCESS_VM_WRITEV: Sysno = 5305;
pub const SYS_KCMP: Sysno = 5306;
pub const SYS_FINIT_MODULE: Sysno = 5307;
pub const SYS_GETDENTS64: Sysno = 5308;
pub const SYS_SCHED_SETATTR: Sysno = 5309;
pub const SYS_SCHED_GETATTR: Sysno = 5310;
pub const SYS_RENAMEAT2: Sysno = 5311;
pub const SYS_SECCOMP: Sysno = 5312;
pub const SYS_GETRANDOM: Sysno = 5313;
pub const SYS_MEMFD_CREATE: Sysno = 5314;
pub const SYS_BPF: Sysno = 5315;
pub const SYS_EXECVEAT: Sysno = 5316;
pub const SYS_USERFAULTFD: Sysno = 5317;
pub const SYS_MEMBARRIER: Sysno = 5318;
pub const SYS_MLOCK2: Sysno = 5319;
pub const SYS_COPY_FILE_RANGE: Sysno = 5320;
pub const SYS_PREADV2: Sysno = 5321;
pub const SYS_PWRITEV2: Sysno = 5322;
pub const SYS_PKEY_MPROTECT: Sysno = 5323;
pub const SYS_PKEY_ALLOC: Sysno = 5324;
pub const SYS_PKEY_FREE: Sysno = 5325;
pub const SYS_STATX: Sysno = 5326;
pub const SYS_RSEQ: Sysno = 5327;
pub const SYS_IO_PGETEVENTS: Sysno = 5328;
pub const SYS_PIDFD_SEND_SIGNAL: Sysno = 5424;
pub const SYS_IO_URING_SETUP: Sysno = 5425;
pub const SYS_IO_URING_ENTER: Sysno = 5426;
pub const SYS_IO_URING_REGISTER: Sysno = 5427;
pub const SYS_OPEN_TREE: Sysno = 5428;
pub const SYS_MOVE_MOUNT: Sysno = 5429;
pub const SYS_FSOPEN: Sysno = 5430;
pub const SYS_FSCONFIG: Sysno = 5431;
pub const SYS_FSMOUNT: Sysno = 5432;
pub const SYS_FSPICK: Sysno = 5433;
pub const SYS_PIDFD_OPEN: Sysno = 5434;
pub const SYS_CLONE3: Sysno = 5435;
pub const SYS_CLOSE_RANGE: Sysno = 5436;
pub const SYS_OPENAT2: Sysno = 5437;
pub const SYS_PIDFD_GETFD: Sysno = 5438;
pub const SYS_FACCESSAT2: Sysno = 5439;
pub const SYS_PROCESS_MADVISE: Sysno = 5440;
pub const SYS_EPOLL_PWAIT2: Sysno = 5441;
pub const SYS_MOUNT_SETATTR: Sysno = 5442;
pub const SYS_QUOTACTL_FD: Sysno = 5443;
pub const SYS_LANDLOCK_CREATE_RULESET: Sysno = 5444;
pub const SYS_LANDLOCK_ADD_RULE: Sysno = 5445;
pub const SYS_LANDLOCK_RESTRICT_SELF: Sysno = 5446;
pub const SYS_PROCESS_MRELEASE: Sysno = 5448;
pub const SYS_FUTEX_WAITV: Sysno = 5449;
pub const SYS_SET_MEMPOLICY_HOME_NODE: Sysno = 5450;
pub const SYS_CACHESTAT: Sysno = 5451;
pub const SYS_FCHMODAT2: Sysno = 5452;
pub const SYS_MAP_SHADOW_STACK: Sysno = 5453;
pub const SYS_FUTEX_WAKE: Sysno = 5454;
pub const SYS_FUTEX_WAIT: Sysno = 5455;
pub const SYS_FUTEX_REQUEUE: Sysno = 5456;
pub const SYS_STATMOUNT: Sysno = 5457;
pub const SYS_LISTMOUNT: Sysno = 5458;
pub const SYS_LSM_GET_SELF_ATTR: Sysno = 5459;
pub const SYS_LSM_SET_SELF_ATTR: Sysno = 5460;
pub const SYS_LSM_LIST_MODULES: Sysno = 5461;
