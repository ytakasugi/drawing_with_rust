# [Plotters Developer Guide](https://plotters-rs.github.io/book/)

---

## Basic plotting

---

### Define chart context

先に述べたように、Plottersは描画ライブラリです。理論的には、Plottersの描画APIに基づいて、あらゆるデータプロットを描くことができるはずです。描画APIの上には、チャート・コンテキストが用意されていて、データの可視化に適した環境を整えています。

---

#### Create a chart context from a drwaing area

`ChartBuilder`は、チャート・コンテキストを作成するためのビルダー・タイプです。

次のコードは、X軸とY軸の両方が0から100の範囲である2次元の直交座標を使用したチャートコンテキストを作成する方法を示しています。

```rust
use plotters::prelude::*;

fn main() {
  let drawing_area = BitMapBackend::new("images/2.0.png", (1024, 768))
    .into_drawing_area();
  
  let _chart = ChartBuilder::on(&drawing_area)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();
}
```

---

#### Draw series on to the chart context

チャートのコンテキストができたら、その上に系列を置くことができます。この例では、`LineSeries`タイプを使用して、チャート上にラインシリーズを描きます。

```rust
use plotters::prelude::*;

fn main() {
  let drawing_area = BitMapBackend::new("images/2.1.png", (600, 400))
    .into_drawing_area();

  drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&drawing_area)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();

  chart.draw_series(
    LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK),
  ).unwrap();
}
```



