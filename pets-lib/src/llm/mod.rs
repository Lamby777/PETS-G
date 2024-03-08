use crate::prelude::*;

use godot::engine::file_access::ModeFlags;
use godot::engine::GFile;

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

/// returns the model loaded from disk
fn load_llm() -> Bloom {
    let model_path = get_llm_path().unwrap();
    llm::load(
        &model_path,
        llm::TokenizerSource::Embedded,
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"))
}

pub fn llm_generate() {
    let model = load_llm();

    let mut session = model.start_session(Default::default());
    let res = session.infer::<Infallible>(
        &model,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: Prompt::Text("AI is interesting... Maybe someday it can"),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        |response| {
            use InferenceResponse::*;
            match response {
                PromptToken(t) | InferredToken(t) => {
                    print!("{t}");
                    std::io::stdout().flush().unwrap();
                }
                _ => {}
            }

            Ok(InferenceFeedback::Continue)
        },
    );

    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
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
