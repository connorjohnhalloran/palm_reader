# Palm Reader
> An ensemble classification model that reads hand positions. Created with PyTorch, Rust, Solid JS, Tauri, and Azure Kinect DK camera.

Palm Reader takes in 30fps depth data from an Azure Kinect DK and classifies the hand position using an ensemble of convolutional neural networks. Predictions are displayed to the user in real time with no perceptible latency.

Recognized hand gestures:
- Open Hand
- Fist
- Peace Sign
- Point
- Shaka/Hang Ten

![Screenshot](https://raw.githubusercontent.com/connorjohnhalloran/palm_reader/main/public/screenshot.png)
