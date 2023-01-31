# conch

A Rust proc macro which embeds [Nushell](https://www.nushell.sh/) with compile-time linting.

```rust
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
```

```
error: nu::parser::missing_positional (https://docs.rs/nu-parser/0.74.0/nu_parser/enum.ParseError.html#variant.MissingPositional)
       
         × Missing required positional argument.
         help: Usage: fetch {flags} <URL>
  --> conch/src/lib.rs:12:35
   |
12 |           if let Value(result, _) = sh! {
   |  ___________________________________^
13 | |             let post_id = (fetch  | first);
14 | |             fetch $"https://hacker-news.firebaseio.com/v0/item/($post_id).json" | get score;
15 | |         }? {
   | |_________^
   |
   = note: this error originates in the macro `sh` (in Nightly builds, run with -Z macro-backtrace for more info)

error: could not compile `conch` due to previous error
```

Yes, the name is intentionally meant to be confused with [xonsh](https://github.com/xonsh/xonsh).
