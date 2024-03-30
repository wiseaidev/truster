use plotters::prelude::*;
use plotters::prelude::full_palette::{PINK, GREY};

fn main() {
    let root = BitMapBackend::new("out.png", (800, 600)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let data = ["C", "Rust", "Go", "Julia", "TypeScript"];
    let mut chart = ChartBuilder::on(&root)
        .caption("Average time taken for computing 45th Fibonacci sequence", ("sans-serif", 20))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d((&data[..]).into_segmented(), 0.0..20.0f64)
        .unwrap();

     let _ = chart.configure_mesh().disable_mesh()
            .x_desc("Programming Language")
            .y_desc("Time (seconds)").draw();

    let hist = Histogram::vertical(&chart).data(vec![(&"C", 3.387515)]).margin(15).style(GREY.filled());

    let _ = chart.draw_series(hist);

    let hist = Histogram::vertical(&chart).data(vec![(&"Rust", 4.653)]).margin(10).style(RED.mix(0.7).filled());

    let _ = chart.draw_series(hist);

    let hist = Histogram::vertical(&chart).data(vec![(&"Go", 6.4279091374499995)]).margin(10).style(BLUE.mix(0.7).filled());

    let _ = chart.draw_series(hist);

    let hist = Histogram::vertical(&chart).data(vec![(&"Julia", 6.988585221767425)]).margin(10).style(PINK.mix(0.9).filled());

    let _ = chart.draw_series(hist);

    let hist = Histogram::vertical(&chart).data(vec![(&"TypeScript", 17.065100000000005)]).margin(10).style(BLUE.filled());

    let _ = chart.draw_series(hist);

    // TODO: Refactor to

    // let data = [
    //     ("C", 3.387515, GREY),
    //     ("Rust", 4.653, RED.mix(0.7)),
    //     ("Go", 6.4279091374499995, BLUE.mix(0.7)),
    //     ("Julia", 6.988585221767425, PINK.mix(0.9)),
    //     ("TypeScript", 17.065100000000005, BLUE.mix(0.9)),
    // ];

    // data.iter().map(|&(label, value, color)| {
    //     let hist = Histogram::vertical(&chart)
    //         .data(vec![(label, value)])
    //         .margin(10)
    //         .style(color.filled());

    //     chart.draw_series(hist)
    // }).collect::<Result<Vec<_>, _>>().unwrap();
}
