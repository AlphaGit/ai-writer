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

The output will be a non-numbered list with the section titles. No explanations. Include subitems with two spaces of indentation that explains contents that should be included in the section. Some sections might require a lot of explanations, so they can have more points. Some sections might require little to no detail at all, so they will have less points."
    )
}

pub const SYSTEM_PROMPT: &str = "You are a writer for a developer that works on software, open source, architecture and productivity. Your job is to write a blog post about a subject and develop it to completion. All output should be markdown based.";
