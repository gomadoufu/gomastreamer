# gomastreamer

```sh
$ gomast -h
gomastreamer:
Thin Rust wrapper for gstreamer, for development, for myself

Usage: gomast [OPTIONS] [HOST] [PORT]

Arguments:
  [HOST]  host name of udpsink [default: localhost]
  [PORT]  port number of udpsink [default: 8080]

Options:
      --show                     show information of devices
  -i, --input <INPUT>            source of video [default: test] [possible values: test, mipi, usb]
  -r, --resolution <RESOLUTION>  resolution of video [default: vga] [possible values: vga, sd, hd]
  -f, --format <FORMAT>          format of video [default: vp8] [possible values: vp8, h264]
      --hardware                 use hardware encode
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
```

## About

gomastreamer is a wrapper for GStreamer.

This tool provides a convenient command line interface for executing GStreamer shell commands.

## Download

RaspberryPiOS and MacOS binaries are available from [Release](http://github.com/gomadoufu/gomastreamer/releases) page.

## Usage

Format: `gomast [OPTIONS] [HOST] [PORT]`.

The HOST and PORT arguments correspond to the host and port arguments of the GStreamer `udpsink` element.

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

- [ ] Dry run mode
- [x] gomast show (= gst-inspect; show available devices)
- [ ] load environment variables
