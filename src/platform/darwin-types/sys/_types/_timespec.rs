// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::__darwin_time_t;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct timespec_t {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: isize,
}
