// SPDX-License-Identifier: GPL-2
//
// BPFContain - Container security with eBPF
// Copyright (C) 2020  William Findlay
//
// Dec. 29, 2020  William Findlay  Created this.

fn main() {
    println!("cargo:rustc-link-lib=dylib=bpfcontain");
}
