import 'justfiles/linting.just'


run *args:
    cargo run -- {{args}}

w *args:
    cargo watch -x 'run'