mod plot;

/** */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "png/sine_curve.png";
    let points_sine: Vec<(f32, f32)> = (-10..=10)
        .map(|x| x as f32)
        .map(|x| (x / 10., x.sin()))
        .collect();
    let points2: Vec<(f32, f32)> = (-10..=10)
        .map(|x| x as f32)
        .map(|x| (x / 10., x / 10.))
        .collect();

    let data = plot::GraphData {
        name: "sample".to_string(),
        points: &points_sine,
    };
    let data2 = plot::GraphData {
        name: "sample2".to_string(),
        points: &points2,
    };

    let graph1 = plot::GraphInfo {
        width: 640,
        height: 480,
        name: name.to_string(),
        caption: "sine curve".to_string(),
        points: &vec![data, data2],
    };

    // plot::sample_plot2(name.to_string()).unwrap();

    plot::plot_data(&graph1).unwrap();

    {
        let name2 = "png/sine_curve2.png";
        let points_sine2: Vec<(f32, f32)> = (-20..=20)
            .map(|x| x as f32)
            .map(|x| (x / 10., x.sin()))
            .collect();
        let data2 = plot::GraphData {
            name: "sample".to_string(),
            points: &points_sine2,
        };
        let graph2 = plot::GraphInfo {
            width: 640,
            height: 480,
            name: name2.to_string(),
            caption: "sine curve2".to_string(),
            points: &vec![data2],
        };

        plot::plot_data(&graph2).unwrap();
    }
    Ok(())
}
