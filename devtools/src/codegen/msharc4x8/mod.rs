use bimap::BiHashMap;
use minidsp::formats::xml_config::Setting;
use strong_xml::XmlRead;

use super::spec::*;

pub struct Target {}
impl crate::Target for Target {
    fn filename() -> &'static str {
        "msharc4x8.rs"
    }

    fn symbols() -> bimap::BiMap<String, usize> {
        symbols()
    }

    fn device() -> Device {
        device()
    }
}

pub(crate) fn input(input: usize) -> Input {
    Input {
        gate: Gate {
            enable: format!("DGain_{}_0_status", input + 1),
            gain: format!("DGain_{}_0", input + 1),
        },
        meter: format!("Meter02_C1_{}", input),
        peq: (0..10usize)
            .map(|index| format!("PEQ_{}_{}", input + 1, 10 - index))
            .collect(),
        routing: (0..4usize)
            .map(|output| Gate {
                enable: format!("MixerNxMSmoothed1_{}_{}_status", input, output),
                gain: format!("MixerNxMSmoothed1_{}_{}", input, output),
            })
            .collect(),
    }
}

pub(crate) fn output(output: usize) -> Output {
    Output {
        gate: Gate {
            enable: format!("DGain_{}_0_status", 3 + output),
            gain: format!("DGain_{}_0", 3 + output),
        },
        meter: format!("Meter10_C1_{}", 4 + output),
        delay_addr: format!("Delay_{}_0", 3 + output),
        invert_addr: format!("polarity_out_{}_0", 1 + output),
        peq: (0..10usize)
            .map(|index| format!("PEQ_{}_{}", output + 3, 10 - index))
            .collect(),
        xover: Some(Crossover {
            peqs: [1, 5]
                .iter()
                .map(|group| format!("BPF_{}_{}", output + 3, group))
                .collect(),
        }),
        compressor: Some(Compressor {
            bypass: format!("COMP_{}_0_status", output + 3),
            threshold: format!("COMP_{}_0_threshold", output + 3),
            ratio: format!("COMP_{}_0_ratio", output + 3),
            attack: format!("COMP_{}_0_atime", output + 3),
            release: format!("COMP_{}_0_rtime", output + 3),
            meter: format!("Meter10_C1_{}", output),
        }),
        fir: Some(Fir {
            index: output as u8,
            num_coefficients: format!("FIR_{}_0_Taps", output + 3),
            bypass: format!("FIR_{}_0_status", output + 3),
            max_coefficients: 2048,
        }),
    }
}


pub fn device() -> Device {
    Device {
        product_name: "MiniSHARC 4x8".into(),
        sources: Vec::new(),
        inputs: (0..4).map(input).collect(),
        outputs: (0..8).map(output).collect(),
        fir_max_taps: 9600,

        // FIXME: This depends on the installed plugin
        internal_sampling_rate: 96000,
    }
}

pub fn symbols() -> BiHashMap<String, usize> {
    let cfg = include_str!("config.xml");
    Setting::from_str(cfg).unwrap().name_map()
}


#[cfg(test)]
#[test]
fn test_codegen() {
    let mut symbol_map = symbols();
    let spec = device();
    super::generate_static_config(&mut symbol_map, &spec).to_string();
}