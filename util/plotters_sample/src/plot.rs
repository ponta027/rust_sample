use plotters::prelude::*;

/** */
pub struct GraphInfo<'a> {
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub caption: String,
    pub points: &'a Vec<GraphData<'a>>,
}

/** */
pub struct GraphData<'a> {
    pub name: String,
    pub points: &'a Vec<(f32, f32)>,
}

/** */
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
            .label(&it.name)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
