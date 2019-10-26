# RpiVideo

###

Rust + Nerves 在视频采集上的应用

Steven Gu
steven.gu@letote.cn

###

自我介绍

1. Rails Coder
2. 关注 Rust 的嵌入式开发

###

Letote 的应用

1. RFID 设备标识衣服或者首饰
2. （未来可能）在仓库流程上的视频应用

###

技术选型

选择 Port 的原因：

分离应用功能和硬件功能。

选择 Rust 的原因：

支持 `armv7-unknown-linux-gnueabihf` Target。

###

Rust 相关库

1. rust-bindgen:

https://github.com/rust-lang/rust-bindgen（C binding）

2. nix:

https://github.com/nix-rust/nix

###

产出 Projects

1. rpi-mmal-rs:

https://github.com/Pragmatic-Elixir-Meetup/rpi-mmal-rs

2. rpi-video-rs:

https://github.com/Pragmatic-Elixir-Meetup/rpi-video-rs

3. rpi_video (Prepare for open-source)

###

rpi-mmal-rs (v0.0.2)

1. Raspberry Pi GPU Libraries: https://github.com/raspberrypi/userland
2. 使用 `rust-bindgen` 封装 Framework `MMAL` (Multi-Media Abstraction Layer)

###

rpi-video-rs (v0.0.2)

1. 需要在 RPI 上安装摄像头
2. 依赖 `rpi-mmal-rs`
3. 实现 h264 的视频录制流程

###

MMAL 接口介绍

1. MMAL_COMPONENT_T

Camera, H264 Encoder

2. MMAL_PORT_T

Camera 的 Capture、Preview 和 Output，Encoder 的 Input 和 Output

3. MMAL_CONNECTION_T

Connects Camera-Output 和 Encoder-Input

###

rpi_video (Prepare for open-source)

1. 使用 Rust Crate `nix`-select 实现 Rust 部分的事件循环（比较 mio 更容易监控硬件 devices）
1. 使用 Elixir GenServer 启动 Port 实现 Elixir 部分的事件循环
3. Rust `std::sync_channel` 用于同步 Rust 不同进程的消息

###

Q & A?
