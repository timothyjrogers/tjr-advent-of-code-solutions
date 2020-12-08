# [Day 7: Handy Haversacks](https://adventofcode.com/2020/day/7)

This one was fun, and not just because of the D&D reference. It required some creative string parsing to get the input into a useful data structure and I was never great at recursion in school so I'm pretty pleased with how this turned out.

Both parts follow the standard pattern of reading the input file into a vector of strings. Both parts utilize std::collections::HashMap to create a lookup table.

For part 1 each line is parsed into a key and value:

* Key = everything before "bags contain" trimmed for whitespace
* Value = everything after "bags contain", split on commas to be vectorized, and trimmed down to just the color by string searching the known whitespace and "bag(s)" prefix on each item

Ultimately each HashMap entry looks like this:

```
//shiny gold bags contain 5 light black bags, 3 mirrored yellow bags, 5 muted plum bags.
"shiny gold": vec!["light black", "mirrored yellow", "muted plum"]
```
The solution is then calculated by calling the recursive ```contains_bag``` function on each key in the HashMap which operates as follows:

* Attempt to fetch the passed in bag name (key) from the HashMap
* If the bag is not found then it contained no additional bags (e.g. "pale coral bags contain no other bags."), return false
* If the bag is found check if it's the bag being searched for:
  * If yes, return true
  * If no, recurse and call ```contains bag``` on the current bag

Once the requried bag, in my case "shiny gold", is found the function collapses to a true value. Otherwise all bags in the set are searched and a false bubbles up.

Part 2 has a similar setup however instead of just vectorizing the parsed rules as bag names they are parsed as a tuple of bag quantity and name. Then the recurisve function ```num_bags``` is called which crawls the set of rules like before but instead of searching it adds up the bag quantities at each level multiplied by the number of that bag.