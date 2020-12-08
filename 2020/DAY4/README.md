# [Day 4: Passport Processing](https://adventofcode.com/2020/day/4)

The code is a little messy on this one because I got busy and rushed through it a day late, will circle back to clean up later.

The setup to both parts is the same. Read all the lines from the input data into a file, then iterate the data tracking a 'start' pointer. When an empty line is encountered everything between the start pointer (inclusive) and the empy line (exclusive) is concatenated into a single string which represents one passport. Then the start pointer is updated to the line after the empty one. A special check is performed to catch the final passport which does not have an empty line following it.

For part one every passport is checked and split on spaces to get each individual key:value pair. To validate the passport an array containing the required key strings is iterated and every key:value pair from the passport is checked (split on the ':' character) to determine if any of the required keys are missing. If none are missing the validation returns a true result, otherwise a false result.

For part two the same iteration occurs for inclusion of required keys however it is expanded such that every time a required key is found to be present it's value is also validated. This validation leans heavily on the [regex crate](https://docs.rs/regex/1.4.2/regex/index.html) which allows for compiling regular expressions and using them to match and search strings. A pattern match is performed against the possible passport keys and each is validated via the rules in the puzzle -- most of them can simply be validated via a matching regex (e.g. the hex color field) but the fields with numerical constraints require parsing the values to integers to perform those checks.