"use client";

import { useEffect, useState } from "react";
import MessageView, { message } from "./message-view";
import PublishBar from "./publish-bar";
import SettingsPage from "./settings-pane/settings-pane";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { topic } from "./types";
import SecondarySidebar from "./secondary-sidebar";
import LogsMessageView from "./logs-message-view";

export default function Home() {
	const [isLogsPaneActive, setLogsPaneActive] = useState(false);
	const [isMQTTConnected, setIsMQTTConnected] = useState(false);
	const [topicList, setTopicList] = useState<topic[]>([]);
	const [inputValue, setInputValue] = useState("");
	const [topic, setTopic] = useState<topic | null>(null);

	const [MQTTMessageArray, setMQTTMessageArray] = useState<message[]>([]);

	useEffect(() => {
		let unlisten: UnlistenFn | undefined;

		const setupListener = async () => {
			await listen<message>("newMessage", (payload) => {
				setMQTTMessageArray((prevState) => [...prevState, payload.payload]);
			});
		};

		setupListener();

		return () => {
			if (unlisten) {
				// Check if unlisten is defined
				unlisten();
			}
		};
	}, []);

	

	return (
		<div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
			<main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start max-h-screen max-w-screen overflow-hidden">
				<div className="absolute grid grid-cols-100 top-0 left-0 w-screen h-screen max-h-screen max-w-screen">
					<div className="col-start-1 col-span-20 h-screen max-h-screen max-w-screen w-full z-10">
						<SettingsPage
							topicList={topicList}
							setTopicList={setTopicList}
							connected={isMQTTConnected}
							setConnected={setIsMQTTConnected}
							address={"change-me"}
						/>
					</div>

					<div className="col-start-22 col-span-70 w-full flex flex-col h-screen max-h-screen max-w-screen bg-gray100 background-image z-20">
						<div className="h-screen overflow-y-scroll hide-scrollbar z-20 scroll-pb-40 message-box">
							{
								!isLogsPaneActive && <MessageView messageArray={MQTTMessageArray}/>
							}
							{
								isLogsPaneActive && <LogsMessageView/>
							}
						</div>
						<div className="h-[122px] -mt-30 relative flex flex-col">
							<div className="h-[42px] w-full col-start-5 col-span-14 mt-18 bg-transparent z-30">
								<PublishBar
									topic={topic}
									setTopic={setTopic}
									inputValue={inputValue}
									setInputValue={setInputValue}
									topicList={topicList}
									enabled={isMQTTConnected}
								/>
							</div>
						</div>
					</div>

					<div className="col-start-92 col-span-9 h-full flex flex-col items-center justify-end z-30">
						<div className="mb-5 -mt-17 h-34 w-12">
							<SecondarySidebar
								sendButtonEnabled={isMQTTConnected}
								inputValue={inputValue}
								topic={topic}
								setInputValue={setInputValue}
								isShowingLogs={isLogsPaneActive}
								setShowingLogs={setLogsPaneActive}
							/>
						</div>
					</div>
				</div>
				<div className="absolute left-0 bottom-0 h-[120px] w-screen bg-linear-to-b from-transparent via-gray100 to-gray100 -z-20" />
			</main>
		</div>
	);
}
