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
use std::ffi;

pub struct VM {
    raw: api::JSContextGroupRef
}

impl VM {
    pub fn new() -> VM {
        unsafe {
            VM {
                raw: api::JSContextGroupCreate(),
            }
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            api::JSContextGroupRelease(self.raw);
        }
    }
}

pub struct Context {
    raw: api::JSGlobalContextRef
}

impl Context {
    pub fn new(vm: &VM) -> Context {
        unsafe {
            Context {
                raw: api::JSGlobalContextCreateInGroup(vm.raw, ptr::null_mut()),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            api::JSGlobalContextRelease(self.raw);
        }
    }
}

pub struct String {
    raw: api::JSStringRef
}

impl String {
    pub fn new(s: &str) -> String {
        let cstr = ffi::CString::new(s.as_bytes()).unwrap();
        unsafe {
            String {
                raw: api::JSStringCreateWithUTF8CString(cstr.as_ptr())
            }
        }
    }

    pub fn length(&self) {
        unsafe {
            api::JSStringGetLength(self.raw);
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            api::JSStringRelease(self.raw);
        }
    }
}

pub struct Value {
    raw: api::JSValueRef
}

pub type JSResult<T> = Result<T, Value>;

// Value is GC-managed. So it does not implement Drop trait.
impl Value {
    pub fn with_boolean(ctx: &Context, value: bool) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeBoolean(ctx.raw, value as u8)
            }
        }
    }

    pub fn with_number(ctx: &Context, value: f64) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeNumber(ctx.raw, value)
            }
        }
    }

    pub fn with_string(ctx: &Context, value: &String) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeString(ctx.raw, value.raw)
            }
        }
    }

    pub fn null(ctx: &Context) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeNull(ctx.raw)
            }
        }
    }

    pub fn undefined(ctx: &Context) -> Value {
        unsafe {
            Value {
                raw: api::JSValueMakeUndefined(ctx.raw)
            }
        }
    }

    pub fn is_boolean(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsBoolean(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_null(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsNull(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_undefined(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsUndefined(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_number(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsNumber(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_string(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsString(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_object(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsObject(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_array(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsArray(ctx.raw, self.raw) != 0
        }
    }

    pub fn is_date(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueIsDate(ctx.raw, self.raw) != 0
        }
    }

    pub fn to_number(&self, ctx: &Context) -> JSResult<f64> {
        unsafe {
            let exception : api::JSValueRef = ptr::null_mut();
            let result = api::JSValueToNumber(ctx.raw, self.raw, exception as *mut _);
            if exception == ptr::null() {
                Ok(result)
            } else {
                Err(Value { raw: exception })
            }
        }
    }

    pub fn to_boolean(&self, ctx: &Context) -> bool {
        unsafe {
            api::JSValueToBoolean(ctx.raw, self.raw) != 0
        }
    }
}

