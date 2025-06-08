export type logMessage = {
	level: "info" | "warning" | "error" | "debug" | "trace";
	module?: string;
	target: string;
	timestamp: string;
	message: string;
};

export interface MessageViewProps {
	messageArray: logMessage[];
}

export default function LogsMessageView({ messageArray }: MessageViewProps) {
	return (
		<div className="w-full h-full mt-5">
			<ul>
				{messageArray.map((message, index) => {
					const topicColor = (() => {
						switch (message.level.toLowerCase()) {
							case "info":
								return "text-[var(--accent)]";
							case "warning":
								return "text-yellow-300";
							case "error":
								return "text-red-500";
							default:
								return "text-gray-500";
						}
					})();
					return (
						<li
							id={`log-message-${index}`}
							key={index}
							className="w-full bg-transparent pl-2 mt-3 break-words overflow-x-clip"
						>
							<div>
								<label
									className={
										"flex w-fit max-w-[calc(100%-2rem)] overflow-x-hidden whitespace-break-spaces max-h-8 text-l p-1 bg-gray80 rounded-t-xl m-0 border-b-2 border-b-gray80 " +
										topicColor
									}
								>
									<label className="overflow-clip overflow-ellipsis text-nowrap max-w-[calc(70vw-7rem)]">
										{message.module ? message.module : ""}
									</label>
									<label className="ml-5 text-xs mt-auto inline-flex text-gray40 min-w-fit">
										{new Date(
											parseInt(message.timestamp) * 1000,
										).toLocaleString()}
									</label>
								</label>
								<div className="rounded-b-2xl rounded-tr-2xl p-1 bg-gray80 m-0">
									{message.message}
								</div>
							</div>
						</li>
					);
				})}
			</ul>
			<div className="w-full h-30"></div>
		</div>
	);
}
