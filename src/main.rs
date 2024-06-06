use ai_writer::get_completion;

#[tokio::main]
async fn main() {
    println!("Starting...");

    let response = get_completion("What is the capital of the United States?").await;

    println!("Response: {}", response);
}
