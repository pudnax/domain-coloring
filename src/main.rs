extern crate image;
extern crate num_complex;

use num_complex::Complex;

mod col;
use col::ColormapType;

const W: u32 = 2560;
const H: u32 = 1600;
const FOVY: u32 = 1;
const SUPER_SAMPLING: u32 = 1;

fn complex_function(z: Complex<f64>) -> Complex<f64> {
    z.inv().sin()
    // ((z - 1.) / (z + 1.)).sin().powi(3)
}

fn complex_color(z: Complex<f64>, color_map: &col::ColorMap) -> image::Rgba<u8> {
    let phase = z.to_polar();
    let mut t = phase.1 / std::f64::consts::PI + 1.;
    if t > 1. {
        t = 2. - t;
    }

    let c = color_map.at(t);
    image::Rgba([c.r(), c.g(), c.b(), 255])
}

fn pixel_coordinates(px: u32, py: u32) -> (f64, f64) {
    let center_real = 0.;
    let center_imag = 0.;
    let sw = f64::from(SUPER_SAMPLING * W);
    let sh = f64::from(SUPER_SAMPLING * H);
    let aspect_ratio = f64::from(W) / f64::from(H);
    let half_fovy = f64::from(FOVY) / 2.;

    let x = ((f64::from(px) / (sw - 1.)) * 2. - 1.) * aspect_ratio * half_fovy + center_real;
    let y = (((sh - f64::from(py) - 1.) / (sh - 1.)) * 2. - 1.) * half_fovy + center_imag;
    (x, y)
}

fn main() {
    let sw = SUPER_SAMPLING * W;
    let sh = SUPER_SAMPLING * H;
    let mut imgbuf = image::ImageBuffer::new(sw, sh);

    let (x0, y0) = pixel_coordinates(0, 0);
    let (x1, y1) = pixel_coordinates(sw - 1, sh - 1);
    let dx = (x1 - x0) / f64::from(sw - 1);
    let dy = (y1 - y0) / f64::from(sh - 1);

    let color_map = col::ColorMap::new(ColormapType::Inferno);

    let mut y = y0;
    for py in 0..sh {
        let mut x = x0;
        for px in 0..sw {
            let z = complex_function(Complex::new(x, y));
            let c = complex_color(z, &color_map);
            imgbuf.put_pixel(px, py, c);
            x += dx;
        }
        y += dy;
    }

    imgbuf.save("out0.png").unwrap();
}
