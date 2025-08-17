use crate::prelude::*;

use godot::classes::file_access::ModeFlags;
use godot::classes::{DirAccess, GFile};
use godot::prelude::*;

use std::io::Write;

use llm::models::Bloom;
use llm::{InferenceFeedback, InferenceResponse, Model as _, Prompt};

// default model taken from
// https://huggingface.co/rustformers/bloom-ggml/blob/main/bloom-1b7-q4_0.bin
const LLM_PATH_RES: &str = "res://assets/default-model.llm";
const LLM_PATH_USER: &str = "user://model.llm";

/// get the path of the pretrained model
fn prep_llm_path() -> Result<PathBuf> {
    // Open the model file, or copy it out first if not exists
    let open_it = || GFile::open(LLM_PATH_USER, ModeFlags::READ);
    let mut file = open_it();

    // TODO refactor into an unwrap_or_else or something
    if let Err(_) = file {
        // copy the dang file!!1!1111
        godot_print!("Copying model... This may take a while.");
        let mut dir = DirAccess::open("res://assets/".into()).unwrap();
        dir.copy(LLM_PATH_RES.into(), LLM_PATH_USER.into());
        godot_print!("Done!");

        // try again
        file = open_it();
    };

    let file = file?;

    let path = file.path_absolute().to_string();
    path.parse()
        .map_err(|e| anyhow!("Failed to parse path: {}", e))
}

fn load_llm() -> Bloom {
    let model_path = prep_llm_path().unwrap();
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
    let res = session.infer::<!>(
        &model,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: Prompt::Text("ML is interesting... Maybe someday it can"),
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

pub struct _LLMInterface;

impl _LLMInterface {
    pub fn _preprocess_shopkeeper_joke(joke: &str) -> String {
        format!(include_str!("prompts/shopkeeper_joke.txt"), joke)
    }

    pub fn _preprocess_devon_shop(intro: &str) -> String {
        format!(include_str!("prompts/devon_shop.txt"), intro)
    }
}
