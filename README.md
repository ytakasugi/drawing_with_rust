# [Plotters Developer Guide](https://plotters-rs.github.io/book/)

- Add dependency to `Cargo.toml`

  ```cmd
  cargo add plotters
  ```

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

このコードでは、以下の図が表示されます。

![basic0201_01](./plotter_developer_guide/section02/section0201/section0201_01/basic0201_01/images/2.1.png)

---

### Draw figure components

多くの場合、チャートには軸やメッシュグリッドなどの多くのコンポーネントが必要です。`ChartContext`タイプは、これらのコンポーネントを自動的に描画することができます。

---

#### Mesh

以下のコードは、`ChartContext::configure_mesh`を使用して、チャートにメッシュを追加する方法を示しています。

```rust
use plotters::prelude::*;

fn main() {
  let root_drawing_area = BitMapBackend::new("images/2.2.png", (600, 400))
    .into_drawing_area();

  root_drawing_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_drawing_area)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

}
```

---

#### Axes

プロットに軸を追加するには、2つのステップが必要です。

1. `ChartContext`の作成時に、ラベル領域のサイズを定義します。
2. `configure_mesh`を使用して、チャート・コンポーネントを描画します。

次のコードは、軸を追加する方法を示しています。

```rust
use plotters::prelude::*;

fn main() {
  let root_drawing_area = BitMapBackend::new("images/2.3.png", (600, 400))
    .into_drawing_area();

  root_drawing_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_drawing_area)
    // enables Y axis, the size is 40 px
    .set_label_area_size(LabelAreaPosition::Left, 40)
    // enable X axis, the size is 40 px
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

}
```

---

#### Title

次の例は、`ChartBuilder::caption`を使用してプロットにタイトルを追加する方法を示しています。

プロッタでは、フォントを表現する最も一般的な方法は、フォントフェイス名とフォントサイズのタプルを使用することです。

```rust
use plotters::prelude::*;

fn main() {
  let root_drawing_area = BitMapBackend::new("images/2.4.png", (600, 400))
    .into_drawing_area();

  root_drawing_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_drawing_area)
    .caption("Figure Sample", ("Arial", 30))
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

}
```

---

### Basic data Plotting

このセクションでは、Plottersを使用して、さまざまなタイプのプロットを作成してみましょう。一般的に、`ChartContext::draw_series`APIは、あらゆる種類のチャートを描画する機能を提供します。以下では、このAPIを使ってさまざまな種類のプロットを描画する方法について説明します。

---

#### Line series

次のコードは、プロッタで線分を描く方法を示しています。

```rust
use plotters::prelude::*;

fn main() {
  let root_area = BitMapBackend::new("images/2.5.png", (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Line Plot Demo", ("sans-serif", 40))
    .build_cartesian_2d(-10..10, 0..100)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

  ctx.draw_series(
    LineSeries::new((-10..=10).map(|x| (x, x* x)), &GREEN)
  ).unwrap();
}
```

このコードでは、以下の図が表示されます。

![basic0203_01](./plotter_developer_guide/section02/section0203/section0203_01/basic0203_01/images/2.5.png)

---

#### Scatter Plot

次のコードは、散布図を作成し、異なるポインティングエレメントを使用する方法を示しています。この例では、2つの異なる系列に対して`Circle`と`TriangleMarker`のポインティングエレメントを使用しています。

```rust
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.6.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..50, -10..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        DATA1.iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap();

    ctx.draw_series(DATA2.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();
}
const DATA1: [(i32, i32); 30] =  [(-3, 1), (-2, 3), (4, 2), (3, 0), (6, -5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];
const DATA2: [(i32, i32); 30] = [(1, 22), (0, 22), (1, 20), (2, 24), (4, 26), (6, 24), (5, 27), (6, 27), (7, 27), (8, 30), (10, 30), (10, 33), (12, 34), (13, 31), (15, 35), (14, 33), (17, 36), (16, 35), (17, 39), (19, 38), (21, 38), (22, 39), (23, 43), (24, 44), (24, 46), (26, 47), (27, 48), (26, 49), (28, 47), (28, 50)];
```

このコードでは、以下の図が表示されます。

![basic0203_02](./plotter_developer_guide/section02/section0203/section0203_02/basic0203_02/images/2.6.png)