import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { settingsButtonClassname } from "./settings-pane";
import commands from "../types/commands";

interface AddTopicProps {
	input_classname: string;
	serverID: number;
	handleKeyDown: (event: { key: string }) => void;
	setTopicListMode: () => void;
}

export default function AddTopic({
	input_classname,
	serverID,
	handleKeyDown,
	setTopicListMode,
}: AddTopicProps) {
	const [topic, setTopic] = useState("");
	return (
		<div
			onMouseUpCapture={(event) => {
				console.log("Mouse up event:", event.button);
				if (event.button === 3) {
					setTopicListMode();
				}
			}}
		>
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
			<button
				className={settingsButtonClassname + " mt-5"}
				onClick={() => {
					if (topic.trim() !== "") {
						invoke(commands.add_topic, {
							name: topic,
							serverId: serverID,
						});
						setTopic("");
						setTopicListMode();
					}
				}}
			>
				Add Topic
			</button>
		</div>
	);
}
