// Copyleft 2015-2019 Superstring.Community
// This file is part of Susy Sophon.

// Susy Sophon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Susy Sophon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Susy Sophon.  If not, see <http://www.gnu.org/licenses/>.

use std::{mem, ptr};
use std::ffi::c_void;
use std::sync::Arc;

use {Callback, susy_config_from_cli, susy_destroy, susy_rpc_worker, susy_start, susy_set_logger,
	susy_unsubscribe_ws, susy_ws_worker, SusyParams};

use jni::{JavaVM, JNIEnv};
use jni::objects::{JClass, JString, JObject, JValue, GlobalRef};
use jni::sys::{jlong, jobjectArray, va_list};
use susy_sophon::RunningClient;

type CheckedQuery<'a> = (&'a RunningClient, String, JavaVM, GlobalRef);

// Creates a Java callback to a static method named `void callback(Object)`
struct JavaCallback<'a> {
	jvm: JavaVM,
	callback: GlobalRef,
	method_name: &'a str,
	method_descriptor: &'a str,
}

unsafe impl<'a> Send for JavaCallback<'a> {}
unsafe impl<'a> Sync for JavaCallback<'a> {}

impl<'a> JavaCallback<'a> {
	fn new(jvm: JavaVM, callback: GlobalRef) -> Self {
		Self {
			jvm,
			callback,
			method_name: "callback",
			method_descriptor: "(Ljava/lang/Object;)V",
		}
	}
}

impl<'a> Callback for JavaCallback<'a> {
	fn call(&self, msg: &str) {
		let env = self.jvm.attach_current_thread().expect("JavaVM should have an environment; qed");
		let java_str = env.new_string(msg.to_string()).expect("Rust String is valid JString; qed");
		let val = &[JValue::Object(JObject::from(java_str))];
		env.call_method(self.callback.as_obj(), self.method_name, self.method_descriptor, val).expect(
			"The callback must be an instance method and be named \"void callback(Object)\"; qed)");
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_susy_sophon_Susy_configFromCli(env: JNIEnv, _: JClass, cli: jobjectArray) -> jlong {
	let cli_len = env.get_array_length(cli).expect("invalid Java bindings") as usize;

	let mut jni_strings = Vec::with_capacity(cli_len);
	let mut opts = Vec::with_capacity(cli_len);
	let mut opts_lens = Vec::with_capacity(cli_len);

	for n in 0..cli_len as i32 {
		let elem = env.get_object_array_element(cli, n).expect("invalid Java bindings");
		let elem_str: JString = elem.into();
		match env.get_string(elem_str) {
			Ok(s) => {
				opts.push(s.as_ptr());
				opts_lens.push(s.to_bytes().len());
				jni_strings.push(s);
			}
			Err(err) => {
				let _ = env.throw_new("java/lang/Exception", err.to_string());
				return 0
			}
		};
	}

	let mut out = ptr::null_mut();
	match susy_config_from_cli(opts.as_ptr(), opts_lens.as_ptr(), cli_len, &mut out) {
		0 => out as jlong,
		_ => {
			let _ = env.throw_new("java/lang/Exception", "failed to create config object");
			0
		},
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_susy_sophon_Susy_build(
	env: JNIEnv,
	_: JClass,
	config: va_list,
	logger_mode: JString,
	logger_file: JString
) -> jlong {
	let mut params = SusyParams {
		configuration: config,
		.. mem::zeroed()
	};

	let logger_mode: String = env.get_string(logger_mode).expect("valid JString; qed").into();
	let logger_file: String = env.get_string(logger_file).expect("valid JString; qed").into();

	susy_set_logger(logger_mode.as_ptr(), logger_mode.as_bytes().len(), logger_file.as_ptr(),
					  logger_file.as_bytes().len(), &mut params.logger);

	let mut out = ptr::null_mut();
	match susy_start(&params, &mut out) {
		0 => out as jlong,
		_ => {
			let _ = env.throw_new("java/lang/Exception", "failed to start Susy");
			0
		}
	}
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_susy_sophon_Susy_destroy(_env: JNIEnv, _: JClass, susy: va_list) {
	susy_destroy(susy);
}

unsafe fn java_query_checker<'a>(client: va_list, rpc: JString, callback: JObject, env: &JNIEnv<'a>)
-> Result<CheckedQuery<'a>, String> {
	let query: String = env.get_string(rpc)
		.map(Into::into)
		.map_err(|e| e.to_string())?;

	let client: &RunningClient = &*(client as *const RunningClient);
	let jvm = env.get_java_vm().map_err(|e| e.to_string())?;
	let global_ref = env.new_global_ref(callback).map_err(|e| e.to_string())?;
	Ok((client, query, jvm, global_ref))
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_susy_sophon_Susy_rpcQueryNative(
	env: JNIEnv,
	_: JClass,
	susy: va_list,
	rpc: JString,
	timeout_ms: jlong,
	callback: JObject,
	)
{
	let _ = java_query_checker(susy, rpc, callback, &env)
		.map(|(client, query, jvm, global_ref)| {
			let callback = Arc::new(JavaCallback::new(jvm, global_ref));
			susy_rpc_worker(client, &query, callback, timeout_ms as u64);
		})
		.map_err(|e| {
			let _ = env.throw_new("java/lang/Exception", e);
		});
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_susy_sophon_Susy_subscribeWebSocketNative(
	env: JNIEnv,
	_: JClass,
	susy: va_list,
	rpc: JString,
	callback: JObject,
	) -> va_list {

	java_query_checker(susy, rpc, callback, &env)
		.map(move |(client, query, jvm, global_ref)| {
			let callback = Arc::new(JavaCallback::new(jvm, global_ref));
			susy_ws_worker(client, &query, callback) as va_list
		})
		.unwrap_or_else(|e| {
			let _ = env.throw_new("java/lang/Exception", e);
			ptr::null_mut()
		})
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_susy_sophon_Susy_unsubscribeWebSocketNative(
	_: JNIEnv,
	_: JClass,
	session: va_list) {
	susy_unsubscribe_ws(session as *const c_void);
}
