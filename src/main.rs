use byteorder::{ReadBytesExt, LE};
use std::fs::File;
use std::path::Path;

/// Configuration parameters for a transformer model.
///
/// This struct contains various hyperparameters and settings for a transformer model.
/// Each field corresponds to a specific configuration parameter:
///
/// - `dim`: The dimensionality of the model. This is typically the size of the embeddings.
/// - `hidden_dim`: The dimensionality of the "hidden" layers in the feedforward network (FFN).
/// - `n_layers`: The number of layers in the transformer model.
/// - `n_heads`: The number of "query" heads for the multi-head attention mechanism.
/// - `n_kv_heads`: The number of "key/value" heads. This can be less than `n_heads` for multiquery attention mechanisms.
/// - `vocab_size`: The size of the vocabulary. This is typically 256 for byte-level models.
/// - `seq_len`: The maximum sequence length that the model can handle.
#[derive(Debug, Clone, Copy)]
struct Config {
    dim: usize,
    hidden_dim: usize,
    n_layers: usize,
    n_heads: usize,
    n_kv_heads: usize,
    vocab_size: usize,
    seq_len: usize,
}

impl Config {
    /// Constructs a `Config` from a binary file at the given path.
    ///
    /// The file should contain exactly 7 `i32` values in little endian order,
    /// each representing a configuration field in the following order:
    /// `dim`, `hidden_dim`, `n_layers`, `n_heads`, `n_kv_heads`, `vocab_size`, `seq_len`.
    fn read_model_file(path: &Path) -> Self {
        let mut model = File::open(path).expect(
            "Failed to open model file at the given path. Please check the path and try again.",
        );

        Self {
            dim: model.read_i32::<LE>().unwrap() as usize,
            hidden_dim: model.read_i32::<LE>().unwrap() as usize,
            n_layers: model.read_i32::<LE>().unwrap() as usize,
            n_heads: model.read_i32::<LE>().unwrap() as usize,
            n_kv_heads: model.read_i32::<LE>().unwrap() as usize,
            vocab_size: model.read_i32::<LE>().unwrap() as usize,
            seq_len: model.read_i32::<LE>().unwrap() as usize,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
