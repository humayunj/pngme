#[derive(clap::Args, Debug)]
pub struct PrintArgs {
    pub file: String,
}

#[derive(clap::Args, Debug)]
pub struct EncodeArgs {
    pub file: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}
#[derive(clap::Args, Debug)]
pub struct DecodeArgs {
    pub file: String,
    pub chunk_type: String,
}
