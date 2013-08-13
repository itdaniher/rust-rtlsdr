extern mod extra;

use extra::complex;
use std::num;

pub fn asRe ( d: ~[f32] ) -> ~[complex::Complex32] { return d.iter().transform(|&x| {complex::Cmplx {re: x, im: 0.0}}).collect::<~[complex::Complex32]>();}
pub fn asF32 ( d: ~[complex::Complex32] ) -> ~[f32] { return d.iter().transform(|&x| {if (num::abs(x.im) < 0.001) { x.re } else { let (m,p) = x.to_polar(); m*num::signum(p) }}).collect::<~[f32]>(); }
pub fn asF64 ( d: ~[f32] ) -> ~[f64] { return d.iter().transform(|&x| x as f64).collect(); }

pub fn window(m: uint) -> ~[f32] {
	let N = m as f32;
	let pi: f32 = num::atan2(1f32,1f32) * 4f32;
	// blackman-nuttall coefficients
	let a: ~[f32] = ~[0.3635819, 0.4891775, 0.1365995, 0.0106411];
	// blackman-harris window coefficients
	// let a: ~[f32] = ~[0.35875, 0.48829, 0.14128, 0.01168];
	// hamming window coefficients
	// let a: ~[f32] = ~[0.54, 0.46, 0.0, 0.0];
	let results: ~[f32] = range(0, m + 1).transform(|x| {
		let n = x as f32;
		a[0] - a[1]*num::cos(2f32*pi*n/(N-1f32))+a[2]*num::cos(4f32*pi*n/(N-1f32))-a[3]*num::cos(6f32*pi*n/(N-1f32))
	}).collect();
	return results;
}

pub fn sinc(m: uint, fc: f32) -> ~[f32] {
	// fc is decimal amount of sample rate at which to place corner
	// should always be at nyquist or below
	assert!(fc <= 0.5);
	let pi: f32 = num::atan2(1f32,1f32) * 4f32;
	let results: ~[f32] = range(0, m).transform(|x| -> f32 {
		let n = x as f32 - m as f32/2f32;
		let mut r = 2f32*fc;
		if (n != 0.0) { r = num::sin(2f32*pi*fc*n)/(pi*n); }
		r
	}).collect::<~[f32]>();
	return results;
}

pub fn lpf(m: uint, fc: f32) -> ~[f32] {
//	assert_eq!(m % 2, 1);
	let w = window(m);
	let s = sinc(m, fc);
	let r: ~[f32] = w.iter().zip(s.iter()).transform(|(&x, &y)| x * y).collect::<~[f32]>();
	return r;
}

pub fn hpf(m: uint, fc: f32) -> ~[f32] {
	let l: ~[f32] = lpf(m, fc);
	let mut h: ~[f32] = l.iter().transform(|&x| -x ).collect::<~[f32]>();
	h[m/2-1] += 1.0;
	return h;
}

pub fn bsf(m: uint, fc1: f32, fc2: f32) -> ~[f32] {
	let lp: ~[f32] = lpf(m, fc1);
	let hp: ~[f32] = hpf(m, fc2);
	let mut h: ~[f32] = lp.iter().zip(hp.iter()).transform(|(&x, &y)| x+y).collect::<~[f32]>();
	h[m/2-1] -= 1.0;
	return h;
}

pub fn bpf(m:uint, fc1: f32, fc2: f32) -> ~[f32] {
	let b: ~[f32] = bsf(m, fc1, fc2);
	let mut h: ~[f32] = b.iter().transform(|&x| -x ).collect::<~[f32]>();
	return h;
}

fn main() {
	println(fmt!("%?", lpf(511, 20.0e3/88.1e3)));
}
