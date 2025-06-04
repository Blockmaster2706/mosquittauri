import { useEffect } from "react";

export type message = {
	timestamp: string;
	message: string;
	topic: string;
};

export interface MessageViewProps {
	messageArray: message[];
}

export default function LogsMessageView() {
	const messageArray = [
			{ timestamp: "8am", message: "Dies ist ein Logs Test", topic: "test", logLevel: "info" },
			{ timestamp: "9:30am", message: "Logs sind Cool", topic: "test", logLevel: "info" },
			{
				timestamp: "10:21am",
				message:
					"Mein Mitazubi steht richtig hart auf Baumstämme oder wie man das schreibt",
				topic: "Holzbrötchen",
				logLevel: "info"
			},
			{
				timestamp: "11am",
				message:
					"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
				topic: "LoremIpsum",
				logLevel: "error"
			},
			{
				timestamp: "11:11am",
				message: "Dies ist ein *weiterer* Logs Test",
				topic: "test",
				logLevel: "warning"
			},
			{ timestamp: "69am", message: "Lol funny Logs", topic: "hehe", logLevel: "info" },
			{
				timestamp: "420am",
				message: "Wann gratis Logs für alle, Herr Habeck?",
				topic: "Brokkologs",
				logLevel: "info",
			},
			{
				timestamp: "11am",
				message:
					"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
				topic: "LoremIpsum",
				logLevel: "error",
			},
			{
				timestamp: "11am",
				message:
					"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
				topic: "LoremIpsum",
				logLevel: "info",
			},
			{
				timestamp: "11am",
				message:
					"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
				topic: "LoremIpsum",
				logLevel: "warning",
			},
			{
				timestamp: "11am",
				message:
					"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
				topic: "LoremIpsum",
				logLevel: "info",
			},
			{
				timestamp: "11am",
				message:
					"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
				topic: "LoremIpsum",
				logLevel: "info",
			},
		];	

	useEffect(() => {
		
	}, []);

	return (
		<div className="w-full h-full mt-5">
			<ul>
				{messageArray.map((message, index) => {
					const topicColor = message.logLevel === "info" ? "text-[var(--accent)]" : message.logLevel === "warning" ? "text-yellow-300" : message.logLevel === "error" ? "text-red-500" : "text-gray40";
					return (
						<li
							key={index}
							className="w-full bg-transparent pl-2 mt-3 break-all overflow-x-clip"
						>
							<div>
								<label className={"flex w-fit max-w-[calc(100%-2rem)] overflow-x-hidden whitespace-break-spaces max-h-8 text-l p-1 bg-gray80 rounded-t-xl m-0 border-b-2 border-b-gray80 " + topicColor}>
									<label className="overflow-clip overflow-ellipsis text-nowrap max-w-[calc(70vw-7rem)]">
										{message.topic}
									</label>
									<label className="ml-5 text-xs mt-auto inline-flex text-gray40 min-w-fit">
										{message.timestamp}
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
