# Chalgen: A simple command line utility that generates challenges 
This utility will look at a set of text files that you give it and create a set of challenges. For example, you could have some text files that describe pixel art challenges you can try.

`theme.txt`
```
Draw a house
Draw a mountain
Draw a bear
```

`size.txt`
```
at 128x64 pixels
at 256x128 pixels
at 64x64 pixels
```

`palette.txt` 
```
with Midnight Ablaze Palette
with Oil 6 Palette
with Apollo Palette
with Spanish sunset palette
```

With these files in the root directory, you can run something like this:
```
cargo run -- -f theme.txt size.txt palette.txt -c 5
```
This will...
1. Look through the listed files indicated by `-f` or `--files`
2. Print out the challenges details in the order they were listed, randomly picking an entry in each file. This will be done five times, as indicated by the `-c 5` argument. `--count` can also be used instead. 

One example output could be the following:
```
Draw a house at 64x64 pixels with Midnight Ablaze Palette 
Draw a mountain at 256x128 pixels with Oil 6 Palette
Draw a house at 128x64 pixels with Apollo Palette
Draw a bear at 128x64 pixels with Midnight Ablaze Palette
Draw a mountain at 256x128 pixels with Midnight Ablaze Palette
```
### Full Argument List
`-f or --files (Required)`: This will determine what files the program looks through to generate challenges.

`-c or --count (Optional, default 1)`: This will determine how many random instances of challenges are generated

`-s or --separator (Optional, default " ")`: This is a string that will be placed between the separate parts of a single challenge.