"use client";

import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { topic } from "../types";
import AddTopic from "./add-topic";
import AddServer from "./add-server";
import TopicList from "./topic-list";
import ServerList from "./server-list";
import { Server } from "../types/server";
import { invoke } from "@tauri-apps/api/core";

interface SettingsPageProps {
	topicList: topic[];
	setTopicList: Dispatch<SetStateAction<topic[]>>;
	address: string;
	connected: boolean;
	setConnected: Dispatch<SetStateAction<boolean>>;
}

enum Mode {
	ServerList,
	AddServer,
	TopicList,
	AddTopic,
}

export const settingsButtonClassname =
	"w-[calc(100%-10px)] cursor-pointer disabled:bg-gray60 disabled:text-gray30 disabled:border-gray100 disabled:cursor-not-allowed h-7 bg-[var(--accent)] text-gray100 border-2 enabled:hover:bg-gray80 border-accent enabled:hover:border-accent enabled:hover:text-accent duration-100";

export default function SettingsPage({
	topicList,
	setTopicList,
	connected,
	setConnected,
	address,
}: SettingsPageProps) {
	const themes = ["mosquittauri", "flashbang", "UwU"];
	const [currentTheme, setCurrentTheme] = useState("mosquittauri");

	function setTheme(theme: string) {
		console.log("Setting Theme to: ", theme);
		document.documentElement.setAttribute("data-theme", theme);
		setCurrentTheme(theme);
	}

	const [mode, setMode] = useState(Mode.ServerList);
	const [topic, setTopic] = useState("");
	const [serverList, setServerList] = useState<Server[]>([
		{
			id: 0,
			name: "Mosquitto",
			url: "localhost",
			port: 1883,
			clientId: "mosquittauri-client-0",
		},
		{
			id: 1,
			name: "HiveMQ",
			url: "broker.hivemq.com",
			port: 1883,
			clientId: "mosquittauri-client-1",
		},
		{
			id: 2,
			name: "EMQX",
			url: "broker.emqx.io",
			port: 1883,
			clientId: "mosquittauri-client-2",
		},
	]);

	const input_classname =
		"w-[calc(100%-10px)] bg-transparent text-base text-gray20 border-b-[2px] border-gray20 outline-none transition-opacity duration-300 placeholder:text-gray20 focus:opacity-100 focus:border-[var(--accent)]";

	const connected_label = connected ? (
		<label className="text-green-500 pl-2 pb-5">Connected</label>
	) : (
		<label className="text-red-500 pl-2 pb-5">Disconnected</label>
	);

	const handleKeyDown = (event: { key: string }) => {
		if (event.key === "Enter") {
			handleSubmit(topic);
		}
	};

	const handleSubmit = (value: string) => {
		// Process the input value
		if (value === "") return;
		setTopicList([
			...topicList,
			{ id: topicList.length, name: value, selected: false },
		]);
		setTopic(""); // Clear the input after submitting
	};

	const handleClick = (id: number) => {
		const topicListCopy = [...topicList];
		const index = topicList.findIndex((value) => value.id === id);
		console.log(`${id} is currently ${topicListCopy[index].selected}`);
		topicListCopy[index].selected = !topicListCopy[index].selected;
		setTopicList(topicListCopy);
	};

	useEffect(() => {
		if (
			window.matchMedia &&
			window.matchMedia("(prefers-color-scheme: dark)").matches
		) {
		} else setTheme("flashbang");
		setCurrentTheme(
			document.documentElement.getAttribute("data-theme") || themes[0],
		);
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	return (
		<div className="w-full bg-gray80 h-screen pt-2 pl-2 flex flex-col -z-10">
			<div className="-ml-2 flex gap-2 mb-4 text-gray20 bg-transparent">
				<select
					title="Change the Application Theme"
					className={
						"text-center ml-auto mr-auto bg-transparent text-base items-center text-accent hover:text-accentHover hover:cursor-pointer outline-none transition-opacity duration-300 placeholder:text-gray20 focus:opacity-100 appearance-none"
					}
					onChange={(e) => setTheme(e.target.value)}
					value={currentTheme}
				>
					{themes.map((theme) => (
						<option key={theme} value={theme}>
							{theme.charAt(0).toUpperCase() + theme.slice(1)}
						</option>
					))}
				</select>
			</div>

			<div className="h-full gap-2 mb-4 text-gray20 bg-transparent">
				{/* Animation container with positioning */}
				<div className="relative w-full h-full">
					{/* Render all components with absolute positioning and transition styles */}
					<div
						className={`absolute w-full transition-transform duration-300 ease-in-out ${
							mode === Mode.ServerList ? "translate-x-0" : "-translate-x-full"
						}`}
						style={{ opacity: mode === Mode.ServerList ? 1 : 0 }}
					>
						<ServerList
							serverList={serverList}
							handleClick={() => setMode(Mode.AddServer)}
							setServerList={setServerList}
						/>
					</div>

					<div
						className={`absolute w-full transition-transform duration-300 ease-in-out ${
							mode === Mode.AddServer
								? "translate-x-0"
								: mode === Mode.ServerList
									? "translate-x-full"
									: "-translate-x-full"
						}`}
						style={{ opacity: mode === Mode.AddServer ? 1 : 0 }}
					>
						<AddServer
							input_classname={input_classname}
							onBackClick={() => setMode(Mode.ServerList)}
							onAddClick={(name: string, url: string, port: number) => {
								// Convert URL format properly
								let hostname;
								try {
									// Add protocol if missing, so we can extract hostname properly
									if (
										!url.startsWith("http://") &&
										!url.startsWith("https://")
									) {
										hostname = new URL(`http://${url}`);
									} else {
										hostname = new URL(url);
									}
								} catch (e) {
									// If URL is invalid, just use it as is
									console.error("Invalid URL:", e);
								}

								invoke("add_server", {
									name: name,
									url: hostname,
									port: port,
									clientId: `mosquittauri-client-${serverList.length}`,
								});
								setMode(Mode.ServerList);
							}}
						/>
					</div>

					<div
						className={`absolute w-full transition-transform duration-300 ease-in-out ${
							mode === Mode.TopicList
								? "translate-x-0"
								: mode === Mode.ServerList || mode === Mode.AddServer
									? "translate-x-full"
									: "-translate-x-full"
						}`}
						style={{ opacity: mode === Mode.TopicList ? 1 : 0 }}
					>
						<TopicList topicList={topicList} handleClick={handleClick} />
					</div>

					<div
						className={`absolute w-full transition-transform duration-300 ease-in-out ${
							mode === Mode.AddTopic ? "translate-x-0" : "translate-x-full"
						}`}
						style={{ opacity: mode === Mode.AddTopic ? 1 : 0 }}
					>
						<AddTopic
							input_classname={input_classname}
							topic={topic}
							setTopic={setTopic}
							handleKeyDown={handleKeyDown}
						/>
					</div>
				</div>
			</div>

			<div className="mt-auto mb-5 w-full">
				<label className="w-full flex pt-5 text-gray20">
					Status: {connected_label}
				</label>
				<button
					title={address === "" ? "Please input a Server Address" : ""}
					disabled={address === ""}
					onClick={() => {
						setConnected(!connected);
					}}
					className={settingsButtonClassname}
				>
					Connect
				</button>
			</div>
		</div>
	);
}
