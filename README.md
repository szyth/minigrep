Build the project with:
`cargo build --release`

Run binary:
```bash
cd target/release

./minigrep <string_to_search> <filepath>

# to ignore case
IGNORE_CASE=1 ./minigrep <string_to_search> <filepath>
```