import MessageView from "./message-view";
import PaneSwitcher from "./pane_switcher";
import PublishBar from "./publish-bar";
import SettingsPage from "./settings-pane";

export default function Home() {
  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start max-h-screen max-w-screen overflow-hidden">
        <div className="absolute grid grid-cols-100 top-0 left-0 w-screen h-screen max-h-screen max-w-screen">
          <div className="col-start-1 col-span-20 h-screen max-h-screen max-w-screen w-full z-30">
            <SettingsPage/>
          </div>
          
          <div className="col-start-22 col-span-70 w-full flex flex-col h-screen max-h-screen max-w-screen">
            <div className="h-screen overflow-y-scroll hide-scrollbar z-10">
              <MessageView/>
            </div>
            <div className="h-[120px] -mt-30 relative flex flex-col">
              <div className="h-[40px] ml-4 w-full col-start-5 col-span-14 mt-18 bg-transparent z-30">
                <PublishBar/>
              </div>
            </div>
          </div>

          <div className="col-start-92 col-span-9 h-full flex flex-col items-center justify-end z-30">
            <div className="mb-5 -mt-17 h-12 w-12">
              <PaneSwitcher/>
            </div>
          </div>

        </div>
        <div className="absolute left-0 bottom-0 h-[120px] w-screen bg-linear-to-b from-transparent via-black to-black z-20"/>
      </main>
    </div>
  );
}
