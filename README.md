# Vulkan-RS
-----

A rust-safe interface for handling Vulkan structures and sending commands to / rendering on the GPU.
Uses [Ash](https://crates.io/crates/ash) and [vk_mem](https://crates.io/crates/vk-mem) under the hood to handle the actual interfacing with the [C/C++ Vulkan Headers Library](https://github.com/KhronosGroup/Vulkan-Headers).