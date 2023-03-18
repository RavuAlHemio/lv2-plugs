use lv2::prelude::*;


#[derive(PortCollection)]
struct Ports {
    left: InputPort<Audio>,
    right: InputPort<Audio>,
    output: OutputPort<Audio>,
    mode: InputPort<Control>,
}

#[uri("https://ondrahosek.com/uri/lv2/channeling")]
struct Channeling;

impl Plugin for Channeling {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self)
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
        let mode_value = *ports.mode;
        let zippy = ports.left.iter()
            .zip(ports.right.iter())
            .zip(ports.output.iter_mut());
        for ((left_frame, right_frame), out_frame) in zippy {
            // cutoff at halfway point, round ties up
            // e.g. canonical value 1.0 => >= 0.5, < 1.5

            if mode_value < -0.5 {
                // misconfiguration
                *out_frame = 0.0;
            } else if mode_value < 0.5 {
                // Average
                *out_frame = (*left_frame + *right_frame) / 2.0;
            } else if mode_value < 1.5 {
                // Left
                *out_frame = *left_frame;
            } else if mode_value < 2.5 {
                // Right
                *out_frame = *right_frame;
            } else if mode_value < 3.5 {
                // Out-of-Phase Stereo
                *out_frame = *left_frame - *right_frame;
            } else if mode_value < 4.5 {
                // Min
                *out_frame = left_frame.min(*right_frame);
            } else if mode_value < 5.5 {
                // Max
                *out_frame = left_frame.max(*right_frame);
            } else {
                // misconfiguration
                *out_frame = 0.0;
            }
        }
    }
}

lv2_descriptors!(Channeling);
