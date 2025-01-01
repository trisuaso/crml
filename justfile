publish-lib:
    cargo publish --package crml-core --allow-dirty

publish-bin:
    cargo publish --package crml --allow-dirty

publish:
    just publish-lib
    just publish-bin
