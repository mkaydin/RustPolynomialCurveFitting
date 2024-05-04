mod vector;
mod csv_vec;

extern crate csv;
use std::time::Duration;
use std::thread::sleep;

use splinify::{
    read_csv_uxy, read_csv_xy, CubicSplineFit2D, LinearSplineFit2D, ParameterSplineCurveFit, Result,
};
use spliny::{CubicSpline2D, LinearSpline2D};

use vector::vector;
use csv_vec::csv_writer_controller;

fn main() -> Result<()> {
    let (x, y) = vector();
    csv_writer_controller(x,y);

    sleep(Duration::from_millis(1000));

    let (u,x,y) = read_csv_uxy("bb-locus-31.csv")?;
    let mut xy: Vec<f64> = Vec::with_capacity(x.len() * 2);
    x.iter().zip(y.iter()).for_each(|(&x, &y)| {
        xy.push(x);
        xy.push(y);
    });
    let d = ParameterSplineCurveFit::<3, 2>::new(u.clone(), xy.clone())?.smoothing_spline(1E-5)?;

    let json = serde_json::to_string(&d)?;
    println!("{}", json);

    d.plot("bb-fit.png", (2000, 1000))?;

    println!("Done.");
    Ok(())
}
