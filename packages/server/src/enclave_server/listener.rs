use alloy::{
    primitives::{address, Address, B256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{BlockNumberOrTag, Filter, Log},
    sol,
    sol_types::SolEvent,
    transports::BoxTransport,
};
use eyre::Result;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub trait ContractEvent: Send + Sync + 'static {
    fn process(&self) -> Result<()>;
}

impl<T> ContractEvent for T
where
    T: SolEvent + Debug + Send + Sync + 'static,
{
    fn process(&self) -> Result<()> {
        println!("Processing event: {:?}", self);
        Ok(())
    }
}

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
                    if let Ok(event) = decoder(log.clone()) {
                        event.process()?;
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

sol! {
    #[derive(Debug)]
    event TestingEvent(uint256 e3Id, bytes input);
}

#[tokio::main]
async fn start_listener() -> Result<()> {
    let rpc_url = "ws://127.0.0.1:8545";

    let manager = ContractManager::new(rpc_url).await?;

    let address1 = address!("e7f1725E7734CE288F8367e1Bb143E90bb3F0512");
    let mut listener1 = manager.add_listener(address1);
    listener1.add_event_handler::<TestingEvent>();

    let address2 = address!("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    let mut listener2 = manager.add_listener(address2);
    listener2.add_event_handler::<TestingEvent>();

    tokio::try_join!(listener1.listen(), listener2.listen())?;

    Ok(())
}