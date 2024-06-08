use ai_writer::get_completion;
use ai_writer::prompt_builder;
use log::{debug, error, info, warn};
use log4rs;
use std::fs;
use tokio_stream;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting...");

    // TODO These should be requested from the user
    let idea =
        "How to create an automated blog post writer using OpenAI's GPT-4 and some Rust code.";
    let points = "
- Will need some editing to still be your own voice.
- Markdown works best.
- It's important to have GPT perform step-by-step instructions and not all in a single go. It improves the accuracy.
- Conclusions and points to make need to be specified by humans.
- This post is actually an example of that!
- Do not explain basics or how to setup environments
";
    let audience = "developers, technical people";
    let voice =
        "friendly, informal, funny but not cheeky, to the point, straightfworward, and informative";

    let planning_prompt = prompt_builder::get_planning_prompt(idea, points);
    let sections_str = get_completion(&planning_prompt).await;

    let sections = sections_str.split("\n- ").collect::<Vec<&str>>();

    info!("Sections: {:?}", sections);

    let output = tokio_stream::iter(sections)
        .then(|section| async {
            write_section(
                idea,
                points,
                audience,
                voice,
                sections_str.to_string(),
                section,
            )
            .await
        })
        .collect::<Vec<_>>()
        .await
        .join("\n\n");

    debug!("Output:\n{}", output);

    fs::write("output.md", output).unwrap();

    info!("Finished.");
}

async fn write_section(
    idea: &str,
    points: &str,
    audience: &str,
    voice: &str,
    sections: String,
    section: &str,
) -> String {
    info!("Writing section: {}", section.lines().next().unwrap());

    let prompt = format!(
        "Given the context of writing a post about: {idea}

The audience of the post is: {audience}

The voice and tone of the post should be: {voice}

Considering that the post needs to make the following points:

{points}

And considering that the full post is broken down into the following sections:

{sections}

Write the content of the section that talks about the following section, including the following points for its contents:

{section}

Do not go over three levels deep in titles. Do not include formatting indicators, just the content in markdown format.
");
    get_completion(&prompt).await
}
