# Day 2: Password Philosophy

Another simple Rust solution. Input.txt is read in from a file as a vector of string. Each string from the file then gets split into its constiuent pieces -- the low number, the high number, the relevant character, and the full password. Those pieces are then used to do a simple comparison against the contents of the string and the valid ones are counted.