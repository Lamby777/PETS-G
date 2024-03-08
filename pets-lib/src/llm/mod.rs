use crate::prelude::*;

use io::Write;

use llm::models::Bloom;
use llm::Model as _;
use llm::{InferenceFeedback, InferenceResponse, Prompt};

/// get the path of the pretrained model
fn get_llm_path() -> Result<PathBuf> {
    // Open file in read mode
    let model_file = GFile::open("res://assets/llm.bin", ModeFlags::READ)?;

    let path = model_file.path_absolute().to_string();

    path.parse()
        .map_err(|e| anyhow!("Failed to parse path: {}", e))
}

fn load_llm() -> Bloom {
    let model_path = get_llm_path().unwrap();
    // load a GGML model from disk
    llm::load(
        Path::new("/path/to/model"),
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"))
}

fn llm_generate() {
    todo!()

    // let model = load_llm();
    //
    // let mut session = model.start_session(Default::default());
    // let res = session.infer::<std::convert::Infallible>(
    //     // model to use for text generation
    //     &model,
    //     // randomness provider
    //     &mut rand::thread_rng(),
    //     // the prompt to use for text generation, as well as other
    //     // inference parameters
    //     &llm::InferenceRequest {
    //         prompt: "Rust is a cool programming language because",
    //         ..Default::default()
    //     },
    //     // llm::OutputRequest
    //     &mut Default::default(),
    //     // output callback
    //     |t| {
    //         print!("{t}");
    //         io::stdout().flush().unwrap();
    //
    //         Ok(())
    //     },
    // );
}

pub struct LLMInterface;

impl LLMInterface {
    pub fn preprocess_shopkeeper_joke(joke: &str) -> String {
        format!(include_str!("prompts/shopkeeper_joke.txt"), joke)
    }

    pub fn preprocess_devon_shop(intro: &str) -> String {
        format!(include_str!("prompts/devon_shop.txt"), intro)
    }
}
