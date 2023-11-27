use tch::CModule;
use tch::Device;
use tch::Tensor;

pub struct V1 {
    model: CModule,
}

impl V1 {
    pub fn new() -> V1 {
        let mut v1 = CModule::load("E:\\palm_reader\\src-tauri\\src\\models\\ensemble.pt")
            .expect("Unable to load v1 model.");
        v1.set_eval();
        V1 { model: v1 }
    }
    // REWORK TO INSTANTIATE MODEL AND RUN WITH A v1.something()
    pub fn eval(&self, input: Vec<i32>) -> String {
        let tx = Tensor::from_slice(&input[..])
            .to(Device::Cuda(0))
            .reshape([576, 640])
            .unsqueeze(0);

        let x = self
            .model
            .forward_ts(&[tx])
            .expect("Error while running model.");

        let prediction = x.argmax(1, false).int64_value(&[0]);

        match prediction {
            0 => "Open Hand",
            1 => "Fist",
            2 => "Peace Sign",
            3 => "Point",
            4 => "Hang Ten",
            _ => panic!(),
        }
        .into()
    }
}
