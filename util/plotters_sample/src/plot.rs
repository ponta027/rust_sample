use plotters::prelude::*;

/** */
pub struct GraphInfo<'a> {
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub caption: String,
    pub points: &'a Vec<GraphData<'a>>,
}

pub struct GraphData<'a> {
    pub name: String,
    pub points: &'a Vec<(f32, f32)>,
}


pub fn plot_data(graph: &GraphInfo) -> Result<(), Box<dyn std::error::Error>> {
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
pub fn sample_plot2(name: String) -> Result<(), Box<dyn std::error::Error>> {
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


