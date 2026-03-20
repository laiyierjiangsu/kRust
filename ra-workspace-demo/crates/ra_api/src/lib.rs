//! Facade over `ra_core`: re-exports and helpers (cross-crate `use` targets).

use ra_core::Describe;

pub use ra_core::WidgetId;

#[cfg(feature = "full")]
pub use ra_core::WidgetMeta;

/// Format anything that implements [`Describe`] from `ra_core`.
pub fn format_describable(value: &impl Describe) -> String {
    value.describe()
}

/// Available when `full` is enabled (chain: `ra_sandbox` → `full` → `ra_core/extended`).
#[cfg(feature = "full")]
pub fn summarize_meta(meta: &crate::WidgetMeta) -> String {
    meta.describe()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_formats_core_type() {
        let id = WidgetId::new(3);
        assert_eq!(format_describable(&id), "widget#3");
    }
}
