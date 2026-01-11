game:
    cargo run --bin game

ui:
    cargo run --bin ui

# Available at: https://github.com/TheWozard/fix_png
fix:
    fix_png --glob "assets/**/*.png"

test:
    cargo test --message-format short

tiles:
    bash scripts/convert_svg_to_png.sh 64 assets/tiles
