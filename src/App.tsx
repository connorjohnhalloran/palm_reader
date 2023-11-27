import { CapturePage } from "./pages/CapturePage";
import { tw } from "./utils";

export function App() {
  //let [status, setStatus] = useContext(StatusContext);

  return (
    <>
      <div class={style.container}>
        <CapturePage />
      </div>
    </>
  );
}

const style = {
  container: tw(
    `scrollbar-gutter flex h-screen max-h-screen w-full items-center justify-center  bg-mb-gray text-white`,
  ),
};
