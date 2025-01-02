publish-lib:
    cargo publish --package crml-core --allow-dirty

publish-derive:
    cargo publish --package crml-derive --allow-dirty

publish-main:
    cargo publish --package crml --allow-dirty

publish:
    just publish-lib
    just publish-derive
    just publish-main

test:
    cargo run --example simple
