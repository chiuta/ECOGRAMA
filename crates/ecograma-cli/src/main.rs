use ecograma_core::{dft_magnitude, hann, peak_bin};
use std::f64::consts::PI;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version"] => println!("ecograma {}", env!("CARGO_PKG_VERSION")),
        ["selftest", n, bin] => match (n.parse::<usize>(), bin.parse::<f64>()) {
            (Ok(n), Ok(bin)) if n > 0 => {
                let w = hann(n);
                let x: Vec<f32> = (0..n)
                    .map(|i| ((2.0 * PI * bin * i as f64 / n as f64).sin() as f32) * w[i])
                    .collect();
                println!("peak bin: {:?}", peak_bin(&dft_magnitude(&x)));
            }
            _ => usage(),
        },
        _ => usage(),
    }
}

fn usage() {
    eprintln!("usage: ecograma selftest N BIN | --version");
    std::process::exit(2);
}
