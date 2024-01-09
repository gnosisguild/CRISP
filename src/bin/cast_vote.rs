mod util;

use std::{env, error::Error, process::exit, sync::Arc};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng};
use util::timeit::{timeit, timeit_n};
use ethers::{
    prelude::{abigen, Abigen},
    providers::{Http, Provider},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, Bytes},
    core::k256,
    utils,
};
use std::fs;
use std::path::Path;

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("casting encrypted vote");

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
    //println!("{:?}", pk);
    //let () = pk;
    let test = pk.to_bytes();
    //xsprintln!("{:?}", test);
    // voting	
    let dist = Uniform::new_inclusive(0, 1);
    let votes: Vec<u64> = dist
        .sample_iter(&mut thread_rng())
        .take(num_voters)
        .collect();
    println!("plaintext vote: {:?}", votes);
    let mut votes_encrypted = Vec::with_capacity(num_voters);
    let mut _i = 0;
    timeit_n!("Vote casting (single vote)", num_voters as u32, {
        #[allow(unused_assignments)]
        let pt = Plaintext::try_encode(&[votes[_i]], Encoding::poly(), &params)?;
        let ct = pk.try_encrypt(&pt, &mut thread_rng())?;
        votes_encrypted.push(ct);
        _i += 1;
    });

    //println!("{:?}", votes_encrypted[0]);
    //println!("{:?}", votes_encrypted[0].to_bytes());
    let sol_vote = Bytes::from(votes_encrypted[0].to_bytes());
    //println!("{:?}", votes_encrypted[0].to_bytes());
    //println!("{:?}", sol_vote);

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
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#,
    );

    //const RPC_URL: &str = "https://eth.llamarpc.com";
    const WETH_ADDRESS: &str = "0xa5839eaFDc528D977BaEd88172929E71A16c49Ee";

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let wallet: LocalWallet = "66c6c4603b762de30ec1eedaa7c865ba29308218648980efdcf0b35f887db644"
        .parse::<LocalWallet>()?
        .with_chain_id(5 as u64);

    // 6. Wrap the provider and wallet together to create a signer client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    //let client = Arc::new(provider);
    let address: Address = WETH_ADDRESS.parse()?;
    let contract = IERC20::new(address, Arc::new(client.clone()));

    if let Ok(total_supply) = contract.tester().call().await {
        println!("Test value is {total_supply:?}");
    }

    let address_from = "0x8B3B79D6953C9B68E534309ab19047cB37b81249".parse::<Address>()?;
    let address_coord = "0x7735b940d673344845aC239CdDddE1D73b5d5627".parse::<Address>()?;

    //contract.increment(address_from, U256::from(utils::parse_ether(1)?)).send().await?;

    contract.vote_encrypted(sol_vote).send().await?.await?;

    if let Ok(id) = contract.id().call().await {
        println!("id is {id:?}");
    }

    // if let Ok(chain_vote_bytes) = contract.getVote(address_coord).call().await {
    //     println!("{:?}", chain_vote_bytes);
    // }

    //let path = "vote.txt";
    //fs::write(path, votes_encrypted[0].to_bytes()).unwrap();

    Ok(())
}
