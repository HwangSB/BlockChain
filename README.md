# Project Structure
## Block
Stores Proof Of Work data.

## Permanence
Traits that manipulate the database.  
Depending on how you implement the trait, you can also store blocks in memory.

## BlockChain
Blockchain can only add blocks.  
Stores the connections of blocks according to the database implemented in Permanence.  
Implemented Iterater to make it easy to load a list of blocks.

# Example
``` bash
cargo run addblock "Hello, world!"
# Output:
# Mining the block containing "Hello, world!"
# Success!
```

``` bash
cargo run status
# Output:
# =================================== Block Data ===================================
# index: 1
# time: 2022-10-13T15:29:25.420684300+00:00
# prev: 0000000000000000000000000000000000000000000000000000000000000000
# curr: 000043999252e726664dc742cf82f05e912c1a3ee523bebca1dea8106116f829
# data: Hello, world!
# ==================================================================================
```

# 
This repository is a toy project for personal study.