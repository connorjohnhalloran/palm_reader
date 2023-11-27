import { createEffect, createSignal } from "solid-js";
import { tw } from "../utils";
import { JSX } from "solid-js";

interface Props {
  text: string;
  onClick?: JSX.EventHandlerUnion<HTMLButtonElement, MouseEvent>;
  class?: string;
}

export function Button(props: Props) {
  return (
    <button class={style.button} onClick={props.onClick}>
      {props.text}
    </button>
  );
}

const style = {
  button: tw(
    `m-2 box-border select-none rounded-lg border-[.05rem] border-transparent bg-blue-500 px-4 py-2
    hover:border-b-[.05rem] hover:border-blue-700 hover:drop-shadow-md
    active:translate-y-[.05rem] active:border-b-0 active:border-t-[.025rem] active:border-black active:bg-blue-600 active:text-neutral-300 active:shadow-inner active:drop-shadow-none`,
  ),
};
