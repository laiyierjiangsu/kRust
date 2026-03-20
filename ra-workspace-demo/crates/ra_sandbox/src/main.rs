//! Entry point: depends on `ra_api` only (indirect edge to `ra_core`).

use ra_api::{format_describable, WidgetId};

fn main() {
    let id = WidgetId::new(1);
    println!("{}", format_describable(&id));

    #[cfg(feature = "full")]
    {
        use ra_api::{summarize_meta, WidgetMeta};
        let meta = WidgetMeta {
            id: WidgetId::new(2),
            label: "demo".into(),
        };
        println!("{}", summarize_meta(&meta));
    }
}
