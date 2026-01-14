game:
    cargo run --bin game

merchant:
    cargo run --bin merchant

test:
    cargo test --message-format short

assets:
    bash scripts/convert_svg_to_png.sh 16 assets/tiles
