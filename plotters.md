### [Crate plotters](https://docs.rs/plotters/0.3.0/plotters/)

Plottersは、純粋なRustで図形、プロット、チャートを描画するために設計されたドローイングライブラリです。`Plotters`は、bitmap、vector graph、piston window、GTK/Cairo、WebAssemblyなど、さまざまな種類のバックエンドをサポートしています。

- 新しいPlotters Developer's Guideの作成を進めています。プレビュー版は[こちら](https://plotters-rs.github.io/book)からご覧いただけます。
- インタラクティブなJupyterノートブックでPlottersを試すには、[ここ](https://plotters-rs.github.io/plotters-doc-data/evcxr-jupyter-integration.html)をクリックしてください。
- WASMの例を見るには、この[リンク](https://plumberserver.com/plotters-wasm-demo/index.html)をクリックしてください。
- 現在、コンソールプロッティングのための内部コードはすべて準備できていますが、コンソールベースのバックエンドはまだ準備できていません。バックエンドをカスタマイズしてコンソールでプロットする方法については、[こちらの例](https://github.com/38/plotters/blob/master/examples/console.rs)をご覧ください。

---

### Table of Contents

- [Gallery](https://docs.rs/plotters/0.3.0/plotters/#gallery)
- [Quick Start](https://docs.rs/plotters/0.3.0/plotters/#quick-start)
- [Trying with Jupyter evcxr Kernel Interactively](https://docs.rs/plotters/0.3.0/plotters/#trying-with-jupyter-evcxr-kernel-interactively)
- [Interactive Tutorial with Jupyter Notebook](https://docs.rs/plotters/0.3.0/plotters/#interactive-tutorial-with-jupyter-notebook)
- [Plotting in Rust](https://docs.rs/plotters/0.3.0/plotters/#plotting-in-rust)
- [Plotting on HTML5 canvas with WASM Backend](https://docs.rs/plotters/0.3.0/plotters/#plotting-on-html5-canvas-with-wasm-backend)
- [What types of figure are supported?](https://docs.rs/plotters/0.3.0/plotters/#what-types-of-figure-are-supported)
- [Concepts by examples](https://docs.rs/plotters/0.3.0/plotters/#concepts-by-examples)
  - [Drawing Back-ends](https://docs.rs/plotters/0.3.0/plotters/#drawing-backends)
  - [Drawing Area](https://docs.rs/plotters/0.3.0/plotters/#drawing-area)
  - [Elements](https://docs.rs/plotters/0.3.0/plotters/#elements)
  - [Composable Elements](https://docs.rs/plotters/0.3.0/plotters/#composable-elements)
  - [Chart Context](https://docs.rs/plotters/0.3.0/plotters/#chart-context)
- [Misc](https://docs.rs/plotters/0.3.0/plotters/#misc)
  - [Development Version](https://docs.rs/plotters/0.3.0/plotters/#development-version)
  - [Reducing Depending Libraries && Turning Off Backends](https://docs.rs/plotters/0.3.0/plotters/#reducing-depending-libraries--turning-off-backends)
  - [List of Features](https://docs.rs/plotters/0.3.0/plotters/#list-of-features)
- [FAQ List](https://docs.rs/plotters/0.3.0/plotters/#faq-list)

---

### Quick Start

`Plotters`を使用するには、`Cargo.toml`に`Plotters`を追加するだけです。

```
[dependencies]
plotters = "0.3.0"
```

以下のコードは2次関数を描画します。 

`src/main.rs`

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
```

---

#### Trying with Jupyter evcxr Kernel Interactively

`Plotters`が`evcxr`との連携に対応し、`Jupyter Notebook`でインタラクティブにプロットを描くことができるようになりました。`Jupyter Notebook`に`Plotters`を組み込む際には、`evcxr`の機能を有効にする必要があります。

以下のコードは、その最小の例です。

```rust
:dep plotters = { git = "https://github.com/38/plotters", default_features = false, features = ["evcxr"] }
extern crate plotters;
use plotters::prelude::*;

let figure = evcxr_figure((640, 480), |root| {
    root.fill(&WHITE);
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
        &RED,
    )).unwrap()
        .label("y = x^2")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20,y)], &RED));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
});
figure
```

---

#### Interactive Tutorial with Jupyter Notebook

このチュートリアルは現在進行中であり、完全ではありません。

`evcxr`のおかげで、`Plotters`用のインタラクティブなチュートリアルができました! インタラクティブなノートブックを使用するには、`Jupyter`と`evcxr`がコンピュータにインストールされている必要があります。以下の[このページ](https://github.com/google/evcxr/tree/master/evcxr_jupyter)の指示に従って、インストールしてください。

その後、ローカルでJupyterサーバーを起動し、チュートリアルを読み込むことができるようになるはずです

```
git clone https://github.com/38/plotters-doc-data
cd plotteres-doc-data
jupyter notebook
```

そして、`evcxr-jupyter-integration.ipynb`というノートブックを選択します。

また、このノートブックの静的なHTMLバージョンは、[以下の場所](https://plumberserver.com/plotters-docs/evcxr-jupyter-integration.html)で入手できます。

---

#### Plotting in Rust

Rustはデータビジュアライゼーションに最適な言語です。様々な言語で成熟したビジュアライゼーションライブラリはたくさんありますが しかし、Rustはそのニーズに最も適した言語の一つです。

- 使いやすさ：

  Rustには、標準ライブラリに非常に優れたイテレータシステムが組み込まれています。イテレータの助けを借りれば、Rustでの作図はほとんどの高レベルプログラミング言語と同じくらい簡単になります。Rustベースのプロットライブラリは、非常に簡単に使用することができます。

- 速さ：

  何兆ものデータポイントを持つ図をレンダリングする必要がある場合、Rustは良い選択です。Rustの性能は、データ処理のステップとレンダリングのステップを1つのアプリケーションにまとめることができます。JavascriptやPythonなどの高レベルプログラミング言語で作図する場合、パフォーマンスを考慮して、作図プログラムに投入する前にデータポイントをダウンサンプリングしなければなりません。Rustはデータ処理と可視化を1つのプログラムで行うのに十分な速度を持っています。膨大な量のデータを扱うアプリケーションに図の描画コードを組み込んで、リアルタイムに可視化することも可能です。

- WebAssemblyサポート：

   RustはWASMサポートが充実している数少ない言語の一つです。Rustでのプロットは、Webページでのビジュアライゼーションに非常に役立ち、Javascriptに比べて大幅なパフォーマンスの向上が期待できます。

---

#### Plotting on HTML5 canvas with WASM Backend

Plottersは現在、HTML5キャンバスを使用するバックエンドをサポートしています。WASM サポートを使用するには、他のバックエンドの代わりに`CanvasBackend`を使用するだけで、他の API はすべて同じままです。

このリポジトリの`examples/wasm-demo`ディレクトリに、Plotters + WASMの小さなデモがあります。デプロイされたバージョンで遊ぶには、この[リンク](https://plumberserver.com/plotters-wasm-demo/index.html)をクリックしてください。

---

#### Concepts by examples

---

#### Drawing Back-ends

プロッタは、SVG、BitMap、さらにはリアルタイムレンダリングなど、さまざまな描画バックエンドを使用できます。例えば、ビットマップの描画バックエンド。

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a 800*600 bitmap and start drawing
    let mut backend = BitMapBackend::new("plotters-doc-data/1.png", (300, 200));
    // And if we want SVG backend
    // let backend = SVGBackend::new("output.svg", (800, 600));
    backend.draw_rect((50, 50), (200, 150), &RED, true)?;
    Ok(())
}
```

---

#### Drawing Area

プロッターでは、レイアウトのために描画領域という概念を用います。プロッターでは、1つの画像に複数の統合をサポートしています。これは、サブ描画領域を作成することによって行われます。

また、描画エリアでは、カスタマイズされた座標系を使用することができ、これにより、座標マッピングが自動的に行われます。

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_drawing_area =
        BitMapBackend::new("plotters-doc-data/2.png", (300, 200)).into_drawing_area();
    // And we can split the drawing area into 3x3 grid
    let child_drawing_areas = root_drawing_area.split_evenly((3, 3));
    // Then we fill the drawing area with different color
    for (area, color) in child_drawing_areas.into_iter().zip(0..) {
        area.fill(&Palette99::pick(color))?;
    }
    Ok(())
}
```

---

#### Elements

プロッターでは、要素は図形の構成要素です。すべての要素は、描画領域に描画することができます。内蔵されている要素には、線、テキスト、円など、さまざまな種類があります。また、アプリケーションコードで独自の要素を定義することもできます。

また、既存の要素を組み合わせて、複雑な要素を作ることもできます。

要素システムについて詳しく知りたい方は、[element module documentation](https://docs.rs/plotters/0.3.0/plotters/element/index.html)をご覧ください。

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/3.png", (300, 200)).into_drawing_area();
    root.fill(&WHITE)?;
    // Draw an circle on the drawing area
    root.draw(&Circle::new(
        (100, 100),
        50,
        Into::<ShapeStyle>::into(&GREEN).filled(),
    ))?;
    Ok(())
}
```

---

#### Composable Elements

組み込み要素の他に、要素を合成して論理グループを作ることができます。新しい要素を構成する際には、ターゲット座標に左上隅が与えられ、左上隅を`(0,0)`と定義した新しいピクセルベースの座標が、さらなる要素構成の目的のために使用されます。

例えば、ドットとその座標を含む要素があるとします。

```rust
use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/4.png", (640, 480)).into_drawing_area();

    root.fill(&RGBColor(240, 200, 200))?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..640, 0..480),
    ));

    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("({:.2},{:.2})", x, y),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            );
    };

    root.draw(&dot_and_label(0.5, 0.6))?;
    root.draw(&dot_and_label(0.25, 0.33))?;
    root.draw(&dot_and_label(0.8, 0.8))?;
    Ok(())
}
```

---

#### Chart Context

Plottersでチャートを描くためには、ChartContextと呼ばれる、描画領域の上に構築されたデータオブジェクトが必要です。`ChartContext`は、描画領域に比べてさらに上位の構成要素を定義します。例えば、チャート・コンテキスト・オブジェクトの助けを借りて、ラベル・エリア、メッシュを定義したり、データ・シリーズを描画エリアに置くことができます。

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/5.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
        &RED,
    ))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    Ok(())
}
```

---

### Misc

---

#### Development Version

最新の開発バージョンを使用するには、Cargo.tomlにてhttps://github.com/38/plotters.gitをpullします。

```
[dependencies]
plotters = { git = "https://github.com/38/plotters.git" }
```

---

#### Reducing Depending Libraries && Turning Off Backends

`Plotters`では、バックエンドの依存関係を制御するための機能を使用できるようになりました。デフォルトでは、`BitMapBackend`と`SVGBackend`がサポートされています。`Cargo.toml`の依存関係の記述で`default_features = false`を使用すると、バックエンドの実装を選択することができます。

- `svg` Enable the `SVGBackend`
- `bitmap` Enable the `BitMapBackend`

例えば、次のような依存関係の記述があると、ビットマップをサポートしたコンパイルを避けることができます。

```
[dependencies]
plotters = { git = "https://github.com/38/plotters.git", default_features = false, features = ["svg"] }
```

また、このライブラリでは、コンシューマーが`Palette`クレートのカラータイプをデフォルトで利用することができます。この動作は、`default_features = false`を設定することでオフにすることもできます。

---

### List of Features

これが`Plotters`クレートで定義されている機能の全リストです。`default_features = false`を使用して、デフォルトで有効になっている機能を無効にすると、`Plotters`クレートに含めたい機能を選ぶことができるようになります。そうすることで、依存関係の数を`itertools`だけにまで減らし、コンパイル時間を 6 秒以下にすることができます。

以下のリストは、オプトイン/アウトが可能な機能の完全なリストです。

- Tier 1 drawing backends

  | Name           | Description                                                  | Additional Dependency     | Default? |
  | :------------- | :----------------------------------------------------------- | :------------------------ | :------- |
  | bitmap_encoder | `BitMapBackend`が結果をビットマップファイルに保存できるようにする | image, rusttype, font-kit | Yes      |
  | svg_backend    | `SVGBackend`サポートの有効化                                 | None                      | Yes      |
  | bitmap_gif     | Opt-in GIF animation `BitMapBackend` のレンダリングサポート、`bitmap` の有効化を示唆 | gif                       | Yes      |

- Font manipulation features

| Name | Description                            | Additional Dependency | Default? |
| :--- | :------------------------------------- | :-------------------- | :------- |
| ttf  | TrueTypeフォントのサポートを可能にする | rusttype, font-kit    | Yes      |

- Coordinate features

| Name     | Description            | Additional Dependency | Default? |
| :------- | :--------------------- | :-------------------- | :------- |
| datetime | 日付と時刻の調整が可能 | chrono                | Yes      |

- Element, series and util functions

| Name         | Description               | Additional Dependency | Default? |
| :----------- | :------------------------ | :-------------------- | :------- |
| errorbar     | errorbar要素のサポート    | None                  | Yes      |
| candlestick  | candlestick要素のサポート | None                  | Yes      |
| boxplot      | boxplot要素のサポート     | None                  | Yes      |
| area_series  | area seriesサポート       | None                  | Yes      |
| line_series  | line seriesサポート       | None                  | Yes      |
| histogram    | histogram seriesサポート  | None                  | Yes      |
| point_series | point seriesサポート      | None                  | Yes      |

- Misc

| Name             | Description                                                  | Additional Dependency | Default? |
| :--------------- | :----------------------------------------------------------- | :-------------------- | :------- |
| deprecated_items | この機能により、将来削除される予定の非推奨アイテムを使用することができます。 | None                  | Yes      |
| debug            | デバッグに使用するコードを有効にする                         | None                  | No       |

---

### FAQ

- WASMのサンプルが私のマシンで動作しないのはなぜですか？

  WASMの例では、ビルドに`Wasm32`ターゲットを使用する必要があります。`cargo`を使ったビルドでは、ほとんどの場合、x86ターゲットのいずれかであるデフォルトターゲットが使用されます。そのため、カーゴのパラメータリストに`--target=wasm32-unknown-unknown`を追加してビルドする必要があります。

- グラフの上に文字、円、点、長方形などを描くには？

  お気づきのように、Plottersは伝統的なデータプロッティングライブラリではなく、ドローイングライブラリなので、描画領域に何でも自由に描くことができます。描画領域に任意の要素を描くには`DrawArea::draw`を使います。

---

### Module

| [chart](https://docs.rs/plotters/0.3.0/plotters/chart/index.html) | The high-level plotting abstractions.                        |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| [coord](https://docs.rs/plotters/0.3.0/plotters/coord/index.html) | Plottersの重要な特徴の一つは、柔軟な座標系の抽象化であり、このモジュールはPlottersの座標抽象化に使用されるすべての抽象化を提供します。 |
| [data](https://docs.rs/plotters/0.3.0/plotters/data/index.html) | データの可視化に関するアルゴリズムを実装したデータ処理モジュール。ダウンサンプリングなど。 |
| [drawing](https://docs.rs/plotters/0.3.0/plotters/drawing/index.html) | Plotters用の描画ツールです。Plottersには、低レベルAPIと高レベルAPIという2つの描画APIがあります。 |
| [element](https://docs.rs/plotters/0.3.0/plotters/element/index.html) | Plottersの描画システムにおける上位の描画単位である描画要素を定義します。 |
| [prelude](https://docs.rs/plotters/0.3.0/plotters/prelude/index.html) | このモジュールは、Plottersで最もよく使われるタイプとモジュールをインポートしています。 |
| [series](https://docs.rs/plotters/0.3.0/plotters/series/index.html) | このモジュールには、定義済みのタイプのシリーズが含まれています。Plottersの系列は、実際には要素のイテレータで、`ChartContext::draw_series\関数で取得することができます。 |
| [style](https://docs.rs/plotters/0.3.0/plotters/style/index.html) | 図形やテキストのスタイル、フォント、色などを指定します。     |

---

### plotters::prelude::BitMapBackend

- Description

  ビットマップを描画するバックエンド

- Implementation

  - plotters::prelude::BitMapBackend::new

    新しいビットマップバックエンドの作成

---

### plotters::drawing::IntoDrawingArea

- Description

  根幹の描画領域に変換可能なタイプ

- Required methods

  - `into_drawing_area`

    ```rust
    fn into_drawing_area(self) -> DrawingArea<Self, Shift>
    ```

    タイプをルートの描画領域に変換

---

### plotters::prelude::ChartBuilder

- Description

  高レベルの図形描画に使用されるチャート・コンテキストを作成するためのヘルパー・オブジェクトです。このオブジェクトを使うと、基本的な描画領域をチャート・コンテキストに変換することができ、描画領域で高レベルのチャートAPIを使うことができます。

- Implementation

  - protters::prelude::ChartBuilder::on

    与えられた描画領域にチャートビルダーを作成する

    - root:ルートの描画領域
    - Return:チャートビルダーのオブジェクト

  - plotters::prelude::ChartBuilder::build_cartesian_2d

    2次元のデカルト座標系でチャートを作成します。この関数は、データシリーズがレンダリングされるチャートコンテキストを返します。

    - x_spec: X軸の仕様
    - y_spec: Y軸の仕様
    - Retrurn: チャートコンテキスト
    
  - plotters::prelude::ChartBuilder::set_label_area_size
  
    ラベルエリアサイズの設定
  
    - `caption`: グラフのキャプション
    - `size`: ラベルエリアサイズの大きさ
  
  - `plotters::char::ChartBuilder::caption`
  
    チャートのキャプションを設定する
  
    - `caption`: グラフのキャプション
    - `style`: テキストのスタイル
    - Note: キャプションが設定されている場合、マージンオプションは無視されます。

---

### plotters::prelude::LavelAreaPosition

- Description

  ```rust
  pub enum LabelAreaPosition {
      Top,
      Bottom,
      Left,
      Right,
  }
  ```

  ラベル領域の位置を指定するために使用される列挙体です。これは、API `ChartBuilder::set_label_area_size`(struct ChartBuilder.html#method.set_label_area_size)を使用してラベル領域のサイズを構成するときに使用されます。

---

### `plotters::chart::ChartContext`

- Description

  チャートのコンテキスト。これはPlottersの中核となるオブジェクトです。任意のプロット/チャートはこのタイプとして抽象化され、任意のデータ シリーズをチャート コンテキストに配置できます。

  - チャート コンテキストに系列を描画するには、`ChartContext::draw_series`を使用します。
  - 単一の要素をチャートに描画するには、`ChartContext::plotting_area`を使用します。

- Implementation

  - `plotters::chart::Chartcontext::draw_series`

    データ系列を描画します。Plottersのデータ系列は、要素のイテレータとして抽象化されています。
    
  - `plotters::chart::Chartcontext::configure_mesh`
  
    メッシュ構成オブジェクトを初期化し、関数`MeshStyle::draw`を呼び出すことで、メッシュの描画が確定します。

---

### `plotters::chart::MeshStyle`

- Description

  任意のチャートのメッシュの構成を追跡するために使用される構造体です。

- Implementation

  - `plotters::chart::MeshStyle::draw`

    設定したメッシュをターゲットプロットに描画

---

### plotters::prelude::LineSeries

- Description

  ゲスト座標系のポイントのイテレータを受け取り、ラインプロットをレンダリングする要素を作成するラインシリーズオブジェクトです。

- Implementation

  - plotters::prelude::LineSeries::new

    ```rust
    pub fn new<I: IntoIterator<Item = Coord>, S: Into<ShapeStyle>>(
        iter: I,
        style: S
    ) -> Self
    ```

    

---

### plotters::drawing::DrawingArea::fill

- Description

  描画領域全体を色で塗りつぶす

---

### plotters::style::colors

- Description

  基本的な定義済みの色。

- Constants

  - BLACK
  - BLUE
  - CYAN
  - GREEAN
  - MAGENTA
  - RED
  - TRANSPARENT
  - WHITE
  - YELLOW

---



