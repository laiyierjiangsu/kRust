//! Minimal domain types for cross-crate rust-analyzer exercises.

/// Opaque identifier (jump from `ra_api` / `ra_sandbox` here).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u64);

impl WidgetId {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Trait implemented by several types (test go-to-impl / method resolution).
pub trait Describe {
    fn describe(&self) -> String;
}

impl Describe for WidgetId {
    fn describe(&self) -> String {
        format!("widget#{}", self.0)
    }
}

/// Only available with `extended` feature (tests cfg / feature graphs in rust-analyzer).
#[cfg(feature = "extended")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WidgetMeta {
    pub id: WidgetId,
    pub label: String,
}

#[cfg(feature = "extended")]
impl Describe for WidgetMeta {
    fn describe(&self) -> String {
        format!("{} ({})", self.label, self.id.describe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_id_describe() {
        let id = WidgetId::new(7);
        assert_eq!(id.describe(), "widget#7");
    }
}
