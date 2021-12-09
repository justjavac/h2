pub use tracing;
pub use tracing_subscriber;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() -> tracing::dispatcher::DefaultGuard {
    let use_colors = atty::is(atty::Stream::Stderr);
    let layer = tracing_tree::HierarchicalLayer::default()
        .with_writer(std::io::stderr)
        .with_indent_lines(true)
        .with_ansi(use_colors)
        .with_targets(true)
        .with_indent_amount(2);

    tracing_subscriber::Registry::default()
        .with(layer)
        .set_default()
}
