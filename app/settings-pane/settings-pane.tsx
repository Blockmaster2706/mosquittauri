"use client";

import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { topic } from "../types";
import AddTopic from "./add-topic";
import AddServer from "./add-server";
import TopicList from "./topic-list";
import ServerList from "./server-list";
import { Server } from "../types/server";
import { invoke } from "@tauri-apps/api/core";
import EditServer from "./edit-server";
import commands from "../types/commands";
import { emit, listen } from "@tauri-apps/api/event";

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
	EditServer,
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
	const [serverList, setServerList] = useState<Server[]>([]);
	const [serverToEdit, setServerToEdit] = useState<Server>({
		id: 0,
		name: "",
		url: "",
		port: 1883,
		clientId: "",
	});
	const [selectedServerID, setSelectedServerID] = useState<number>(-1);

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
			{ id: topicList.length, name: value, enabled: false },
		]);
		setTopic(""); // Clear the input after submitting
	};

	const handleClick = (topic: topic) => {
		invoke(commands.set_topic_enabled, {
			enabled: !topic.enabled,
			id: topic.id,
		});
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

	useEffect(() => {
		const unlisten = listen("server-selected", (event) => {
			const updatedServerID = event.payload;
			console.log("Received server selecton update event:", event);
			const newServerID = updatedServerID as { id: number };
			console.log("New server selection:", newServerID);

			setSelectedServerID(newServerID.id);
			invoke(commands.get_topics);
			setMode(Mode.TopicList);
		});

		invoke(commands.get_servers);

		return () => {
			unlisten.then((f) => f());
		};
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
							setServerToEdit={setServerToEdit}
							setEditMode={() => setMode(Mode.EditServer)}
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

								invoke(commands.add_server, {
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
							mode === Mode.EditServer
								? "translate-x-0"
								: mode === Mode.ServerList
									? "translate-x-full"
									: "-translate-x-full"
						}`}
						style={{ opacity: mode === Mode.EditServer ? 1 : 0 }}
					>
						<EditServer
							input_classname={input_classname}
							onBackClick={() => setMode(Mode.ServerList)}
							onAddClick={(server: Server) => {
								console.log("Editing server:", server);
								invoke(commands.edit_server, {
									name: server.name,
									url: server.url,
									port: server.port,
									clientId: server.clientId,
									id: server.id,
								});
								setMode(Mode.EditServer);
							}}
							server={serverToEdit}
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
						<TopicList
							selected_server_id={selectedServerID}
							serverName={
								serverList.find((server) => server.id === selectedServerID)
									?.name || "No Server Selected"
							}
							handleClick={handleClick}
							setAddTopicMode={() => setMode(Mode.AddTopic)}
							onBackClick={() => {
								setSelectedServerID(-1);
								setMode(Mode.ServerList);
							}}
							setExternalTopicList={setTopicList}
						/>
					</div>

					<div
						className={`absolute w-full transition-transform duration-300 ease-in-out ${
							mode === Mode.AddTopic ? "translate-x-0" : "translate-x-full"
						}`}
						style={{ opacity: mode === Mode.AddTopic ? 1 : 0 }}
					>
						<AddTopic
							input_classname={input_classname}
							serverID={selectedServerID}
							handleKeyDown={handleKeyDown}
							setTopicListMode={() => setMode(Mode.TopicList)}
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
					disabled={selectedServerID === -1}
					onClick={
						connected
							? () => emit("mqtt-disconnect-request")
							: () => invoke(commands.mqtt_connect)
					}
					className={settingsButtonClassname}
				>
					{connected ? "Disconnect" : "Connect"}
				</button>
			</div>
		</div>
	);
}
