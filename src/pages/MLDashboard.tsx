import { For, createSignal } from "solid-js";
import { tw } from "../utils";
import pencilIcon from "/pencil.svg";
import backArrowIcon from "/back_arrow.svg";
import refreshIcon from "/refresh.svg";
import { invoke } from "@tauri-apps/api";
import { readDir, BaseDirectory } from "@tauri-apps/api/fs";

const [files, setFiles] = createSignal([]);

export function MLDashboard() {
  const [selectedModel, setSelectedModel] = createSignal("Hand Detector");

  return (
    <>
      <div
        class={tw(
          `mr-10 mt-[-.65rem] flex h-12 w-full flex-row items-center border-b-2 border-white`,
        )}
      >
        <img
          src={backArrowIcon}
          class={tw(`h-full select-none p-[.6rem] active:opacity-40`)}
          onClick={() => {
            setSelectedModel(null);
          }}
        ></img>
        <h1>{selectedModel()}</h1>
        <img
          src={refreshIcon}
          class={tw(`ml-auto h-full select-none p-[.7rem] active:opacity-40`)}
          onClick={async () => {
            setFiles(
              (await readDir("E:\\palm_reader\\recordings")).filter((x) => {
                return x.name.endsWith(".mkv");
              }),
            );
          }}
        ></img>
      </div>
      <ol class={style.table}>
        <For each={files()}>
          {({ name, path }) => {
            return (
              <Row
                text={name.substring(0, name.lastIndexOf("."))}
                path={path}
              />
            );
          }}
        </For>
      </ol>
    </>
  );
}

function Row(props) {
  async function rowClick() {
    console.log("Row clicked");
  }

  function editClick(e) {
    e.stopPropagation();
    invoke("edit_data", { path: props.path });
    console.log("Edit clicked");
  }

  return (
    <li class={style.row} onClick={rowClick}>
      <h1 class={style.text}>{props.text}</h1>
      <div class={style.icon} onClick={(e) => editClick(e)}>
        <img src={pencilIcon}></img>
      </div>
    </li>
  );
}

const style = {
  row: tw(
    `flex h-12 items-center border-b border-neutral-500 bg-neutral-700 bg-opacity-50 pl-3
    odd:bg-opacity-30 hover:bg-opacity-100 active:bg-opacity-30`,
  ),
  table: tw(`w-full flex-nowrap`),
  text: tw(``),
  icon: tw(`ml-auto h-12 w-12 select-none p-[.7rem] opacity-50
    hover:opacity-100 active:p-[.75rem] active:opacity-60`),
};
