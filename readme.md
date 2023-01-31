# Function Builder
parser based on function builder pattern

# How to ❔
```rust
// define a parser
// fill args with pattern
// or use combinator to match multiple pattern
let word_number = "more12and4 and then2";
let iter = word_number.chars();

let match_word_or_number = one_or_more( or(word, number) );

assert_eq!( match_word_or_number(iter), Some("more12and4".to_string()) );
```

# Builder 🛠
## Pattern
- word
- number
- space
- operator
- any_line
- literal()
- any_but()
- any_but_pattern()

## Combinator
- or

## Parser
- one
- one_or_more



# Todo 📝
- add one more layer for storing the iterator