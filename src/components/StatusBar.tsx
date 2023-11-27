import errorIcon from "/error.svg";
import checkmarkIcon from "/checkmark.svg";
import { createEffect, createSignal } from "solid-js";
import { tw } from "../utils";

const x = createSignal("S Status goes here!");
export const status = x[0];
export const setStatus = (message) => {
  x[1](""); // Reset so identical messages trigger css anim
  x[1](message);
};

export function StatusBar() {
  let ref: HTMLDivElement;

  // Restart css animation when status changes
  createEffect(() => {
    status();
    ref.style.animation = "None";
    ref.offsetHeight; // Trigger reflow
    ref.style.animation = null;
  });

  const icon = () => determineStyle(status()).icon;
  const color = () => determineStyle(status()).color;
  const message = () => status().slice(2);

  return (
    <>
      <div class={tw(style.statusBar, "bg-neutral-600 drop-shadow-lg")}>
        <div
          ref={ref}
          class={tw(
            style.statusBar,
            color(),
            "absolute w-full animate-fade-out px-2",
          )}
        >
          <img src={icon()} class={style.statusIcon} />
          <h1 class={style.statusText}>{message()}</h1>
        </div>
        <img src={icon()} class={style.statusIcon} />
        <h1 class={style.statusText}>{message()}</h1>
      </div>
    </>
  );
}

function determineStyle(status: string): { icon: string; color: string } {
  switch (status.charAt(0)) {
    case "S":
      return { icon: checkmarkIcon, color: "bg-green-500" };
    case "W":
      return { icon: errorIcon, color: "bg-yellow-500" };
    case "E":
      return { icon: errorIcon, color: "bg-red-700" };
    default:
      return { icon: errorIcon, color: "bg-pink-500" };
  }
}

const style = {
  statusBar: tw(`w-screen-0 flex h-8 flex-row items-center space-x-2`),
  statusIcon: tw(`h-6 w-6`),
  statusText: tw(`font-mono font-bold text-white opacity-100`),
};
