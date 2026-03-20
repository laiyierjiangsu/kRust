//! Integration tests: separate crate graph edge for rust-analyzer.

use ra_api::{format_describable, WidgetId};

#[test]
fn cross_crate_public_api() {
    let id = WidgetId::new(99);
    assert_eq!(format_describable(&id), "widget#99");
}

#[cfg(feature = "full")]
#[test]
fn full_feature_chain() {
    use ra_api::{summarize_meta, WidgetMeta};
    let meta = WidgetMeta {
        id: WidgetId::new(5),
        label: "integration".into(),
    };
    assert_eq!(
        summarize_meta(&meta),
        "integration (widget#5)"
    );
}
