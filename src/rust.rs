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

use api;
use std::ptr;

pub struct VM {
    group : api::JSContextGroupRef
}

impl VM {
    pub fn new() -> VM {
        unsafe {
            VM {
                group: api::JSContextGroupCreate(),
            }
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            api::JSContextGroupRelease(self.group);
        }
    }
}

pub struct Context {
    global: api::JSGlobalContextRef
}

impl Context {
    pub fn new(vm : &VM) -> Context {
        unsafe {
            Context {
                global: api::JSGlobalContextCreateInGroup(vm.group, ptr::null_mut()),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            api::JSGlobalContextRelease(self.global);
        }
    }
}
