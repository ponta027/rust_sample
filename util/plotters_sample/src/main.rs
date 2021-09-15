use plotters::drawing::IntoDrawingArea;
use plotters::prelude::*;

/** */
fn sample_plot2(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0f32..2f32, -0.0f32..2f32)?;

    chart.configure_mesh().draw()?;

    let points: Vec<(f32, f32)> = (-50..=50)
        .map(|x| x as f32 / 50.0)
        .map(|x| (x, x * x))
        .collect();
    let points_sine: Vec<(f32, f32)> = (-100..=100)
        .map(|x| x as f32)
        .map(|x| (x / 100., x.sin()))
        .collect();

    let data_points: Vec<(f32, f32)> = (0..=10).map(|x| x as f32 / 10.0).map(|x| (x, x)).collect();

    chart
        .draw_series(LineSeries::new(data_points.into_iter(), &BLUE))?
        .label("y = x")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(points.into_iter(), &BLUE))?
        .label("y = x^2 ")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(points_sine.into_iter(), &GREEN))?
        .label("y = sin(x) ")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

/** */
struct GraphInfo<'a> {
    width: u32,
    height: u32,
    name: String,
    caption: String,
    points: &'a Vec<GraphData<'a>>,
}

struct GraphData<'a> {
    name: String,
    points: &'a Vec<(f32, f32)>,
}

/** */
fn plot_data(graph: &GraphInfo) -> Result<(), Box<dyn std::error::Error>> {
    //fn plot_data(graph: &GraphInfo, data: Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&graph.name, (graph.width, graph.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(&graph.caption, ("sans-serif", 50).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0f32..2f32, -0.0f32..2f32)?;

    chart.configure_mesh().draw()?;
    for it in graph.points {
        let mut data: Vec<(f32, f32)> = Vec::new();
        for it2 in it.points {
            data.push(*it2);
        }
        chart
            .draw_series(LineSeries::new(data.into_iter(), &BLUE))?
            .label("y = x")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    }
    //    for it in graph.points[0].points {}

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

/** */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let points_sine: Vec<(f32, f32)> = (-10..=10)
        .map(|x| x as f32)
        .map(|x| (x / 10., x.sin()))
        .collect();
    let points2: Vec<(f32, f32)> = (-10..=10)
        .map(|x| x as f32)
        .map(|x| (x / 10., x/10.))
        .collect();


    let data = GraphData {
        name: "sample".to_string(),
        points: &points_sine,
    };
    let data2 = GraphData {
        name: "sample2".to_string(),
        points: &points2,
    };


    let graph1 = GraphInfo {
        width: 640,
        height: 480,
        name: "sine_curve.png".to_string(),
        caption: "sine curve".to_string(),
        points: &vec![data,data2],
    };

    plot_data(&graph1).unwrap();

    sample_plot2("sample_plot2.png".to_string()).unwrap();
    Ok(())
}
