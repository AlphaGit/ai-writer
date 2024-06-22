❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/ai-writer`
2024-06-15T15:38:14.729328-04:00 INFO ai_writer - Starting...
2024-06-15T15:43:11.761342-04:00 DEBUG ai_writer::config - Loading dotenv file
2024-06-15T15:43:11.762889-04:00 DEBUG ai_writer::config - Config name: "OPENAI_API_KEY"
2024-06-15T15:43:11.763057-04:00 DEBUG ai_writer::openai_client - Prompt:
You need to plan what sections an article will have. The idea behind the article is:

<IDEAS>
Lessons about writing a rust application that progressively generates automated blog posts using OpenAI ChatGPT API
</IDEAS>

The article needs to touch the following points. These are not titles, and these can be arranged in any order. They do not all need to be present as sections.

<POINTS>
- The code needs to plan the sections first
- After that it needs to create a full context as part of a prompt
- It needs to send everything together to the get contents
- If the text was going to be long, it would have to be broken in pieces and edited for them to connect, but blogposts are usually short
- Learning rust for this was really fun!
- I have a copy available publicly at https://github.com/AlphaGit/ai-writer
- This very same blogpost was written with this tool!
</POINTS>

Here are some further indications that you should consider when planning the article:

<INDICATIONS>
- write sections that are around 2 to 3 paragraphs each
- do not explain the basics of rust
- do not explain the basics of APIs
- do not give code examples
- mention that human revision is always important
</INDICATIONS>

The output will be a non-numbered list with the section titles. No explanations. Include subitems with two spaces of indentation that explains contents that should be included in the section. Some sections might require a lot of explanations, so they can have more points. Some sections might require little to no detail at all, so they will have less points.
2024-06-15T15:43:11.771686-04:00 DEBUG reqwest::connect - starting new connection: https://api.openai.com/
2024-06-15T15:43:15.857505-04:00 DEBUG ai_writer::openai_client - Completion Response:
- **Introduction**
  - Brief overview of the purpose of the article
  - Mention that the blog post itself was generated using the tool

- **Planning the Blog Post Sections**
  - Importance of structure in a blog post
  - How to plan sections using Rust code

- **Creating Context for the Prompt**
  - Role of context in making API requests
  - How to build a full context for the prompt in Rust

- **Integrating with OpenAI ChatGPT API**
  - Steps to send the prompt together to generate content
  - Mention typical length of a blog post and handling lengthy text

- **Connecting Broken Pieces**
  - Discuss why blog posts are typically shorter
  - Strategies for breaking and connecting text if it gets too long

- **The Joy of Learning Rust**
  - Personal experiences learning Rust
  - Highlights of using Rust in the project

- **Human Revision Is Essential**
  - Importance of reviewing AI-generated text
  - Examples of common issues that need human editing

- **Public Repository**
  - Link to the public repository on GitHub
  - Brief explanation of what can be found in the repository

- **Conclusion**
  - Wrap up the learnings shared in the article
  - Encouragement for others to try building their own Rust applications
2024-06-15T15:43:15.857917-04:00 INFO ai_writer - Sections: ["- **Introduction**\n  - Brief overview of the purpose of the article\n  - Mention that the blog post itself was generated using the tool\n", "**Planning the Blog Post Sections**\n  - Importance of structure in a blog post\n  - How to plan sections using Rust code\n", "**Creating Context for the Prompt**\n  - Role of context in making API requests\n  - How to build a full context for the prompt in Rust\n", "**Integrating with OpenAI ChatGPT API**\n  - Steps to send the prompt together to generate content\n  - Mention typical length of a blog post and handling lengthy text\n", "**Connecting Broken Pieces**\n  - Discuss why blog posts are typically shorter\n  - Strategies for breaking and connecting text if it gets too long\n", "**The Joy of Learning Rust**\n  - Personal experiences learning Rust\n  - Highlights of using Rust in the project\n", "**Human Revision Is Essential**\n  - Importance of reviewing AI-generated text\n  - Examples of common issues that need human editing\n", "**Public Repository**\n  - Link to the public repository on GitHub\n  - Brief explanation of what can be found in the repository\n", "**Conclusion**\n  - Wrap up the learnings shared in the article\n  - Encouragement for others to try building their own Rust applications"]
2024-06-15T15:43:15.858034-04:00 INFO ai_writer - Writing all sections...
2024-06-15T15:43:15.858088-04:00 DEBUG ai_writer::config - Loading dotenv file
2024-06-15T15:43:15.858663-04:00 DEBUG ai_writer::config - Config name: "OPENAI_API_KEY"
2024-06-15T15:43:15.858710-04:00 DEBUG ai_writer::openai_client - Prompt:
Given the context of writing a post about: Lessons about writing a rust application that progressively generates automated blog posts using OpenAI ChatGPT API

The audience of the post is: seasoned developers, technical people, non-begginers

The voice and tone of the post should be: friendly, funny, not cheeky, not too annoying with jokes, informal but not obscene

Considering that the post needs to make the following points:

<POINTS>
- The code needs to plan the sections first
- After that it needs to create a full context as part of a prompt
- It needs to send everything together to the get contents
- If the text was going to be long, it would have to be broken in pieces and edited for them to connect, but blogposts are usually short
- Learning rust for this was really fun!
- I have a copy available publicly at https://github.com/AlphaGit/ai-writer
- This very same blogpost was written with this tool!
</POINTS>

And considering that the full post is broken down into the following sections:

<SECTIONS>
- **Introduction**
  - Brief overview of the purpose of the article
  - Mention that the blog post itself was generated using the tool

- **Planning the Blog Post Sections**
  - Importance of structure in a blog post
  - How to plan sections using Rust code

- **Creating Context for the Prompt**
  - Role of context in making API requests
  - How to build a full context for the prompt in Rust

- **Integrating with OpenAI ChatGPT API**
  - Steps to send the prompt together to generate content
  - Mention typical length of a blog post and handling lengthy text

- **Connecting Broken Pieces**
  - Discuss why blog posts are typically shorter
  - Strategies for breaking and connecting text if it gets too long

- **The Joy of Learning Rust**
  - Personal experiences learning Rust
  - Highlights of using Rust in the project

- **Human Revision Is Essential**
  - Importance of reviewing AI-generated text
  - Examples of common issues that need human editing

- **Public Repository**
  - Link to the public repository on GitHub
  - Brief explanation of what can be found in the repository

- **Conclusion**
  - Wrap up the learnings shared in the article
  - Encouragement for others to try building their own Rust applications
</SECTIONS>

Write the content of the blogpost that talks about those sections, following the order, and creating a new title for each section.

Consider the following indications when writing the content:

<INDICATIONS>
- write sections that are around 2 to 3 paragraphs each
- do not explain the basics of rust
- do not explain the basics of APIs
- do not give code examples
- mention that human revision is always important
</INDICATIONS>
2024-06-15T15:43:15.859981-04:00 DEBUG reqwest::connect - starting new connection: https://api.openai.com/
2024-06-15T15:43:27.656369-04:00 DEBUG ai_writer::openai_client - Completion Response:
# How I Built a Rust Tool that Writes Blog Posts Using OpenAI's ChatGPT API

Ever sat staring at a blank screen, struggling to start writing a blog post? I have a solution for you! Imagine a tool that plans, writes, and almost posts your blog articles while you sip your coffee. Yes, that's what I did using Rust and the OpenAI ChatGPT API. And, guess what? This very article you're reading was written using that same tool! Let's dive into the nuts and bolts of creating a Rust application that automates blog writing.

## Planning the Blog Post Sections

Before jumping into code, planning is key. Just like a well-cooked meal needs a recipe, a good blog post needs a structured outline. Simply put, think of your blog post as a set of sections that need to be covered. In Rust, it's efficient to break down this planning phase into our code to keep things organized. From the introduction to the conclusion, each segment is mapped out in advance, ensuring that the final piece flows logically.

Structure is everything in writing. By planning out sections programmatically, you can ensure the generated content covers all necessary points. And trust me, automating this process is a lot less stressful than staring at a blank document and hoping words will magically appear!

## Creating Context for the Prompt

Once we have our sections planned out, the next step is creating robust context for our API prompts. This step is crucial because the context you provide determines the quality and relevance of the output you get from the API. In Rust, you can build this context string step-by-step, ensuring it encapsulates the entire planned content.

When you think about it, context is like the fuel that drives the engine of API requests. By feeding the AI detailed and structured context, you’re setting the stage for eloquent and targeted output. It's like giving a chef all the necessary ingredients and a recipe instead of just telling them to "make something tasty."

## Integrating with OpenAI ChatGPT API

Now comes the exciting part: talking to the OpenAI ChatGPT API. Once you've structured and contextualized your blog sections, it's time to send everything together to the API. The trick here is making sure your prompt is cohesive, maintaining the narrative you've planned out. Rust’s powerful syntax makes it straightforward to construct these API calls.

Blog posts usually fall on the shorter side, which is fantastic because it means fewer chunks to stitch together later. However, for longer pieces, careful handling of segmented text becomes crucial. But I'll touch more on that in the next section.

## Connecting Broken Pieces

In an ideal world, all blog posts would be short and sweet, but what if you need to generate a longer article? Splitting content into smaller chunks and still making them connect seamlessly is an art in itself. Luckily, Rust’s robust string and data handling capabilities make this less gruesome.

Even though blog posts are typically more bite-sized, you still need strategies for connecting split text. Simple tricks like ensuring transitional phrases or holding the narrative thread can make all the difference. If text generations end up feeling choppy, human intervention can smooth things out, reminding us that AI isn't here to replace us but to assist us!

## The Joy of Learning Rust

Ah, the joys of Rust! Learning Rust for this project was an absolute blast. It's a language that's as powerful as it is rigorous, combining the efficiency of system-level programming with a modern syntax. This journey was a refreshing dive into ownership, concurrency, and safety. Plus, Rust’s tooling and compiler-assisted troubleshooting saved me tons of headaches.

Building this tool in Rust wasn't just about achieving a goal but savoring the process. Rust's focus on safety and performance perfectly complemented the demands of integrating with an external API for creating content. I could geek out about Rust all day, but let's move on!

## Human Revision Is Essential

Here's the thing: no matter how sophisticated your AI tool is, human revision remains indispensable. AI is astoundingly powerful, but it sometimes makes mistakes, misses nuances, or simply doesn’t capture the human touch. So, once the initial draft is generated, go through it and refine it. Your readers will thank you.

For instance, while my tool generated this very post, I made sure to give it a once-over to ensure everything was coherent and engaging. And believe me, it wasn’t just for peace of mind; it was a crucial step in delivering quality content.

## Public Repository

Interested in digging into the code and maybe even creating your own AI blog writer? I've made the code available publicly on GitHub at [AlphaGit/ai-writer](https://github.com/AlphaGit/ai-writer). In this repository, you'll find everything from the initial planning scripts to API integration and context generation. It's a playground for those eager to see Rust and AI in action.

Feel free to fork it, star it, or even contribute if you feel like making it better. Open source is all about collaboration and continuous improvement, after all.

## Conclusion

Automating blog post creation with Rust and OpenAI ChatGPT API was an exciting and rewarding experience. From planning sections to creating robust context and handling API responses, this project highlighted the power of combining structured programming with advanced AI. Whether you're a seasoned Rustacean or just looking to explore new horizons, I encourage you to dive in and try building your own tools. Happy coding!

Remember, the future of writing is here, and it starts with a single line of Rust code.
2024-06-15T15:43:27.657262-04:00 DEBUG ai_writer - Output:
# How I Built a Rust Tool that Writes Blog Posts Using OpenAI's ChatGPT API

Ever sat staring at a blank screen, struggling to start writing a blog post? I have a solution for you! Imagine a tool that plans, writes, and almost posts your blog articles while you sip your coffee. Yes, that's what I did using Rust and the OpenAI ChatGPT API. And, guess what? This very article you're reading was written using that same tool! Let's dive into the nuts and bolts of creating a Rust application that automates blog writing.

## Planning the Blog Post Sections

Before jumping into code, planning is key. Just like a well-cooked meal needs a recipe, a good blog post needs a structured outline. Simply put, think of your blog post as a set of sections that need to be covered. In Rust, it's efficient to break down this planning phase into our code to keep things organized. From the introduction to the conclusion, each segment is mapped out in advance, ensuring that the final piece flows logically.

Structure is everything in writing. By planning out sections programmatically, you can ensure the generated content covers all necessary points. And trust me, automating this process is a lot less stressful than staring at a blank document and hoping words will magically appear!

## Creating Context for the Prompt

Once we have our sections planned out, the next step is creating robust context for our API prompts. This step is crucial because the context you provide determines the quality and relevance of the output you get from the API. In Rust, you can build this context string step-by-step, ensuring it encapsulates the entire planned content.

When you think about it, context is like the fuel that drives the engine of API requests. By feeding the AI detailed and structured context, you’re setting the stage for eloquent and targeted output. It's like giving a chef all the necessary ingredients and a recipe instead of just telling them to "make something tasty."

## Integrating with OpenAI ChatGPT API

Now comes the exciting part: talking to the OpenAI ChatGPT API. Once you've structured and contextualized your blog sections, it's time to send everything together to the API. The trick here is making sure your prompt is cohesive, maintaining the narrative you've planned out. Rust’s powerful syntax makes it straightforward to construct these API calls.

Blog posts usually fall on the shorter side, which is fantastic because it means fewer chunks to stitch together later. However, for longer pieces, careful handling of segmented text becomes crucial. But I'll touch more on that in the next section.

## Connecting Broken Pieces

In an ideal world, all blog posts would be short and sweet, but what if you need to generate a longer article? Splitting content into smaller chunks and still making them connect seamlessly is an art in itself. Luckily, Rust’s robust string and data handling capabilities make this less gruesome.

Even though blog posts are typically more bite-sized, you still need strategies for connecting split text. Simple tricks like ensuring transitional phrases or holding the narrative thread can make all the difference. If text generations end up feeling choppy, human intervention can smooth things out, reminding us that AI isn't here to replace us but to assist us!

## The Joy of Learning Rust

Ah, the joys of Rust! Learning Rust for this project was an absolute blast. It's a language that's as powerful as it is rigorous, combining the efficiency of system-level programming with a modern syntax. This journey was a refreshing dive into ownership, concurrency, and safety. Plus, Rust’s tooling and compiler-assisted troubleshooting saved me tons of headaches.

Building this tool in Rust wasn't just about achieving a goal but savoring the process. Rust's focus on safety and performance perfectly complemented the demands of integrating with an external API for creating content. I could geek out about Rust all day, but let's move on!

## Human Revision Is Essential

Here's the thing: no matter how sophisticated your AI tool is, human revision remains indispensable. AI is astoundingly powerful, but it sometimes makes mistakes, misses nuances, or simply doesn’t capture the human touch. So, once the initial draft is generated, go through it and refine it. Your readers will thank you.

For instance, while my tool generated this very post, I made sure to give it a once-over to ensure everything was coherent and engaging. And believe me, it wasn’t just for peace of mind; it was a crucial step in delivering quality content.

## Public Repository

Interested in digging into the code and maybe even creating your own AI blog writer? I've made the code available publicly on GitHub at [AlphaGit/ai-writer](https://github.com/AlphaGit/ai-writer). In this repository, you'll find everything from the initial planning scripts to API integration and context generation. It's a playground for those eager to see Rust and AI in action.

Feel free to fork it, star it, or even contribute if you feel like making it better. Open source is all about collaboration and continuous improvement, after all.

## Conclusion

Automating blog post creation with Rust and OpenAI ChatGPT API was an exciting and rewarding experience. From planning sections to creating robust context and handling API responses, this project highlighted the power of combining structured programming with advanced AI. Whether you're a seasoned Rustacean or just looking to explore new horizons, I encourage you to dive in and try building your own tools. Happy coding!

Remember, the future of writing is here, and it starts with a single line of Rust code.
2024-06-15T15:43:27.658914-04:00 INFO ai_writer::output_writer - Writing output to file: ./output/output-2024-06-15-15-43-27.md
2024-06-15T15:43:27.660945-04:00 INFO ai_writer - Finished.