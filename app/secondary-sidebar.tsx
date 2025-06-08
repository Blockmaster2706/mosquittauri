"use client";

import { Dispatch, SetStateAction } from "react";
import {
	AutoScrollIcon,
	DocumentIcon,
	ManualScrollIcon,
	MQTTIcon,
	SendIcon,
} from "./icons";
import { topic } from "./types";
import { handleSubmit } from "./publish-bar";

interface PaneSwitcherProps {
	isShowingLogs: boolean;
	setShowingLogs: Dispatch<SetStateAction<boolean>>;
	inputValue: string;
	topic: topic | null;
	setInputValue: Dispatch<SetStateAction<string>>;
	sendButtonEnabled: boolean;
	autoScrollingDisabled: boolean;
	setAutoScrollDisabled: Dispatch<SetStateAction<boolean>>;
	errorCount: number;
	warningCount: number;
	setErrorCount: Dispatch<SetStateAction<number>>;
	setWarningCount: Dispatch<SetStateAction<number>>;
}

export default function SecondarySidebar({
	isShowingLogs,
	setShowingLogs,
	inputValue,
	topic,
	setInputValue,
	sendButtonEnabled,
	autoScrollingDisabled,
	setAutoScrollDisabled,
	errorCount,
	warningCount,
	setErrorCount,
	setWarningCount,
}: PaneSwitcherProps) {
	const button_classname =
		"mt-auto bottom-5 cursor-pointer bg-gray80 w-12 h-12 border-gray80 border-none rounded-full text-[var(--accent)] flex justify-center items-center ";

	const disabled_condition = !(sendButtonEnabled && !(topic === null));

	return (
		<div className="h-full flex flex-col">
			<button
				className={button_classname}
				onClick={() => setAutoScrollDisabled(!autoScrollingDisabled)}
				title={
					autoScrollingDisabled
						? "Enable Auto Scrolling"
						: "Disable Auto Scrolling"
				}
			>
				{autoScrollingDisabled ? (
					<ManualScrollIcon className="size-7" />
				) : (
					<AutoScrollIcon className="size-7" />
				)}
			</button>
			{isShowingLogs ? (
				<button
					className={button_classname}
					onClick={() => setShowingLogs(!isShowingLogs)}
					title="Switch to MQTT View"
				>
					<DocumentIcon className="size-7" />
				</button>
			) : (
				<button
					className={button_classname}
					onClick={() => {
						setErrorCount(0);
						setWarningCount(0);
						setShowingLogs(!isShowingLogs);
					}}
					title="Switch to Logs View"
				>
					<MQTTIcon className="size-7" />
				</button>
			)}
			{errorCount > 0 ? (
				<label className={"text-xs mt-1 text-red-500"}>
					{errorCount} Errors
				</label>
			) : warningCount > 0 ? (
				<label className={"text-xs mt-1 text-yellow-500"}>
					{warningCount} Warnings
				</label>
			) : (
				<label className={"text-xs mt-1 "}>0 Errors</label>
			)}
			<button
				disabled={disabled_condition}
				className={button_classname + (disabled_condition ? "text-gray50" : "")}
				onClick={() => handleSubmit(inputValue, topic, setInputValue)}
				title={disabled_condition ? "Please select a Topic" : "Publish Message"}
			>
				<SendIcon className="size-7" />
			</button>
		</div>
	);
}
