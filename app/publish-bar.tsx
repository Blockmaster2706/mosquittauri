"use client";

import { emit } from "@tauri-apps/api/event";
import { Dispatch, SetStateAction } from "react";
import { message } from "./message-view";
import { topic } from "./types";
import PaginatedDropdown from "./paginated-dropdown";

interface PublishBarProps {
	topicList: topic[];
	enabled: boolean;
	inputValue: string;
	setInputValue: Dispatch<SetStateAction<string>>;
	topic: topic | null;
	setTopic: Dispatch<SetStateAction<topic | null>>;
}

export const handleSubmit = (
	inputValue: string,
	topic: topic | null,
	setInputValue: Dispatch<SetStateAction<string>>,
) => {
	// Process the input value
	const now = new Date();
	emit<message>("newMessage", {
		timestamp: now.toLocaleTimeString([], {
			hour: "2-digit",
			minute: "2-digit",
		}),
		message: inputValue,
		topic: topic?.name ?? "",
	});
	setInputValue(""); // Clear the input after submitting
};

export default function PublishBar({
	topicList,
	enabled,
	inputValue,
	setInputValue,
	topic,
	setTopic,
}: PublishBarProps) {
	const handleKeyDown = (event: { key: string }) => {
		if (event.key === "Enter") {
			handleSubmit(inputValue, topic, setInputValue);
		}
	};

	const handleChange = (event: { target: { value: string } }) => {
		setInputValue(event.target.value); // Update state on input change
	};

	return (
		<div className="flex w-full h-full">
			<input
				id="publish-bar"
				title={enabled ? "" : "Not connected to Broker"}
				disabled={!(enabled && topic)}
				onKeyDown={handleKeyDown}
				value={inputValue} // Bind the value to the state
				onChange={handleChange} // Update state on input change
				placeholder={
					enabled
						? topic === null
							? "Please select a Topic first"
							: "Type here to Publish"
						: "Connect to Broker to Publish"
				}
				className={
					"w-full mt-auto bg-transparent text-base text-[--white] border-b-[2px] border-gray50 outline-none transition-opacity duration-300 placeholder:text-gray50 focus:opacity-100 focus:border-[var(--accent)] " +
					(enabled && topic ? "" : "cursor-not-allowed")
				}
			></input>
			<PaginatedDropdown
				options={topicList}
				onChange={(topic) => {
					setTopic(topic);
				}}
			></PaginatedDropdown>
		</div>
	);
}
