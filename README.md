using cargo run in the build directory should allow for running the commands from the test cases. \

to use DBL as command prexix in other directorys, you can use to add it to your path.\
Make sure that you replace {{PATH_TO_BUILD_DIRECTORY}} with the location that you cloned the reposity, including the enclosing folder. \
cargo build\
export "export PATH="${PATH}:{{PATH_TO_BUILD_DIRECOTY}}/target/debug/""\



Starting a README so that we can keep track of all commits and changes.

11/3/24 6:33\
PM I added a Cargo.toml file with the project name: Final_project_test that will be appended later.\
I also added the main.rs file, which handles the input handler and output handling modules. In this file, I got the minimal prototype to work for initializing an empty repository.\
Both the input and output handlers are done within the main file, which is also in charge of building the project. \
I also added the file_management.rs file that I made to test the functionality of the input and output handling modules.\
The file_management module will be updated later, but I added it just to show my work for testing purposes. \
I tested the input and output handling modules, and it works for initializing an empty repository, as well as if the user enters a command that is not -- init.\
To run the main file, just use the command: cargo run -- init. This will create an empty repository and output to the user that the repository was successfully generated.\
If the user were to enter a command that is not cargo run -- init, for example if the user's input was: cargo run -- blank, the output of the file would be: Unknown command: 'blank'.\ 
The rest of the prototype will be implemented later, but this is a good start. - George \
\
11/4/24 12:25 PM
Moved source code to src folder. - Ethan

11/11/24 10:30 AM\
Moved A.1 Submodules into a directory.\
Added algorithm hiding modules file and implemented Unique ID - Implementation may be updated further.\
Began implementing file version storage.  
