
const SOLIDITY_IMAGE_ID_PATH: &str = "../../evm/contracts/ImageID.sol";
const SOLIDITY_ELF_PATH: &str = "../../evm/contracts/Elf.sol";

fn main() {
    let guests = risc0_build::embed_methods();
    let solidity_opts = risc0_build_ethereum::Options::default()
    .with_image_id_sol_path(SOLIDITY_IMAGE_ID_PATH)
    .with_elf_sol_path(SOLIDITY_ELF_PATH);

    risc0_build_ethereum::generate_solidity_files(guests.as_slice(), &solidity_opts).unwrap();
}