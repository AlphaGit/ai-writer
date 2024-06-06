use ai_writer::get_completion;
use log::{debug, error, info, warn};
use log4rs;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting...");

    let idea = "How to create an automated blog post writer using OpenAI's GPT-4";
    let points = "
- Needs to be done piece by piece, not in a single go
- Will need some editing to still be your own voice
- Markdown works best
";
    let prompt = get_planning_prompt(idea, points);

    let response = get_completion(&prompt).await;
    info!("Response: \n{}", response);
}

fn get_planning_prompt(idea: &str, points: &str) -> String {
    format!(
        "You need to plan what sections an article will have. The idea behind the article is:

<IDEAS>
{idea}
</IDEAS>

The article needs to touch the following points. These are not titles, and these can be arranged in any order.

<POINTS>
{points}
</POINTS>

Output only a list with the section titles. No explanations."
    )
}
