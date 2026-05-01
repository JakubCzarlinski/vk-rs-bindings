mkdir -p registry
curl -o registry/video.xml https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/refs/heads/main/xml/video.xml
curl -o registry/vk.xml https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/refs/heads/main/xml/vk.xml

cargo run -p vk-codegen -- \
  --vk ./registry/vk.xml \
  --video ./registry/video.xml \
  --out ./vk-rs-bindings

cargo fmt && cargo clippy --all-features --fix --allow-dirty && cargo fmt
