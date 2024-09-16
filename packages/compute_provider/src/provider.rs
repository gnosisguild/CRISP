use crate::compute_input::ComputeInput;

pub trait ComputeOutput {
    fn ciphertext(&self) -> &Vec<u8>;
    fn merkle_root(&self) -> String;
    fn params_hash(&self) -> String;
}

pub trait ComputeProvider {
    type Output: ComputeOutput + Send + Sync;

    fn prove(
        &self,
        input: &ComputeInput
    ) -> Self::Output;
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputeResult {
    pub ciphertext: Vec<u8>,
    pub params_hash: String,
    pub merkle_root: String,
}

impl ComputeOutput for ComputeResult {
    fn ciphertext(&self) -> &Vec<u8> {
        &self.ciphertext
    }

    fn merkle_root(&self) -> String {
        self.merkle_root.clone()
    }

    fn params_hash(&self) -> String {
        self.params_hash.clone()
    }
}
