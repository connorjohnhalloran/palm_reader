import { Button } from "../components/Button";
import { CaptureButton } from "../components/CaptureButton";
import { Match, Show, Switch, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

import openHandIcon from "/open_hand.svg";
import fistIcon from "/fist.svg";
import peaceIcon from "/peace.svg";
import pointIcon from "/point.svg";
import hangTenIcon from "/shaka.svg";
import { tw } from "../utils";

let pc = new RTCPeerConnection();

pc.onicecandidate = (event) => {
  if (event.candidate) {
    invoke("register_ice", {
      candidate: event.candidate.candidate,
      sdpMid: event.candidate.sdpMid,
      sdpMlineIndex: event.candidate.sdpMLineIndex,
      usernameFragment: event.candidate.usernameFragment,
    });
  }
};

pc.ondatachannel = (event) => {
  console.log("Data channel received.");
  event.channel.onmessage = handleReceiveMessage;
};

const [currentPose, setCurrentPose] = createSignal("");
async function handleReceiveMessage(event) {
  console.log("Got a message");

  pc.getReceivers().forEach((receiver) => {
    console.log(receiver.getParameters());
    receiver
      .getSynchronizationSources()
      .forEach((ssrc) => console.log(ssrc.rtpTimestamp)); // compare to clockrate
  });

  let dec = new TextDecoder("utf-8");
  setCurrentPose(dec.decode(event.data));
}

const unlisten = listen("backend_offer_ready", async (event) => {
  let sdp = event.payload;

  let sd = new RTCSessionDescription({ type: "offer", sdp: sdp as string });
  await pc.setRemoteDescription(sd);
  let answer = await pc.createAnswer();
  await pc.setLocalDescription(answer);
  // onicecandidate TRIGGERS HERE
  invoke("remote_ready", { answer: answer.sdp });
});

const ul = listen("register_client_ice_candidate", (event) => {
  pc.addIceCandidate(event.payload);
});

export function CapturePage() {
  const [isConnected, setIsConnected] = createSignal(false);
  const [isCapturing, setIsCapturing] = createSignal(false);

  function handleConnectButtonClick() {
    invoke("create_offer");
    setIsConnected(true);
  }

  function handleCaptureButtonClick() {
    if (isCapturing()) {
      setIsCapturing(false);
      invoke("stop_capture");
    } else {
      setIsCapturing(true);
      invoke("start_capture");
    }
  }

  const icon = () => {
    switch (currentPose()) {
      case "Open Hand":
        return openHandIcon;
      case "Fist":
        return fistIcon;
      case "Peace Sign":
        return peaceIcon;
      case "Point":
        return pointIcon;
      case "Hang Ten":
        return hangTenIcon;
    }
  };

  return (
    <>
      <Show
        when={isConnected()}
        fallback={<Button text="Connect" onClick={handleConnectButtonClick} />}
      >
        <div class="m-4 justify-center">
          <CaptureButton
            inactiveText="Start Capture"
            activeText="Capturing..."
            isActive={isCapturing()}
            onClick={handleCaptureButtonClick}
          />
          <div
            class={tw(
              "mt-2 flex h-64 w-64 flex-col items-center rounded-3xl bg-neutral-200",
            )}
          >
            <h1 class={tw("mt-6 text-xl font-bold text-black")}>
              {currentPose()}
            </h1>
            <img src={icon()} class={tw("mt-4 w-32")} />
          </div>
        </div>
      </Show>
    </>
  );
}
