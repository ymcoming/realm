use tokio;
use tokio::runtime::Builder;
use tokio::task;
mod relay;
mod resolver;
mod udp;

fn main() {
    let relay_configs = realm::parse_arguments();
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let local = task::LocalSet::new();
    rt.block_on(local.run_until(relay::start_relay(relay_configs)));
}
