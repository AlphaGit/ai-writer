use ai_writer::{input_reader, openai_client, output_writer, prompt_builder};
use log::{debug, info};
use log4rs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting...");

    // TODO These should be requested from the user
    // ---------------- BEGIN ---------------------
    let idea = input_reader::get_idea();
    // "How to create an automated blog post writer using OpenAI's GPT-4 and some Rust code.";
    let points = input_reader::get_points();
// "
// - Will need some editing to still be your own voice.
// - Markdown works best for generated text.
// - It's important to have GPT perform step-by-step instructions and not all in a single go. It improves the accuracy.
// - Conclusions and points to make need to be specified by humans.
// - This post is actually an example of that!
// - Do not explain basics or how to setup environments
// - Why Rust? Aside from the benefits everyone knows, it was a nice excuse to learn and use the language.
// - Remember that GPT is outdated. After the text is generated you'll need to edit the text to fix factual inaccuracies.
// - Always re-send the context to GPT on each generation, so it's all coherent.
// - A good approach (like I did here) is to generate a first text, and enter the feedback as points to consider for a second iteration.
// - Example at https://github.com/AlphaGit/ai-writer
// - Things that worked: running the whole article in a single generation, because 128k tokens is a lot.
// - Things that were difficult: working in multiple simultaneous futures in Rust.
// - Things that did not work: generating each section separately, because the context was lost.
// ";
    let audience = input_reader::get_intended_audience();
    // "developers, technical people";
    let voice = input_reader::get_writing_voice();
        // "friendly, informal, funny but not cheesy, to the point, straightfworward, and informative";
    let indications = input_reader::get_writer_indications();
// "
// - DO NOT go over three levels deep in titles (and use level 3 only sparingly).
// - DO NOT cover basics of API setup or what GPT is, or how to use it
// - DO NOT explain what Rust is or how its used. Assume the reader knows.
// - DO NOT exceed 50000 words for the post, or 1000 words in each section. Each section should have several paragraphs.
// - DO talk about the strategy behind the creation of a coherent piece of content
// - DO talk about the importance of having a human in the loop
// - DO explain the reasons behind the specific points that are made
// ";
    // ---------------- END ---------------------

    let planning_prompt = prompt_builder::get_planning_prompt(idea.clone(), points.clone(), indications.clone());
    let sections_str = openai_client::get_completion(&planning_prompt)
        .await
        .unwrap();

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

    output_writer::write_output(&output);

    info!("Finished.");
    Ok(())
}

async fn write_all_sections(
    idea: String,
    points: String,
    indications: String,
    audience: String,
    voice: String,
    sections: String,
) -> Option<String> {
    info!("Writing all sections...");

    let prompt =
        prompt_builder::get_content_prompt(idea, points, indications, audience, voice, sections);

    openai_client::get_completion(&prompt).await
}
