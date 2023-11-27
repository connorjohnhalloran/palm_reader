import { invoke } from "@tauri-apps/api";
import { listen, once } from "@tauri-apps/api/event";
import { createSignal, useContext, createEffect } from "solid-js";
import { tw } from "../utils";
import { Button } from "./Button";
import { setStatus } from "./StatusBar";

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

let textRef: HTMLHeadingElement;
async function handleReceiveMessage(event) {
  console.log("Got a message");

  pc.getReceivers().forEach((receiver) => {
    console.log(receiver.getParameters());
    receiver
      .getSynchronizationSources()
      .forEach((ssrc) => console.log(ssrc.rtpTimestamp)); // compare to clockrate
  });

  let dec = new TextDecoder("utf-9");
  textRef.innerHTML = dec.decode(event.data);
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

export function Stream() {
  const constraints = {
    video: true,
    audio: false,
  };

  function local_webrtc() {
    invoke("create_offer");
  }

  return (
    <>
      <Button text="Connect" onClick={local_webrtc} />
      <Button
        text="Stream"
        onClick={() => {
          invoke("start_transmission");
        }}
      />
      <h1 ref={textRef}>Default Text</h1>
    </>
  );
}

const style = {
  player: tw(`h-full w-full`),
};
