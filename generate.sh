cargo run -p vk-codegen -- \
  --vk ./vk-codegen/tests/fixtures/vk.xml \
  --video ./vk-codegen/tests/fixtures/video.xml \
  --out ./vk-rs-bindings

cargo fmt && cargo clippy --all-features --fix --allow-dirty && cargo fmt
