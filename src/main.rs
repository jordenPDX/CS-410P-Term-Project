//!Jorden Hanford
//!CS410P
//!Blog Project
//!Bart Massey
//!This program will run a blog where the user can add and edit posts.

use std::convert::TryInto;
use text_io::read;

struct Post {
    topic: String,
    author: String,
    body: String,
    id: u32,
}

fn makepost(newpost: &mut Post, postcount: u32) {
    //Enter the topic of the new post
    println!("Topic (press ENTER or TAB to complete): ");
    newpost.topic = read!("{}\n");

    //Get the author
    println!("Author (press ENTER or TAB to complete): ");
    newpost.author = read!("{}\n");

    //Enter the body of the post
    println!("Content (press ENTER or TAB to complete): ");
    newpost.body = read!("{}\n");

    //Set the id of the post
    if postcount == 0 {
        newpost.id = 1;
    } else {
        newpost.id = postcount + 1;
    }
}

//Function to edit the contents of a desired post
fn editpost(post: &mut Post) {
    //Edit the topic of the post
    post.topic.clear();
    println!("Topic (press ENTER or TAB to complete): ");
    post.topic = read!("{}\n");

    //Edit the author
    post.author.clear();
    println!("Author (press ENTER or TAB to complete): ");
    post.author = read!("{}\n");

    //Edit the body of the post
    post.body.clear();
    println!("Content (press ENTER or TAB to complete): ");
    post.body = read!("{}\n");
}

//Function to display the contents of a post
fn displaypost(post: &mut Post) {
    //Display the posts topic
    println!("\nTopic: {}\n", post.topic);

    //Display the posts author
    if post.author.is_empty() {
        println!("Author: Anonymous\n");
    } else {
        println!("Author: {}\n", post.author);
    }

    //Display the posts body
    println!("{}\n", post.body);

    //Display the posts ID
    println!("{}\n", post.id);
}

//Main driver of the blog app
fn main() {
    //Create a Vec to store the posts
    let mut posts = vec![];

    println!("Welcome, this program will let you create and edit blog/forum posts.\n");
    println!("enter the number corresponding to the choice you want to add or edit a post\n\n");
    println!("Choices: Add post (1), Edit post (2), Show posts (3), Exit (4) :");

    //Have the user decide what to do
    let mut choice: u32 = read!("{}");

    while choice != 4 {
        //Execute the function the user selected
        match choice {
            1 => {
                let mut newpost = Post {
                    topic: String::new(),
                    author: String::new(),
                    body: String::new(),
                    id: 0,
                };
                makepost(&mut newpost, posts.len().try_into().unwrap());
                posts.push(newpost);
            }
            2 => {
                println!("Enter the post id of the desired post to edit:\n");
                for i in 0..posts.len() {
                    println!("{}", i);
                }
                let edit_id: usize = read!("{}");
                let temp = &mut posts[edit_id];
                editpost(temp);
            }
            3 => {
                for j in 0..posts.len() {
                    let nextpost: usize = j;
                    displaypost(&mut posts[nextpost]);
                }
            }
            _ => (),
        }

        //Ask what the user wants to do next
        println!("\nAdd another (1), Edit a post (2), Show the posts (3), Exit (4): ");
        choice = read!("{}");
    }
}
