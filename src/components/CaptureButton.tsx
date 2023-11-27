import { createSignal } from "solid-js";
import { tw } from "../utils";

export function CaptureButton(props) {
  function handleClick() {
    props.onClick();
  }

  const message = () =>
    props.isActive ? props.activeText : props.inactiveText;

  return (
    <>
      <div
        class={tw(
          style.container,
          props.isActive ? "border-white" : "border-transparent",
        )}
        onClick={handleClick}
      >
        <div
          class={tw(style.recording_dot, props.isActive ? "block" : "hidden")}
        ></div>

        <h1 class={tw(style.text, props.isActive ? "animate-pulse" : "")}>
          {message()}
        </h1>
      </div>
    </>
  );
}

const style = {
  container: tw(
    `flex w-full select-none flex-row items-center justify-center space-x-2 rounded-full border-2 bg-neutral-600 p-3 pr-6 active:opacity-40`,
  ),
  recording_dot: tw(`h-6 w-6 animate-pulse rounded-full border-2 bg-red-600`),
  text: tw(``),
};
