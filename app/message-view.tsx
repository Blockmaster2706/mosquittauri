export type message = {
	timestamp: string;
	message: string;
	topic: string;
};

export interface MessageViewProps {
	messageArray: message[];
}

export default function MessageView({ messageArray }: MessageViewProps) {
	return (
		<div className="w-full h-full mt-5">
			<ul>
				{messageArray.map((message, index) => {
					return (
						<li
							id={`message-${index}`}
							key={index}
							className="w-full bg-transparent pl-2 mt-3 break-all overflow-x-clip"
						>
							<div>
								<label className="flex w-fit max-w-[calc(100%-2rem)] overflow-x-hidden whitespace-break-spaces max-h-8 text-l text-[var(--accent)] p-1 bg-gray80 rounded-t-xl m-0 border-b-2 border-b-gray80">
									<label className="overflow-clip overflow-ellipsis text-nowrap max-w-[calc(70vw-7rem)]">
										{message.topic}
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
