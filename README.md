# 🦀 Crab Markup Language

**CRML** is a simple markup language which compiles all given files into a Rust module which fits right into your crate source.

The generated `crml/mod.rs` file exports functions which build the contents of your given templates as HTML.

## Configuration

Your project must contain a `crml.json` file in order to tell the CLI how to build your templates. The repository contains an example [`crml.json`](https://github.com/trisuaso/crml/blob/master/crml.json) file which links to [`examples/simple`](https://github.com/trisuaso/crml/blob/master/examples/simple) to build templates.

You can run this example for yourself with the following commands:

```bash
just test
```

The only key you need to provide in this file is the directory where your templates are stored.

```json
{
    "root_dir": "./templates"
}
```

## Usage

A very simple usage example is shown in the [example](https://github.com/trisuaso/crml/blob/master/examples/simple).

## Attribution

CRML is licensed under the MIT license. You can view the license [here](https://github.com/trisuaso/crml/blob/master/LICENSE).
