# gomastreamer

Thin wrapper for GStreamer, for development and testing on RaspberryPi.

```sh
# input: testpattern, YUY2
# output: 640x480, vp8, example.com:5004
$ gomast input test yuy2 output 640 480 vp8 example.com 5004
Running: gst-launch-1.0 videotestsrc ! video/x-raw,format=YUY2,width=640,height=480 ! videoconvert ! vp8enc ! rtpvp8pay ! udpsink host=example.com port=5004
Setting pipeline to PAUSED ...
Pipeline is PREROLLING ...
Redistribute latency...
Pipeline is PREROLLED ...
Setting pipeline to PLAYING ...
Redistribute latency...
New clock: GstSystemClock
0:00:06.3 / 99:99:99.

# input: usb camera, YUY2
# output: 1280x720, h264, example.com:7001, hardware encode
# dry-run mode
$ gomast input usb yuy2 output 1280 720 h264 example.com 7001 --hardware --dry-run
gst-launch-1.0 -v v4l2src ! video/x-raw,format=YUY2,width=1280,height=720 ! videoconvert ! v4l2h264enc ! 'video/x-h264,level=(string)4' ! rtph264pay ! udpsink host=example.com port=7001

# show available devices
$ gomast show | grep h264
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

## Dependencies

[GStreamer](https://gstreamer.freedesktop.org/)

If you are using RaspberryPiOS, you can install GStreamer with the following command:

```sh
apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-doc gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio
```

## Download

RaspberryPiOS(armv7-unknown-linux-musl), Linux(unknown-linux-musl), and MacOS(apple-darwin)
binaries are available from [Release](http://github.com/gomadoufu/gomastreamer/releases) page.

## Usage

`gomast input <INPUT_TYPE> <FORMAT> output <WIDTH> <HEIGHT> <ENCODER> <HOST> <PORT> [options]`

The HOST and PORT arguments correspond to the host and port arguments of the GStreamer `udpsink` element. There are required arguments.

For more information on the available options and their usage, run `gomast --help`.

## Future

- [x] Dry run mode
- [x] gomast show (= gst-inspect; show available devices)
- [ ] load environment variables
