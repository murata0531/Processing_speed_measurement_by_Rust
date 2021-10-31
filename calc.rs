use std::time::{Duration, Instant};
use std::thread::sleep;

// 年率計算
fn Measure(repeat_cnt:i64,principal:f64,goal:f64) {
    for i in 0..repeat_cnt {
        let mut age_cnt:i64 = 0;
        let mut inner_principal:f64 = principal;
        loop {
            inner_principal *= 1.05;
            age_cnt += 1;
            if inner_principal >= goal {
                println!("{}",inner_principal);
                println!("{}",age_cnt);
                break;
            }
        }
    }
}

// ライプニッツ級数
fn Leibniz() {
    let mut pi4:f64 = 0.0;
	
	for i in 0..100000000 {
        let i_cast:f64 = i as f64;
		pi4 += (1.0 / (i_cast * 4.0 + 1.0) - 1.0 / (i_cast * 4.0 + 3.0));
	}
    pi4 *= 4.0;
	println!("{}",pi4);
}

fn main() {

    // 年率計算
	// let repeat_cnt:i64 = 100;
    // let principal:f64 = 500000.0;
	// let goal:f64 = 45000000.0;

    let start = Instant::now();
	// Measure(repeat_cnt,principal,goal);

    Leibniz();
    let end = start.elapsed();

    // 秒単位
    // println!("end:{}.{:017}[sec]", end.as_secs(), end.subsec_nanos() / 1_000_000);
    println!("end:{:017}[nano_sec]", end.subsec_nanos());

}