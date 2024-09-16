use alloy::{
    primitives::{Address, B256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{BlockNumberOrTag, Filter, Log},
    sol_types::SolEvent,
    transports::BoxTransport,
};
use eyre::Result;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use log::{info, error};

use super::events::{E3Activated, InputPublished, PlaintextOutputPublished};

pub trait ContractEvent: Send + Sync + 'static {
    fn process(&self, log: Log) -> Result<()>;
}

// impl<T> ContractEvent for T
// where
//     T: SolEvent + Debug + Send + Sync + 'static,
// {
//     fn process(&self) -> Result<()> {
//         println!("Processing event: {:?}", self);
//         Ok(())
//     }
// }

pub struct EventListener {
    provider: Arc<RootProvider<BoxTransport>>,
    filter: Filter,
    handlers: HashMap<B256, Arc<dyn Fn(Log) -> Result<Box<dyn ContractEvent>> + Send + Sync>>,
}

impl EventListener {
    pub fn new(provider: Arc<RootProvider<BoxTransport>>, filter: Filter) -> Self {
        Self {
            provider,
            filter,
            handlers: HashMap::new(),
        }
    }

    pub fn add_event_handler<E>(&mut self)
    where
        E: SolEvent + ContractEvent + 'static,
    {
        let signature = E::SIGNATURE_HASH;
        let handler = Arc::new(move |log: Log| -> Result<Box<dyn ContractEvent>> {
            let event = log.log_decode::<E>()?.inner.data;
            Ok(Box::new(event))
        });

        self.handlers.insert(signature, handler);
    }

    pub async fn listen(&self) -> Result<()> {
        let mut stream = self
            .provider
            .subscribe_logs(&self.filter)
            .await?
            .into_stream();
        while let Some(log) = stream.next().await {
            if let Some(topic0) = log.topic0() {
                if let Some(decoder) = self.handlers.get(topic0) {
                    match decoder(log.clone()) {
                        Ok(event) => {
                            event.process(log)?;
                        }
                        Err(e) => {
                            println!("Error decoding event 0x{:x}: {:?}", topic0, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub struct ContractManager {
    provider: Arc<RootProvider<BoxTransport>>,
}

impl ContractManager {
    pub async fn new(rpc_url: &str) -> Result<Self> {
        let provider = ProviderBuilder::new().on_builtin(rpc_url).await?;
        Ok(Self {
            provider: Arc::new(provider),
        })
    }

    pub fn add_listener(&self, contract_address: Address) -> EventListener {
        let filter = Filter::new()
            .address(contract_address)
            .from_block(BlockNumberOrTag::Latest);

        EventListener::new(self.provider.clone(), filter)
    }
}
pub async fn start_listener(contract_address: &str) -> Result<()> {
    let rpc_url = "ws://127.0.0.1:8545";
    let address: Address = contract_address.parse()?;
    
    loop {
        match run_listener(rpc_url, address).await {
            Ok(_) => {
                info!("Listener finished successfully. Checking for reconnection...");
            },
            Err(e) => {
                error!("Error occurred in listener: {}. Reconnecting after delay...", e);
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
}

// Separate function to encapsulate listener logic
async fn run_listener(rpc_url: &str, contract_address: Address) -> Result<()> {
    let manager = ContractManager::new(rpc_url).await?;
    
    let mut listener = manager.add_listener(contract_address);
    listener.add_event_handler::<E3Activated>();
    listener.add_event_handler::<InputPublished>();
    listener.add_event_handler::<PlaintextOutputPublished>();

    loop {
        match listener.listen().await {
            Ok(_) => {
                info!("Listener is still active...");
            }
            Err(e) => {
                error!("Connection lost or error occurred: {}. Attempting to reconnect...", e);
                break;
            }
        }
    }
    
    Ok(())
}