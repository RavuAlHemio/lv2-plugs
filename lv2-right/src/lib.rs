use lv2::prelude::*;


#[derive(PortCollection)]
struct Ports {
    _left: InputPort<Audio>,
    right: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[uri("https://ondrahosek.com/uri/lv2/right")]
struct Right;

impl Plugin for Right {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self)
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
        for (right_frame, out_frame) in ports.right.iter().zip(ports.output.iter_mut()) {
            *out_frame = *right_frame;
        }
    }
}

lv2_descriptors!(Right);
