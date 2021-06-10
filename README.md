###CS410P Final Project

#####Names: Jorden Hanford

#####Project Name: A Rusty Blog

#####Description:
What this project is supposed to be is a blog app where a user can create a blog post
that contains a topic, the author, the content of the post, and a post id. There is functionality
for the user to either add or edit a given post by its post id, and display all of the posts that
have been made. This project utilizes the text-io crate to get the users input and store it into
the appropriate fields.

How to build the project is very simple. All you have to do is run "cargo build" which will build
the project then just run "cargo run" its that simple.

Aside from user testing, there isnt much testing due to time constraints from failed attempts
trying to do this on a web framework.

When you first run the program there will be a small explanation on that it does and how it will
work like previously stated. Then you will be presented with options on what you would like to do
in the form of a numerical choice: "Choices: Add a post (1), Edit a post (2), Show all posts (3),
Exit (4)" entering in the corresponding number will result in that task being fulfilled. After
choosing and completing an option, you will have the chance to pick another option until you are
done and choose to exit.

The overwhelmingly large problem that I faced was trying to build the blog app on a web framework
because it was so much more complicated than I had anticipated, and the feature of hosting your 
web app locally on your computer didnt work for me so I couldnt even see if the code I wrote
worked or not so I hadto switch off of that idea which took up a large amount of time. Also tryig
to understand all of the different pieces needed to have the Rust code work with javascript and 
all the code to run it in a webpage was insanely confusing to me. However, I think I accomplished
my goal in what little time I had left and got a working bare bones idea of what I would have
wanted to work but in a webapp. Although, I deffenitly want to understand how to make an
interactive web app from scratch though, that would be really cool.

The license this program uses is the recommended MIT license which can be found in the
[LICENSE](LICENSE.txt) file.
