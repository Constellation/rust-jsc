/*
  Copyright (C) 2016 Yusuke Suzuki <utatane.tea@gmail.com>

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.

  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
  IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
  ARE DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> BE LIABLE FOR ANY
  DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
  (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
  LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
  ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
  (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
  THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

extern crate pkg_config;

use std::env;
use std::process::{Command, Stdio};

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
    let here = env::current_dir().unwrap();
    let webkit_library_dir = here.join("WebKit").join("WebKitBuild").join("Release").join("lib");
    let result = Command::new("make")
        .args(&["-R", "-f", "makefile.cargo"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(result.success());
    println!("cargo:rustc-link-search=native={}", webkit_library_dir.display());
	println!("cargo:rustc-link-lib=static=JavaScriptCore");
	println!("cargo:rustc-link-lib=static=WTF");
	println!("cargo:rustc-link-lib=static=bmalloc");

    let lib = pkg_config::find_library("icu-uc").unwrap();
    for library in &lib.libs {
        println!("cargo:rustc-link-lib=dylib={}", library);
    }

	println!("cargo:rustc-link-lib=static=icuuc");

	println!("cargo:rustc-link-lib=stdc++");
	println!("cargo:outdir={}", out_dir);
}
