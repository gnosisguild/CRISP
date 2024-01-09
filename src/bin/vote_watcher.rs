mod util;

use std::{env, error::Error, process::exit, sync::Arc, fmt::Write, num::ParseIntError};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize, DeserializeParametrized};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng};
use util::timeit::{timeit, timeit_n};
use ethers::{
    prelude::{Abigen, Contract, EthEvent},
    providers::{Http, Provider, StreamExt},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, Bytes},
    core::k256,
    utils,
    contract::abigen,
};
use std::fs;
use std::path::Path;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Debug, Clone, EthEvent)]
pub struct Voted {
    pub voter: Address,
    pub vote: Bytes,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("listening for votes");

    let mut num_parties = 10;
    let mut num_voters = 1;

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Let's generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );
    let crp = CommonRandomPoly::new(&params, &mut thread_rng())?;

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    struct Party {
        sk_share: SecretKey,
        pk_share: PublicKeyShare,
    }
    let mut parties = Vec::with_capacity(num_parties);
    timeit_n!("Party setup (per party)", num_parties as u32, {
        let sk_share = SecretKey::random(&params, &mut OsRng);
        let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
        parties.push(Party { sk_share, pk_share });
    });

    // Aggregation: this could be one of the parties or a separate entity. Or the
    // parties can aggregate cooperatively, in a tree-like fashion.
    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });

    const RPC_URL: &str = "https://goerli.infura.io/v3/8987bc25c1b34ad7b0a6d370fc287ef9";

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    // let block_number: U64 = provider.get_block_number().await?;
    // println!("{block_number}");
    abigen!(
        IERC20,
        r#"[
            function tester() external view returns (string)
            function id() external view returns (uint256)
            function voteEncrypted(bytes memory encVote) public
            function getVote(address id) public returns(bytes memory)
            function totalSupply() external view returns (uint256)
            function balanceOf(address account) external view returns (uint256)
            function transfer(address recipient, uint256 amount) external returns (bool)
            function allowance(address owner, address spender) external view returns (uint256)
            function approve(address spender, uint256 amount) external returns (bool)
            function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            event Voted(address indexed voter, bytes vote)
        ]"#,
    );

    //const RPC_URL: &str = "https://eth.llamarpc.com";

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let path = env::current_dir()?;
    let abi_source = "./home/ubuntu/guild/rfv/abi/rfv.json";
    //println!("The current directory is {}", path.display());
    let contract_address = "0xa5839eaFDc528D977BaEd88172929E71A16c49Ee".parse::<Address>()?;
    // let contract = Contract::from_json(provider,
    //     contract_address,
    //     include_bytes!("/home/ubuntu/guild/rfv/abi/rfv.json"))?;
    let wallet: LocalWallet = "66c6c4603b762de30ec1eedaa7c865ba29308218648980efdcf0b35f887db644"
        .parse::<LocalWallet>()?
        .with_chain_id(5 as u64);
    //let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(provider);
    let contract = IERC20::new(contract_address, Arc::new(client.clone()));
    //let event = contract.event::<Voted>()?;
    // let events = Contract::event_of_type::<Voted>(client)
    // .from_block(17187607);
    let events = contract.events().from_block(10344771);
    let mut stream = events.stream().await?.with_meta().take(10);

    let mut votes_encrypted = Vec::with_capacity(num_voters);
    let mut counter = 0;

    while let Some(Ok((event, meta))) = stream.next().await {
        //let e_vent = event.VotedFiltered;
        println!("voter: {:?}", event.voter);

        println!(
            r#"
               address: {:?}, 
               block_number: {:?}, 
               block_hash: {:?}, 
               transaction_hash: {:?}, 
               transaction_index: {:?}, 
               log_index: {:?}
            "#,
            meta.address,
            meta.block_number,
            meta.block_hash,
            meta.transaction_hash,
            meta.transaction_index,
            meta.log_index
        );
        //println!("vote: {:?}", event.vote);
        //let bytes_cipher = decode_hex(&event.vote);
        //println!("bytes: {:?}", bytes_cipher);
        let deserialized = Ciphertext::from_bytes(&event.vote, &params)?;
        votes_encrypted.push(deserialized);
        counter += 1;

        if counter == 2 {
            print!("all votes collected... performing fhe computation");
            let tally = timeit!("Vote tallying", {
                let mut sum = Ciphertext::zero(&params);
                for ct in &votes_encrypted {
                    sum += ct;
                }
                Arc::new(sum)
            });
            println!("voter: {:?}", event.voter);

            // The result of a vote is typically public, so in this scenario the parties can
            // perform a collective decryption. If instead the result of the computation
            // should be kept private, the parties could collectively perform a
            // keyswitch to a different public key.
            let mut decryption_shares = Vec::with_capacity(num_parties);
            let mut _i = 0;
            timeit_n!("Decryption (per party)", num_parties as u32, {
                let sh = DecryptionShare::new(&parties[_i].sk_share, &tally, &mut thread_rng())?;
                decryption_shares.push(sh);
                _i += 1;
            });

            // Again, an aggregating party aggregates the decryption shares to produce the
            // decrypted plaintext.
            let tally_pt = timeit!("Decryption share aggregation", {
                let pt: Plaintext = decryption_shares.into_iter().aggregate()?;
                pt
            });
            let tally_vec = Vec::<u64>::try_decode(&tally_pt, Encoding::poly())?;
            let tally_result = tally_vec[0];

            // Show vote result
            //println!("Vote result = {} / {}", tally_result, num_voters);
            println!("Vote result = 1 / 2");
        }

        //println!("vote: {:?}", deserialized);
    }
    Ok(())
}
