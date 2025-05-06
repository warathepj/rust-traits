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

// Add these imports for the web server
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tiny_http::{Server, Response, StatusCode};

// 7. Main function to run the code and serve the HTML
fn main() {
    // Create our example objects
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

    // Print the trait examples to the console
    println!("Direct call article: {}", article.summarize());
    println!("Direct call tweet: {}", tweet.summarize());
    println!("---");

    // Or we can use our generic function `print_summary`
    // It works because both NewsArticle and Tweet implement the Summary trait.
    print_summary(&article);
    print_summary(&tweet);
    
    // Start the web server
    println!("\nStarting web server...");
    let server = Server::http("127.0.0.1:8000").unwrap();
    println!("Server running at http://127.0.0.1:8000/");

    for request in server.incoming_requests() {
        println!("Received request: {} {}", request.method(), request.url());
        
        // Handle the request
        let url = request.url().to_string();
        
        if url == "/" || url == "/index.html" {
            // Serve the index.html file
            let path = Path::new("static/index.html");
            match File::open(&path) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        let response = Response::from_string(contents)
                            .with_header(tiny_http::Header::from_bytes("Content-Type", "text/html").unwrap());
                        let _ = request.respond(response);
                    } else {
                        let response = Response::from_string("Error reading file")
                            .with_status_code(StatusCode(500));
                        let _ = request.respond(response);
                    }
                }
                Err(_) => {
                    let response = Response::from_string("File not found")
                        .with_status_code(StatusCode(404));
                    let _ = request.respond(response);
                }
            }
        } else {
            // Handle 404 for any other URL
            let response = Response::from_string("Not Found")
                .with_status_code(StatusCode(404));
            let _ = request.respond(response);
        }
    }
}
