use leptos::*;
use leptos_meta::{provide_meta_context, Link, Script, Style, Title};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,

        <Title text="Script test"/>
        // <Link rel="modulepreload" href="https://cdn.jsdelivr.net/npm/vega@5" />
        // <Link rel="modulepreload" href="https://cdn.jsdelivr.net/npm/vega-lite@5" />
        // <Link rel="modulepreload" href="https://cdn.jsdelivr.net/npm/vega-embed@6" />

        <Script src="https://cdn.jsdelivr.net/npm/vega@5" async_="false" />
        <Script src="https://cdn.jsdelivr.net/npm/vega-lite@5" async_="false" />
        <Script src="https://cdn.jsdelivr.net/npm/vega-embed@6" async_="false" />

        // <script src="https://cdn.jsdelivr.net/npm/vega@5" ></script>
        // <script src="https://cdn.jsdelivr.net/npm/vega-lite@5" ></script>
        // <script src="https://cdn.jsdelivr.net/npm/vega-embed@6" ></script>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Vega /> } />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Vega(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {cx,
        <p>"Meta Script test"</p>
        <Style>
        r#"
            #view {
                width: 90vw;
                height: 100px;
            }
        "#
        </Style>
        <script>
        r#"
            function data_to_spec(arr) {
                var data = arr.map((count, bp) => ({ count, bp }));
                var spec = {
                    "$schema": "https://vega.github.io/schema/vega-lite/v5.json",
                    width: "container",
                    height: "container",
                    data: {
                      values: data
                    },
                    mark: "area",
                    encoding: {
                      x: { "field": "bp", "type": "quantitative", "axis": { "tickCount": 20} },
                      y: { "field": "count", "type": "quantitative" }
                    },
                };
                return spec
            }
        "#
        </script>

        <div id="view" />
        {
            let data = vec![0,2,4,6,8];
            log!("Drawing test: {}", data.len());
            view! { cx,
             <script>
                "var compactData = "{serde_json::to_string(&data)}r#";
                var plotSpec = data_to_spec(compactData);
                vegaEmbed('#view', plotSpec).catch(console.error);
                "#
                </script>
            }
        }
    }
}
