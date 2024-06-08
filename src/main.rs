use ai_writer::get_completion;
use ai_writer::prompt_builder;
use log::{debug, error, info, warn};
use log4rs;
use tokio_stream;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting...");

    // TODO These should be requested from the user
    let idea = "How to create an automated blog post writer using OpenAI's GPT-4";
    let points = "
- Needs to be done piece by piece, not in a single go
- Will need some editing to still be your own voice
- Markdown works best
";

    let planning_prompt = prompt_builder::get_planning_prompt(idea, points);
    let sections_str = get_completion(&planning_prompt).await;

    let sections = sections_str
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(|line| line.trim_start_matches("- ").trim())
        .collect::<Vec<&str>>();

    info!("Sections: {:?}", sections);

    let output = tokio_stream::iter(sections)
        .then(|section| async {
            write_section(idea, points, sections_str.to_string(), section).await
        })
        .collect::<Vec<_>>()
        .await
        .join("\n\n");

    info!("Output:\n{}", output);

    info!("Finished.");
}

async fn write_section(idea: &str, points: &str, sections: String, section: &str) -> String {
    let prompt = format!(
        "Given the context of writing a post about: {idea}

Considering that the post needs to make the following points:

{points}

And considering that the full post is broken down into the following sections:

{sections}

Write the content of the section that talks about: {section}, using pure markdown."
    );
    get_completion(&prompt).await
}
