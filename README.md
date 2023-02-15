# gomastreamer

```sh
$ gomast -h
gomastreamer:
Thin Rust wrapper for gstreamer, for development, for myself

Usage: gomast [OPTIONS] <HOST> <PORT>

Arguments:
  <HOST>  host name of udpsink
  <PORT>  port number of udpsink

Options:
      --show                     Show information of devices
  -i, --input <INPUT>            Source of video [default: test] [possible values: test, mipi, usb]
  -r, --resolution <RESOLUTION>  Resolution of video [default: vga] [possible values: vga, sd, hd]
  -f, --format <FORMAT>          Format of video [default: h264] [possible values: vp8, h264]
      --hardware                 Use hardware encode
      --dry-run                  Dry-run mode, just print command
  -h, --help                     Print help
  -V, --version                  Print version

$ gomast example.com 5000
Running: gst-launch-1.0 videotestsrc ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! vp8enc ! rtpvp8pay ! udpsink  host=example.com  port=5000
Setting pipeline to PAUSED ...
Pipeline is PREROLLING ...
Redistribute latency...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
Redistribute latency...

$ gomast -i mipi -r hd -f h264 --hardware example.com 5000
Running: gst-launch-1.0 libcamerasrc ! video/x-raw,width=1280,height=720,framerate=30/1 ! videoconvert ! v4l2h264enc 'video/x-h264,level=(string)4' ! rtph264pay ! udpsink  host=example.com  port=5000

$ gomast --show | grep h264
applemedia:  vtenc_h264: H.264 encoder
applemedia:  vtenc_h264_hw: H.264 (HW only) encoder
codectimestamper:  h264timestamper: H.264 timestamper
rtp:  rtph264depay: RTP H264 depayloader
rtp:  rtph264pay: RTP H264 payloader
typefindfunctions: video/x-h264: h264, x264, 264
videoparsersbad:  h264parse: H.264 parser

$ gomast --dry-run -i usb -f h264 example.com 5000
gst-launch-1.0 -v v4l2src ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! x264enc ! rtph264pay ! udpsink  host=example.com  port=5000
```

## About

gomastreamer is a wrapper for GStreamer.

This tool provides a convenient command line interface for executing GStreamer shell commands.

## Dependencies

[GStreamer](https://gstreamer.freedesktop.org/)

If you are using RaspberryPiOS, you can install GStreamer with the following command:

```sh
apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-doc gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio
```

## Download

RaspberryPiOS and MacOS binaries are available from [Release](http://github.com/gomadoufu/gomastreamer/releases) page.

## Usage

Format: `gomast [OPTIONS] [HOST] [PORT]`.

The HOST and PORT arguments correspond to the host and port arguments of the GStreamer `udpsink` element. There are required arguments.

e.g. `gomast -i test -r sd -f vp8` will translate to:

```sh
gst-launch-1.0 videotestsrc ! video/x-raw,width=800,height=600,framerate=30/1 ! videoconvert ! vp8enc ! rtpvp8pay ! udpsink host=localhost port=8080
```

And `gomast example.com 5000 -i mipi -r vga -f h264 --hardware` will translate to:

```sh
gst-launch-1.0 libcamerasrc ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! v4l2h264enc 'video/x-h264,level=(string)4' ! rtph264pay ! udpsink host=example.com port=5000
```

For more information on the available options and their usage, run `gomast --help`.

## Future

- [x] Dry run mode
- [x] gomast show (= gst-inspect; show available devices)
- [ ] load environment variables
