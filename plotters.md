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

