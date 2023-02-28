use lv2::prelude::*;


#[derive(PortCollection)]
struct Ports {
    left: InputPort<Audio>,
    right: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[uri("https://ondrahosek.com/uri/lv2/oops")]
struct Oops;

impl Plugin for Oops {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self)
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
        for ((left_frame, right_frame), out_frame) in ports.left.iter().zip(ports.right.iter()).zip(ports.output.iter_mut()) {
            *out_frame = *left_frame - *right_frame;
        }
    }
}

lv2_descriptors!(Oops);
