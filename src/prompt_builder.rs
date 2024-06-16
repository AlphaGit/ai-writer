pub fn get_planning_prompt(idea: String, points: String, indications: String) -> String {
    format!(
        "You need to plan what sections an article will have. The idea behind the article is:

<IDEAS>
{idea}
</IDEAS>

The article needs to touch the following points. These are not titles, and these can be arranged in any order. They do not all need to be present as sections.

<POINTS>
{points}
</POINTS>

Here are some further indications that you should consider when planning the article:

<INDICATIONS>
{indications}
</INDICATIONS>

The output will be a non-numbered list with the section titles. No explanations. Include subitems with two spaces of indentation that explains contents that should be included in the section. Some sections might require a lot of explanations, so they can have more points. Some sections might require little to no detail at all, so they will have less points."
    )
}

pub const SYSTEM_PROMPT: &str = "You are a writer for a developer and writer about open source, architecture and productivity. Your job is to write a blog post about a subject and develop it to completion. All output should be markdown based. Be informal but avoid vulgarity.";

pub fn get_content_prompt(
    idea: String,
    points: String,
    indications: String,
    audience: String,
    voice: String,
    sections: String,
) -> String {
    format!(
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

Write the content of the blogpost that talks about those sections, following the order, and creating a new title for each section.

Consider the following indications when writing the content:

<INDICATIONS>
{indications}
</INDICATIONS>"
)
}
