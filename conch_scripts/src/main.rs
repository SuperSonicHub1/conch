use conch::{sh, NuError};
use nu_protocol::{PipelineData, Span};

fn main() -> Result<(), NuError> {
    let data: PipelineData = sh! { 
        let id = (fetch "https://hacker-news.firebaseio.com/v0/topstories.json" | first);
        fetch $"https://hacker-news.firebaseio.com/v0/item/($id).json"
    }?;
    let top_story = data.into_value(Span::unknown());
    println!("Top story: {}", top_story.get_data_by_key("title").unwrap().as_string()?);
    let upvotes = top_story.get_data_by_key("score").unwrap();
    println!("{} upvote(s)", upvotes.as_i64()?);

    Ok(())
}
