pub fn get_planning_prompt(idea: &str, points: &str) -> String {
    format!(
        "You need to plan what sections an article will have. The idea behind the article is:

<IDEAS>
{idea}
</IDEAS>

The article needs to touch the following points. These are not necesarilly titles, and these can be arranged in any order.

<POINTS>
{points}
</POINTS>

Output only a non-numbered list with the section titles. No explanations."
    )
}

pub const SYSTEM_PROMPT: &str = "You are a writer for a developer that works on software, open source, architecture and productivity. Your job is to write a blog post about a subject and develop it to completion. All output should be markdown based.";
