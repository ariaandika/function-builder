# Patterns

parser is a function builder

## same type parser
- fn(Chars) -> Result<(str,Chars),Chars>

all parser have same type, accept iterator then return the result and the iterator

❌ Problem: cannot check one character multiple time because iterator already consumed

## variant type parser ⚓
single char matching parser: fn(Chars) -> Result<char,char>

combinator single char: fn(String) -> Option<String>

❌ Problem: creating iterator for each parser is expensive


---
- add operator pattern
- add any but pattern
- rename type match to pattern
- change the pattern from Result to Option
---
- add alias/trait to parser and pattern