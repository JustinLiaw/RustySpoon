# RustySpoon

Phase 1

Direction for Compilation:


1. To get files:
    git clone https://github.com/JustinLiaw/RustySpoon.git

2. To execute the code

    Executable: ./proj_phase1

    When testing individual parts by commenting out main.rs:
    cd RustySpoon/proj_phase1
    cargo run

For Execution:

All code will run all parts at the beginning. If you want to isolate a specific 
part comment out the rest of the sections in main. No function definitions need to
be commented out to run. 

Phase 2:

1. git clone https://github.com/JustinLiaw/RustySpoon.git

2. cd proj_phase2

3. cargo run

Description and Utilization of Rust: 
    We made a hangman game. This game pulls a random word from a text file
    and then plays the standard hangman game. The main feature of rust is
    the fact that it is memory safe. You will notice this in the requirement 
    to explicitly name variable mutable. The code took considerably longer 
    becuase we kept having to find which varible were meant to be mutable and
    which ones were not. Furthermore, the file read in is also a build-in memory
    safe function. 