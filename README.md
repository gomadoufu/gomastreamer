# gomastreamer

Thin wrapper for GStreamer, for development and testing on RaspberryPi.

```sh
# use default options (test video, vga, 30fps, h264)
$ gomast example.com 5000
Running: gst-launch-1.0 videotestsrc ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! x264enc ! rtph264pay ! udpsink  host=example.com  port=5000
Setting pipeline to PAUSED ...
Pipeline is PREROLLING ...
...

# use custom options (mipi video, hd, 60fps, vp8)
$ gomast myserver.com 7000 -i mipi -f 60 -t vp8 -r hd
Running: gst-launch-1.0 libcamerasrc ! video/x-raw,width=1280,height=720,framerate=60/1 ! videoconvert ! vp8enc ! rtpvp8pay ! udpsink  host=myserver.com  port=7000
Setting pipeline to PAUSED ...
Pipeline is PREROLLING ...
...

# use hardware encoder
$ gomast myserver.com 7000 -9 usb --hardware
Running: gst-launch-1.0 -v v4l2src ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! v4l2h264enc ! 'video/x-h264,level=(string)4' ! rtph264pay ! udpsink  host=myserver.com  port=7000
Setting pipeline to PAUSED ...
Pipeline is PREROLLING ...
...

# dry-run mode
$ gomast myserver.com 7000 --dry-run
gst-launch-1.0 videotestsrc ! video/x-raw,width=640,height=480,framerate=30/1 ! videoconvert ! x264enc ! rtph264pay ! udpsink  host=myserver.com  port=7000

# show available devices
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

Format: `gomast [OPTIONS] [HOST] [PORT]`.

You can specify the following options:

-i, --input : Source of video [default: test] [possible values: test, mipi, usb]

-r, --resolution : Resolution of video [default: vga] [possible values: vga, sd, hd]

-f, --framerate : Framerate of video [default: 30]

-t, --type : Format of video [default: h264] [possible values: vp8, h264]

--hardware : Use hardware encode

--dry-run : Dry-run mode, just print command

--show : Show information of devices

The HOST and PORT arguments correspond to the host and port arguments of the GStreamer `udpsink` element. There are required arguments.

For more information on the available options and their usage, run `gomast --help`.

## Future

- [x] Dry run mode
- [x] gomast show (= gst-inspect; show available devices)
- [ ] load environment variables
