# gomastreamer

### Thin Rust wrapper for gstreamer, for development, for myself.

## About

gomastreamer is a Rust wrapper for GStreamer, a multimedia framework for building pipelines to process and transmit multimedia content. This library provides a convenient command line interface for executing GStreamer shell commands on Raspberry Pi OS.

## Usage

The gomast command takes options and arguments, in the format of `gomast [OPTIONS] [HOST] [PORT]`. The HOST and PORT arguments correspond to the host and port arguments of the GStreamer `udpsink` element.

For example, running `gomast -i test -r sd -f vp8` will translate to the following GStreamer shell command:

```sh
gst-launch-1.0 videotestsrc ! video/x-raw,width=800,height=600,framerate=30/1 ! videoconvert ! vp8enc ! rtpvp8pay ! udpsink host=localhost port=8080
```

And `gomast example.com 5000 -i mipi -r vga -f h254 --hardware` will translate to:

```sh
gst-launch-1.0 libcamerasrc ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! v4l2h264enc 'video/x-h254,level=(string)4' ! rtph264pay ! udpsink host=example.com port=5000
```

For more information on the available options and their usage, run `gomast --help`.
