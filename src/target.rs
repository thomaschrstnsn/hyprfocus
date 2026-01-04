use crate::args::Target;

impl Target {
    pub fn execute(&self) {
        tracing::info!(?self, "hello");
    }
}
