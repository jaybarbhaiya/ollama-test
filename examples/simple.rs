use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use ollama_test::consts::DEFAULT_SYSTEM_MOCK;
use ollama_test::consts::MODEL;
use ollama_test::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // by default localhost:11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "What is the best programming language?(Be concise)".to_string();

    let gen_req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    // Single response generation
    let res = ollama.generate(gen_req).await?;
    println!("->> res {}", res.response);

    Ok(())
}
