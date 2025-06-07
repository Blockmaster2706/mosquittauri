import { useEffect, useState } from "react";
import { topic } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Server } from "http";
import commands from "../types/commands";
import { settingsButtonClassname } from "./settings-pane";

interface TopicListProps {
	selected_server_id: number;
	serverName: string;
	handleClick: (topic: topic) => void;
	setAddTopicMode: () => void;
	onBackClick: () => void;
}

export default function TopicList({
	selected_server_id,
	serverName,
	handleClick,
	setAddTopicMode,
	onBackClick,
}: TopicListProps) {
	const [topicList, setTopicList] = useState<topic[]>([]);

	useEffect(() => {
		const unlisten = listen("topic-update", (event) => {
			const updatedTopicList = event.payload;
			console.log("Received topic update event:", event);
			const newTopicList = updatedTopicList as { list: topic[] };
			console.log("New topic list:", newTopicList);

			let allTopicsEnabled: boolean = true;
			for (const topic of newTopicList.list) {
				if (!topic.enabled) {
					allTopicsEnabled = false;
					break;
				}
			}
			invoke(commands.set_listen_all_topics, {
				enabled: allTopicsEnabled,
				serverId: selected_server_id,
			});

			setTopicList(newTopicList.list);
		});

		invoke(commands.get_topics);

		return () => {
			unlisten.then((f) => f());
		};
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	return (
		<div
			onMouseUpCapture={(event) => {
				console.log("Mouse up event:", event.button);
				if (event.button === 3) {
					onBackClick();
				}
			}}
		>
			<div className="w-full h-full max-h-[48%] mt-1">
				<label>
					Current Server:{" "}
					<label className="text-[var(--accent)]">{serverName}</label>
				</label>
				<hr className="border-t border-gray-300 my-2 -ml-2" />
				<button
					className={settingsButtonClassname}
					onClick={() => setAddTopicMode()}
				>
					New Topic
				</button>
				<ol className="w-full h-full overflow-y-scroll scrollbar-theme break-words">
					{topicList.map((topic) => {
						return (
							<li key={topic.id} className="w-full mt-2">
								<button
									onClick={() => handleClick(topic)}
									className={
										"w-[calc(100%-10px)] border-1 border-gray100 " +
										(topic.enabled
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
		</div>
	);
}
