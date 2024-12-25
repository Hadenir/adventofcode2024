
pub type Page = u64;

#[derive(Debug, Clone, Copy)]
pub struct OrderingRule {
    pub before: Page,
    pub after: Page,
}
