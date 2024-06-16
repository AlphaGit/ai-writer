use edit::edit;

pub fn get_idea() -> String {
    let prompt = "<Replace this text with your idea. It should be a single sentence.>";
    edit(prompt).expect("Failed to get idea from editor")
}

pub fn get_points() -> String {
    let prompt = "<Replace this text with the main points you want to cover. Write one per line, as a list, like this:

- The importance of being Ernest
- The black plague was a disasterous event for humanity as a whole.

etc.>";
    edit(prompt).expect("Failed to get points from editor")
}

pub fn get_intended_audience() -> String {
    let prompt = "<Replace this text with the intended audience for your article.>";
    edit(prompt).expect("Failed to get intended audience from editor")
}

pub fn get_writing_voice() -> String {
    let prompt = "<Replace this text with the writing voice you want to use.>";
    edit(prompt).expect("Failed to get writing voice from editor")
}

pub fn get_writer_indications() -> String {
    let prompt = "<Replace this text with any indications you want to give the writer, such as paragraph length, approaches to explaining things, etc.>";
    edit(prompt).expect("Failed to get writer indications from editor")
}