# Snippset lightweight snippets creating terminal util

This tool was inspired by a friend of mine after sharing his tools about creating snippets which is specific for macOS. So I decided to make my own that works on windows / Linux.

Takes a json file and load it for snippets. The file needs to be valid json, the key will be used for the title, and the body will be used for the snippet. Use '{}' to mark an area as editable this will be the placeholder for replacement.

#### Example json format:
```json
{
    "The snippet title": "The snippet body {}",
    "Another Title": "Snippet mark new lines \nplaceholder {} can have multiple {} placeholders"
}
```

Enter will move your cursor to the next placeholder if you have multiples, while Esc will move to previous ones.

Usage: snippset [OPTIONS] <PATH>

Arguments:
  <PATH>
          Path of snippet file to load, file must be valid json with string keys and string values"

Options:
  -a, --add
          Add snippet to the selected json file using interactive mode. If the selected json file does not exist, a new one in the path will be created

  -e, --edit
          Edit snippet in the selected json file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

## Examples 

### Loading
Loading using the json file:
```json
{
  "Adding new json title snippet": "This is a snippet {}",
  "Another snippet": "Yet another {} with multiple {} placeholders\nand new lines"
}
```

![Using program's snippets](./examples/loading-snippet.gif)


### Adding
With -a, -add flag you are able to add new snippets to already existing file. Or a new file will be created if such file does not exist.

![Adding snippet flag will create new file and add snippet to it](./examples/adding-snippet.gif)

### Editing
With -e, -edit flag you are able to edit already existing snippets inside a file.

![Editing pre-existing snippet](./examples/editing-snippet.gif)
