#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetCode(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GizmoCode(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}
