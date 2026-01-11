game:
    cargo run --bin game

ui:
    cargo run --bin ui

test:
    cargo test --message-format short

assets:
    bash scripts/convert_svg_to_png.sh 64 assets/tiles
