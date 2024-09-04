use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::Circle;
use plotters::prelude::{Color, HSLColor, IntoFont, LineSeries, WHITE};

use crate::contour::Contour;
use crate::scene::Scene;

pub fn draw_contour(
    path: impl ToString,
    _scene: &Scene,
    contours: &[Contour],
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.to_string();

    // 创建一个400x400像素的PNG文件

    // white bg:
    let root = BitMapBackend::new(&path, (400, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    // 创建图表上下文
    let mut chart = ChartBuilder::on(&root)
        .caption("散点图示例", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..5f64, 0f64..5f64)?;

    // 配置坐标轴
    chart.configure_mesh().draw()?;

    // // Scene
    // let ps = scene
    //     .points
    //     .iter()
    //     .map(|p| Circle::new((p.x, p.y), 3, &BLUE.mix(0.5)));
    // chart.draw_series(ps)?;

    let l = contours.len() as f64;
    for (i, contour) in contours.iter().enumerate() {
        let color = HSLColor(i as f64 / l, 0.8, 0.5).mix(0.5);

        chart.draw_series(LineSeries::new(
            contour.points.iter().map(|p| (p.x, p.y)),
            &color.mix(0.2),
        ))?;

        // Draw contour
        let ps = contour
            .points
            .iter()
            .map(|p| Circle::new((p.x, p.y), 1, &color.mix(0.5)));
        chart.draw_series(ps)?;
    }

    root.present()?;

    Ok(())
}
