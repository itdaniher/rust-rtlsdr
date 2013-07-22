use std::str;
use std::libc::{c_int, c_uint, c_void};
use std::vec;
use std::cast::transmute;
use std::ptr::{null};
use std::task::{SingleThreaded, spawn_sched};

// NB. pointers are 64b-int-like
extern {
	fn rtlsdr_open(dev: **c_void, devIndex: u32) -> u32;
    fn rtlsdr_get_device_count() -> u32;
	fn rtlsdr_get_device_name(devIndex: u32) -> *i8;
	fn rtlsdr_reset_buffer(dev: *c_void) -> c_int;
	fn rtlsdr_set_center_freq(dev: *c_void, freq: u32) -> c_int;
	fn rtlsdr_set_tuner_gain(dev: *c_void, gain: u32) -> c_int;
	fn rtlsdr_read_sync(dev: *c_void, buf: *mut u8, len: u32, n_read: *c_int) -> c_int;
	fn rtlsdr_read_async(dev: *c_void, cb: u64, ctx: *c_void, buf_num: u32, buf_len: u32) -> c_int;
	fn rtlsdr_cancel_async(dev: *c_void) -> c_int;
	fn rtlsdr_close(dev: *c_void) -> c_int;
}

pub fn close(dev: *c_void){
	unsafe {
		let success = rtlsdr_close(dev);
		assert_eq!(success, 0);
	}
}

extern fn rtlsdr_callback(buf: *u8, len: u32, ctx: *c_void) {
	assert_eq!(len, 512);
	unsafe {
		let data = vec::raw::from_buf_raw(buf, len as uint);
	}
}

pub fn readAsync(dev: *c_void) -> () {
	do spawn_sched(SingleThreaded) { 
		unsafe{
			rtlsdr_read_async(dev, transmute(rtlsdr_callback), null(), 32, 512);
		}
	}
}

pub fn stopAsync(dev: *c_void) -> () {
	unsafe {
		let success = rtlsdr_cancel_async(dev);
		println(fmt!("%?", success));
		assert_eq!(success, 0);
	}
}

pub fn readSync(dev: *c_void, ct: c_uint) -> ~[u8] {
	unsafe {
		let n_read: c_int = 0;
		let mut buffer: ~[u8] = ~[0, ..512];
		let success = rtlsdr_read_sync(dev, vec::raw::to_mut_ptr(buffer), ct, &n_read);
		assert_eq!(success, 0);
		assert_eq!(ct as i32, n_read);
		return buffer;
	}
}

pub fn getDeviceCount() -> u32 {
	unsafe {
		let x: u32 = rtlsdr_get_device_count();
		return x;
	}
}

pub fn openDevice(devIndex: u32) -> *c_void{
	unsafe {
		let devStructPtr: *c_void = transmute(0);
		let success = rtlsdr_open(&devStructPtr, devIndex);
		assert_eq!(success, 0);
		return devStructPtr;
	}
}

pub fn getDeviceName(devIndex: u32) -> ~str {
	unsafe {
		let deviceString: *i8 = rtlsdr_get_device_name(devIndex);
		return str::raw::from_c_str(deviceString);
	}
}

pub fn clearBuffer(device: *c_void) {
	unsafe {
		let success = rtlsdr_reset_buffer(device);
		assert_eq!(success, 0);
	}
}

pub fn setFrequency(device: *c_void, freq: u32) {
	unsafe {
		let success = rtlsdr_set_center_freq(device, freq);
		assert_eq!(success, 0);
	}
}

pub fn setGainAuto(device: *c_void) {
	unsafe {
		let success = rtlsdr_set_tuner_gain(device, 0);
		assert_eq!(success, 0);
	}
}
