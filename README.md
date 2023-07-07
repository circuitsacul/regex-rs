# regex-rs
Python bindings for the rust regex crate

## Usage
```py
from regex_rs import Regex

re = Regex(r"\d+")
for number_match in re.find_iter("1 2 34 123"):
    # number_match: regex_rs.Match
    print(number_match)
# 1
# 2
# 34
# 123
```

## Available methods
- `Regex.find`
- `Regex.find_iter`
- `Regex.captures`
- `Regex.captures_iter`
- `Regex.split`
- `Regex.replace`

## Note
These bindings are missing a significant portion of the rust regex crate.
If you see a struct or method that you would like added, feel free to
open an issue for it.
