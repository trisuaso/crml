publish-lib:
    cargo publish --package crml-core

publish-bin:
    cargo publish --package crml

publish:
    just publish-lib
    just publish-bin
