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
#![allow(unused_variables)]

extern crate jsc;
use jsc::api;

#[test]
fn simple() {
    unsafe {
        let vm = api::JSContextGroupCreate();
        api::JSContextGroupRelease(vm);
    }
}

#[test]
fn context() {
    let vm = jsc::VM::new();
    let context = jsc::Context::new(&vm);
    let string = jsc::String::new("Hello World");
    {
        let value = jsc::Value::with_boolean(&context, false);
        assert!(value.is_boolean(&context));
    }

    {
        let value = jsc::Value::with_number(&context, 42 as f64);
        match value.to_number(&context) {
            Ok(n) => assert_eq!(n, 42 as f64),
            Err(_) => unreachable!(),
        }
    }
}
