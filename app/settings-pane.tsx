"use client";

import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { topic } from "./types";

interface SettingsPageProps {
	topicList: topic[];
	setTopicList: Dispatch<SetStateAction<topic[]>>;
	address: string;
	setAddress: Dispatch<SetStateAction<string>>;
	connected: boolean;
	setConnected: Dispatch<SetStateAction<boolean>>;
}

export default function SettingsPage({
	topicList,
	setTopicList,
	connected,
	setConnected,
	address,
	setAddress,
}: SettingsPageProps) {
	const themes = ["mosquittauri", "flashbang", "UwU"];
	const [currentTheme, setCurrentTheme] = useState("mosquittauri");

	function setTheme(theme: string) {
		console.log("Setting Theme to: ", theme);
		document.documentElement.setAttribute("data-theme", theme);
		setCurrentTheme(theme);
	}

	const [topic, setTopic] = useState("");

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
	}, []);

	return (
		<div className="w-full bg-gray80 h-full pt-2 pl-2 flex flex-col">
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

			<label className="w-full flex pt-5 text-gray20">Server Address:</label>
			<input
				className={input_classname}
				type="text"
				title="Server Address"
				value={address}
				onChange={(event) => {
					setAddress(event.currentTarget.value);
				}}
			></input>

			<label className="w-full flex pt-5 text-gray20">Add new Topic:</label>
			<input
				className={input_classname}
				onKeyDown={handleKeyDown}
				type="text"
				placeholder=""
				title="Topic"
				value={topic}
				onChange={(event) => {
					setTopic(event.currentTarget.value);
				}}
			></input>

			<div className="w-full h-full max-h-[48%] mt-5">
				<ol className="w-full h-full overflow-y-scroll scrollbar-theme break-words">
					{topicList.map((topic) => {
						return (
							<li key={topic.id} className="w-full">
								<button
									onClick={() => handleClick(topic.id)}
									className={
										"w-full border-1 border-gray100 " +
										(topic.selected
											? "bg-accent hover:bg-accentHover text-gray100"
											: "bg-gray60 hover:bg-gray80")
									}
								>
									{topic?.name}
								</button>
							</li>
						);
					})}
				</ol>
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
					className="w-[calc(100%-20px)] cursor-pointer disabled:bg-gray60 disabled:text-gray30 disabled:border-gray100 disabled:cursor-not-allowed disabled:bg- h-10 bg-[var(--accent)] text-gray100 border-2 enabled:hover:bg-gray80 border-accent enabled:hover:border-accent enabled:hover:text-accent duration-100"
				>
					Connect
				</button>
			</div>
		</div>
	);
}
