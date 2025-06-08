"use client";

import { useEffect, useState } from "react";
import MessageView, { message } from "./message-view";
import PublishBar from "./publish-bar";
import SettingsPage from "./settings-pane/settings-pane";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { topic } from "./types";
import SecondarySidebar from "./secondary-sidebar";
import LogsMessageView, { logMessage } from "./logs-message-view";

export default function Home() {
	const [isLogsPaneActive, setLogsPaneActive] = useState(false);
	const [isMQTTConnected, setIsMQTTConnected] = useState(false);
	const [topicList, setTopicList] = useState<topic[]>([]);
	const [inputValue, setInputValue] = useState("");
	const [topic, setTopic] = useState<topic | null>(null);

	const [MQTTMessageArray, setMQTTMessageArray] = useState<message[]>([]);
	const [logMessageArray, setLogMessageArray] = useState<logMessage[]>([]);

	const [autoScrollingDisabled, setAutoScrollingDisabled] = useState(false);

	const [errorCount, setErrorCount] = useState(0);
	const [warningCount, setWarningCount] = useState(0);

	useEffect(() => {
		const logUnlisten = listen("log", (event) => {
			const newMessage = event.payload as logMessage;

			console.log("Log message array updated:", newMessage);
			if (newMessage?.level.toLowerCase() === "error" && !isLogsPaneActive) {
				setErrorCount((prevCount) => prevCount + 1);
			}
			if (newMessage?.level.toLowerCase() === "warning" && !isLogsPaneActive) {
				setWarningCount((prevCount) => prevCount + 1);
			}

			setLogMessageArray((prevMessages: logMessage[]) => {
				return [...prevMessages, newMessage];
			});
		});

		const mqttConnectUnlisten = listen("mqtt-connect", () => {
			console.log("MQTT connected");
			setIsMQTTConnected(true);
		});

		const mqttDisconnectUnlisten = listen("mqtt-disconnect", () => {
			console.log("MQTT disconnected");
			setIsMQTTConnected(false);
		});

		let unlisten: UnlistenFn | undefined;

		const setupListener = async () => {
			type MQTTPayloadPayload = {
				messages: MQTTPayload[];
			};
			type MQTTPayload = {
				topic?: string;
				payload?: string;
				timestamp?: number;
			};

			await listen<MQTTPayloadPayload>("mqtt-pull", (payload) => {
				console.log("Received MQTT pull event:", payload);
				if (!payload.payload || typeof payload.payload !== "object") {
					console.error("Invalid MQTT payload received:", payload);
					return;
				}
				const mqttPayload = payload.payload.messages[0];
				console.log("MQTT payload:", mqttPayload);
				if (!mqttPayload.topic || !mqttPayload.payload) {
					console.error("MQTT payload missing topic or payload:", mqttPayload);
					return;
				}
				const message: message = {
					topic: mqttPayload.topic,
					message: mqttPayload.payload,
					timestamp: mqttPayload.timestamp
						? new Date(mqttPayload.timestamp).toLocaleTimeString()
						: new Date().toLocaleTimeString(),
				};

				console.log("Received MQTT message:", payload);

				setMQTTMessageArray((prevState) => [...prevState, message]);
			});
		};

		setupListener();

		return () => {
			logUnlisten.then((f) => f());
			mqttConnectUnlisten.then((f) => f());
			mqttDisconnectUnlisten.then((f) => f());
			if (unlisten) {
				// Check if unlisten is defined
				unlisten();
			}
		};
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	useEffect(() => {
		if (autoScrollingDisabled) return;
		const element = document.getElementById(
			`message-${MQTTMessageArray.length - 1}`,
		);
		if (element) {
			element.scrollIntoView({ behavior: "smooth", block: "end" });
		}
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [MQTTMessageArray]);

	useEffect(() => {
		if (autoScrollingDisabled) return;
		const element = document.getElementById(
			`log-message-${logMessageArray.length - 1}`,
		);
		if (element) {
			element.scrollIntoView({ behavior: "smooth", block: "end" });
		}
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [logMessageArray]);

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
							{!isLogsPaneActive && (
								<MessageView messageArray={MQTTMessageArray} />
							)}
							{isLogsPaneActive && (
								<LogsMessageView messageArray={logMessageArray} />
							)}
						</div>
						<div className="h-[122px] -mt-30 relative flex flex-col">
							<div className="h-[42px] w-full col-start-5 col-span-14 mt-18 bg-transparent z-30">
								<PublishBar
									topic={topic}
									setTopic={setTopic}
									inputValue={inputValue}
									setInputValue={setInputValue}
									topicList={topicList ?? []}
									enabled={isMQTTConnected}
								/>
							</div>
						</div>
					</div>

					<div className="col-start-92 col-span-9 h-full flex flex-col items-center justify-end z-30">
						<div className="mb-5 -mt-17 h-51 w-12">
							<SecondarySidebar
								sendButtonEnabled={isMQTTConnected}
								inputValue={inputValue}
								topic={topic}
								setInputValue={setInputValue}
								isShowingLogs={isLogsPaneActive}
								setShowingLogs={setLogsPaneActive}
								autoScrollingDisabled={autoScrollingDisabled}
								setAutoScrollDisabled={setAutoScrollingDisabled}
								errorCount={errorCount}
								setErrorCount={setErrorCount}
								warningCount={warningCount}
								setWarningCount={setWarningCount}
							/>
						</div>
					</div>
				</div>
				<div className="absolute left-0 bottom-0 h-[120px] w-screen bg-linear-to-b from-transparent via-gray100 to-gray100 -z-20" />
			</main>
		</div>
	);
}
