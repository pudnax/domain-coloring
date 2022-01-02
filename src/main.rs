use num_complex::Complex;

use rayon::prelude::*;

mod col;
use col::ColormapType;

const W: u32 = 2560;
const H: u32 = 1600;
const FOVY: u32 = 1;
const SUPER_SAMPLING: u32 = 1;

const SW: u32 = SUPER_SAMPLING * W;
const SH: u32 = SUPER_SAMPLING * H;

fn complex_function(z: Complex<f64>) -> Complex<f64> {
    // z.inv().sin()
    ((z - 1.) / (z + 1.)).cos().powi(20)
    // ((z * z - 1.) * (z - 2. - Complex::i()).powi(2)) / (z * z + 2. + 2. * Complex::i())
}

fn complex_color(z: Complex<f64>, color_map: &col::ColorMap) -> image::Rgba<u8> {
    let phase = z.arg();
    let mut t = phase / std::f64::consts::PI + 1.;
    if t > 1. {
        t = 2. - t;
    }

    let c = color_map.at(t);
    image::Rgba([c.r(), c.g(), c.b(), 255])
}

fn pixel_coordinates(px: u32, py: u32) -> (f64, f64) {
    let center_real = 0.;
    let center_imag = 0.;
    let sw_f = f64::from(SW);
    let sh_f = f64::from(SH);
    let aspect_ratio = f64::from(W) / f64::from(H);
    let half_fovy = f64::from(FOVY) / 2.;

    let x = ((f64::from(px) / (sw_f - 1.)) * 2. - 1.) * aspect_ratio * half_fovy + center_real;
    let y = (((sh_f - f64::from(py) - 1.) / (sh_f - 1.)) * 2. - 1.) * half_fovy + center_imag;
    (x, y)
}

fn main() {
    let (x0, y0) = pixel_coordinates(0, 0);
    let (x1, y1) = pixel_coordinates(SW - 1, SH - 1);
    let dx = (x1 - x0) / f64::from(SW - 1);
    let dy = (y1 - y0) / f64::from(SH - 1);

    let color_map = col::ColorMap::new(ColormapType::Inferno);

    let mut buf = vec![[0; 4]; (SW * SH) as usize];
    (0..(SW * SH) as usize)
        .into_par_iter()
        .enumerate()
        .map(|(idx, _)| {
            let px = (idx % SW as usize) as f64;
            let py = (idx / SW as usize) as f64;
            let x = x0 + px * dx;
            let y = y0 + py * dy;
            let z = complex_function(Complex::new(x, y));
            let pixel = complex_color(z, &color_map);
            pixel.0
        })
        .collect_into_vec(&mut buf);
    let buf = buf.iter().flatten().copied().collect();

    image::ImageBuffer::<image::Rgba<u8>, _>::from_vec(SW, SH, buf)
        .unwrap()
        .save("pic.png")
        .unwrap();
}
