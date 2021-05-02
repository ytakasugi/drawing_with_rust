use plotters::prelude::*;

fn main() {
    // 画像サイズが600 x 400の画像を`./images/`配下に作成
    let root_area = BitMapBackend::new("images/2.5.png", (600, 400)).into_drawing_area();
    // 背景を白に設定
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
                    .set_label_area_size(LabelAreaPosition::Left, 40)
                    .set_label_area_size(LabelAreaPosition::Bottom, 40)
                    .caption("Line Plot Demo", ("sans-serif", 40))
                    .build_cartesian_2d(-10..10, 0..100)
                    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((-10..=10).map(|x| (x, x * x)), &GREEN)
    ).unwrap();
}
