Solutions are made with the [advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) template in mind which is why the code will look a bit shorter/weirder. Boilerplate is generated via cargo alias run commands.    
 
Inputs are downloaded using [aoc-cli](https://github.com/scarvalhojr/aoc-cli) (advent-of-code-rust explains how to set it up to be used with it's template). 
  
The functions will look shorter because the IO is handled by the template and then passed as a &str to each function for a specific day.

The Option<u32> return in the case of None is used to signify any errors or to create the initial function stub for the templates.
