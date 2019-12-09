# rpi_video

This project implements `H264` video record on Raspberry Pi. It wraps a Rust
H264 video record library `rpi-video-rs` which has already implemented the low
level features. By taking advantage of
[Erlang Port](http://erlang.org/doc/tutorial/c_port.html), the simple
[Elixir GenServer](https://hexdocs.pm/elixir/GenServer.html) APIs are provided.

Since it is an experimental project, not the full video record features of
Raspberry Pi are supposed to be implemented. If you have any ideas or need some
helps, feel free to send an email to asongala@163.com.

## Usage

Add the following to your `mix.exs`:

```exs
defp deps do
  [
    {:rpi_video, "0.0.1"}
  ]
end
```

Get the dependencies and compile (if something went wrong, please reference the
below `Development` part for details):

```
mix deps.get
mix compile
mix compile.cargo
```

Run on a `iex` console:

```
iex -S mix
iex(1)> RpiVideo.start(true)    # true for MockServer, and false for RealServer.
iex(2)> RpiVideo.record(true)   # same as above
iex(3)> RpiVideo.stop(true)     # same as above
```

## Development

This project provides two Elixir GenServer implementations `MockServer` and
`RealServer`.

The MockServer is used for debugging, and compiled only for Apple Mac
(`{:unix, :darwin}`). It simulates the asynchronous process of video record, but
no video is captured and saved.

The RealServer could only be compiled and run on a Raspberry Pi. It could
capture and save videos from a camera.

The Rust implementations of MockServer and RealServer are compiled by the below
command which could identify the current target platform (Mac or Raspberry Pi).
Reference the source code in `lib/mix/tasks/compile.cargo.ex` for details.

```
max compile.cargo
```

Since this project requires the real camera to record H264 videos finally, you
needs a RPI to run or test. But you could also uses a cross-compiling
environment for compiling and developing. Reference the
[docker folder](https://github.com/Pragmatic-Elixir-Meetup/rpi-video-rs/tree/master/tools/docker)
of project [rpi-video-rs](https://github.com/Pragmatic-Elixir-Meetup/rpi-video-rs)
which is a separate cross-compiling environment.

For developing on either a RPI device or a Docker container, you should install
the standard Rust development environment, and then adds Rust targets as below.

```
rustup target add arm-unknown-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf
```

Reference below for how to enable a camera on Raspberry Pi.

### Installing Camera Device on RPI

1. Adds text `bcm2835-v4l2` to the bottom of file `/etc/modules`.

```
sudo echo bcm2835-v4l2 >> /etc/modules
```

2. Activates the camera in configuration.

```
sudo raspi-config
```

3. Follows the prompt to reboot Raspberry PI.

4. You could use the command `raspivid` to test recording a `H264` video and
check if the camera is installed properly.

```
raspivid -o test_video.h264
```
