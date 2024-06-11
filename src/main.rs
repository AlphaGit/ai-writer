use ai_writer::get_completion;
use ai_writer::prompt_builder;
use log::{debug, info};
use log4rs;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting...");

    // TODO These should be requested from the user
    // ---------------- BEGIN ---------------------
    let idea =
        "How to create an automated blog post writer using OpenAI's GPT-4 and some Rust code.";
    let points = "
- Will need some editing to still be your own voice.
- Markdown works best for generated text.
- It's important to have GPT perform step-by-step instructions and not all in a single go. It improves the accuracy.
- Conclusions and points to make need to be specified by humans.
- This post is actually an example of that!
- Do not explain basics or how to setup environments
- Why Rust? Aside from the benefits everyone knows, it was a nice excuse to learn and use the language.
- Remember that GPT is outdated. After the text is generated you'll need to edit the text to fix factual inaccuracies.
- Always re-send the context to GPT on each generation, so it's all coherent.
- A good approach (like I did here) is to generate a first text, and enter the feedback as points to consider for a second iteration.
";
    let audience = "developers, technical people";
    let voice =
        "friendly, informal, funny but not cheesy, to the point, straightfworward, and informative";
    let indications = "
- DO NOT go over three levels deep in titles (and use level 3 only sparingly).
- DO NOT cover basics of API setup or what GPT is, or how to use it
- DO NOT exceed 5000 words for the post, or 400 words in each section
- DO talk about the strategy behind the creation of a coherent piece of content
- DO talk about the importance of having a human in the loop
- DO explain the why of the specific points that are made
";
    // ---------------- END ---------------------

    let planning_prompt = prompt_builder::get_planning_prompt(idea, points, indications);
    let sections_str = get_completion(&planning_prompt).await.unwrap();

    let sections = sections_str.split("\n- ").collect::<Vec<&str>>();

    info!("Sections: {:?}", sections);

    let output = write_all_sections(
        idea,
        points,
        indications,
        audience,
        voice,
        sections_str.to_string(),
    )
    .await
    .unwrap();

    debug!("Output:\n{}", output);

    fs::write("output.md", output).unwrap();

    info!("Finished.");
    Ok(())
}

async fn write_all_sections(
    idea: &str,
    points: &str,
    indications: &str,
    audience: &str,
    voice: &str,
    sections: String,
) -> Option<String> {
    info!("Writing all sections...");

    let prompt = format!(
        "Given the context of writing a post about: {idea}

The audience of the post is: {audience}

The voice and tone of the post should be: {voice}

Considering that the post needs to make the following points:

<POINTS>
{points}
</POINTS>

And considering that the full post is broken down into the following sections:

<SECTIONS>
{sections}
</SECTIONS>

Write the content of the section of the blogpost that talks about those sections, following the order.

Consider the following indications when writing the content:

<INDICATIONS>
{indications}
</INDICATIONS>
");
    get_completion(&prompt).await
}
