use std::env;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;

fn main() -> anyhow::Result<()> {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        panic!("please provide two sentences as arguments to calculate cosine similarity")
    }
    // Set-up sentence embeddings model
    let model = SentenceEmbeddingsBuilder::local("models/all-MiniLM-L12-v2")
        .create_model()?;

    // Generate Embeddings
    let embeddings = model.encode(&args[1..3])?;
    
    let similarity = cosine_similarity(&embeddings[0], &embeddings[1]);
    println!("{:?}", similarity);
    Ok(())
}

fn cosine_similarity(left: &Vec<f32>, right: &Vec<f32>) -> f32 {
    if left.len() != right.len() {
        panic!("embeddings should have the same length")
    }

    let (norm_l, norm_r, norm_prod) = left.iter()
        .zip(right.iter())
        .fold(
            (0f32,0f32,0f32),
            |(norm_a, norm_b, norm_prod), (l, r)| {
            (norm_a + l*l, norm_b + r*r, norm_prod + l*r)
        });
    norm_prod / (norm_l.sqrt() * norm_r).sqrt()
}