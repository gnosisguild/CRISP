use rfv::enclave_server::start_server;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    start_server()
}
