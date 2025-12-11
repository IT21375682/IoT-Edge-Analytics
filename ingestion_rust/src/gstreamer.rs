extern crate gstreamer as gst;
use gst::prelude::*;

pub fn start_video_stream() {
    gst::init().unwrap();
    let pipeline = gst::parse_launch("v4l2src ! videoconvert ! autovideosink").unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();
    let bus = pipeline.bus().unwrap();

    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        match msg.view() {
            gst::MessageView::Eos(_) => break,
            gst::MessageView::Error(err) => {
                eprintln!("Error: {}", err);
                break;
            },
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();
}
