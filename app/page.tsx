'use client'

import { useEffect, useState } from "react";
import MessageView, { message } from "./message-view";
import PaneSwitcher from "./pane_switcher";
import PublishBar from "./publish-bar";
import SettingsPage from "./settings-pane";
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { topic } from "./types";

export default function Home() {

  const [isLogsPaneActive, setLogsPaneActive] = useState(false)
  const [isMQTTConnected, setIsMQTTConnected] = useState(false)
  const [topicList, setTopicList] = useState<topic[]>([])
  const [address, setAddress] = useState("")

  const [MQTTMessageArray, setMQTTMessageArray] = useState<message[]>([])

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const setupListener = async () => {
      await listen<message>('newMessage', (payload) => {
        setMQTTMessageArray((prevState) => [...prevState, payload.payload])
      })
    }

    setupListener();

    return () => {
      if (unlisten) { // Check if unlisten is defined
        unlisten();
      }
    };
  }, [])

  const LogsMessageArray = [
    {timestamp: "8am", message: "Dies ist ein Logs Test", topic: "test"},
    {timestamp: "9:30am", message: "Logs sind Cool", topic: "test"},
    {timestamp: "10:21am", message: "Mein Mitazubi steht richtig hart auf Baumstämme oder wie man das schreibt", topic: "Holzbrötchen"},
    {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
    {timestamp: "11:11am", message: "Dies ist ein *weiterer* Logs Test", topic: "test"},
    {timestamp: "69am", message: "Lol funny Logs", topic: "hehe"},
    {timestamp: "420am", message: "Wann gratis Logs für alle, Herr Habeck?", topic: "Brokkologs"},
    {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
    {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
    {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
    {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
    {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
  ]

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start max-h-screen max-w-screen overflow-hidden">
        <div className="absolute grid grid-cols-100 top-0 left-0 w-screen h-screen max-h-screen max-w-screen">
          <div className="col-start-1 col-span-20 h-screen max-h-screen max-w-screen w-full z-30">
            <SettingsPage topicList={topicList} setTopicList={setTopicList} connected={isMQTTConnected} setConnected={setIsMQTTConnected} address={address} setAddress={setAddress}/>
          </div>

          <div className="col-start-22 col-span-70 w-full flex flex-col h-screen max-h-screen max-w-screen">
            <div className="h-screen overflow-y-scroll hide-scrollbar z-10 scroll-pb-40">
              <MessageView messageArray={isLogsPaneActive ? LogsMessageArray : MQTTMessageArray}/>
            </div>
            <div className="h-[120px] -mt-30 relative flex flex-col">
              <div className="h-[40px] w-full col-start-5 col-span-14 mt-18 bg-transparent z-30">
                <PublishBar topicList={topicList} enabled={isMQTTConnected}/>
              </div>
            </div>
          </div>

          <div className="col-start-92 col-span-9 h-full flex flex-col items-center justify-end z-30">
            <div className="mb-5 -mt-17 h-12 w-12">
              <PaneSwitcher isShowingLogs={isLogsPaneActive} setShowingLogs={setLogsPaneActive}/>
            </div>
          </div>

        </div>
        <div className="absolute left-0 bottom-0 h-[120px] w-screen bg-linear-to-b from-transparent via-black to-black z-20"/>
      </main>
    </div>
  );
}