// 1. Define the trait 'Summary'
// It requires any implementing type to have a method called
//  `summarize`
// that takes an immutable reference to self (`&self`) and 
// returns a String.
trait Summary {
    fn summarize(&self) -> String;
}

// 2. Define a struct 'NewsArticle'
struct NewsArticle {
    headline: String,
    author: String,
    content: String, // Simplified content
}

// 3. Implement the 'Summary' trait FOR 'NewsArticle'
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // Custom summary logic for NewsArticle that also uses 
        // the content field
        format!("Article: '{}' by {} - Preview: {:.20}...", 
                self.headline, 
                self.author, 
                self.content)
    }
}

// 4. Define another struct 'Tweet'
struct Tweet {
    username: String,
    content: String,
    retweets: u32,
}

// 5. Implement the 'Summary' trait FOR 'Tweet'
impl Summary for Tweet {
    fn summarize(&self) -> String {
        // Custom summary logic for Tweet
        format!("Tweet by @{}: {} ({} retweets)", self.username, self.content, self.retweets)
    }
}
// 6. A function that accepts any type that implements the 
// 'Summary' trait
// The `<T: Summary>` part is a "trait bound", meaning T can 
// be any type
// as long as it implements 'Summary'.
fn print_summary<T: Summary>(item: &T) {
    println!("Summary: {}", item.summarize());
}

// 7. Main function to run the code
fn main() {
    let article = NewsArticle {
        headline: String::from("Rust Traits Explained"),
        author: String::from("AI Assistant"),
        content: String::from("Traits are Rust's way of defining shared behavior..."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Loving Rust traits! #rustlang"),
        retweets: 15,
    };

    // We can call the `summarize` method directly on each instance
    println!("Direct call article: {}", article.summarize());
    println!("Direct call tweet: {}", tweet.summarize());

    println!("---");

    // Or we can use our generic function `print_summary`
    // It works because both NewsArticle and Tweet implement the Summary trait.
    print_summary(&article);
    print_summary(&tweet);
}
// TODO:
