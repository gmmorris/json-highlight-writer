# json-highlight-writer

A writer for [json-rust](https://github.com/maciejhirsz/json-rust) object which supports highlighting slices when printing JSON objects

## Usage

There are three public functions:

### highlight

_highlight_ takes a JSON object and a vector of slices you wish to highlight.

```rust
use colored::*;
use json::*;
use json_highlight_writer::{highlight_with_colors, highlight, highlight_with_colors_and_remainder};

let res = object!{
    "code" => 200,
    "success" => true,
    "payload" => object!{
        "features" => array![
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
};

println!("{:#}", highlight(&res, vec![&res["code"], &res["payload"]["features"]]));
```

This code will print out the entire JSON structure with the code field and array of features highlighted in red.

![Single Color Matches](../master/assets/single.png?raw=true)

### highlight_with_colors

_highlight_with_colors_ takes a JSON object, a vector of slices you wish to highlight and a vector of colors to cycle through when matching slices.

```rust
println!("{:#}", highlight_with_colors(&res, vec![&res["code"], &res["payload"]["features"]], vec![Color::Red, Color::Green]));

println!("{:#}", highlight_with_colors(&res, vec![&res["payload"], &res["payload"]["features"]], vec![Color::Red, Color::Green]));
```

This code will print out the entire JSON structure twice, with the slices highlighted in red, the green.
Note the inner color highlighting which is used to display matching slices in different colors if a matched slice resides inside of another matched slice.

![Multiple Color Matches](../master/assets/multiple.png?raw=true)

If there are more slices than there are specified colors the highlighting will cycle back through the vector.

```rust
println!("{:#}", highlight_with_colors(&res, vec![&res["code"], &res["payload"], &res["payload"]["features"]], vec![Color::Red, Color::Green]));
```

![Overlapping Matches](../master/assets/overlap.png?raw=true)

### highlight_with_colors_and_remainder
_highlight_with_colors_and_remainder_ is similar, except it can also take a color for the _remainder_, that is, the parts of the JSON that don't overlap with the slice.

```rust
println!("{:#}", highlight_with_colors_and_remainder(&res, vec![&res["code"], &res["payload"], &res["payload"]["features"]], Some(vec![Color::Red, Color::Green]), Some(Color::White)));
```


