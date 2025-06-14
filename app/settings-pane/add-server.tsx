import { useState } from "react";
import { settingsButtonClassname } from "./settings-pane";

interface AddServerProps {
	input_classname: string;
	onBackClick: () => void;
	onAddClick: (name: string, url: string, port: number) => void;
}

export default function AddServer({
	input_classname,
	onBackClick,
	onAddClick,
}: AddServerProps) {
	const [serverName, setServerName] = useState("");
	const [serverAddress, setServerAddress] = useState("");
	const [serverPort, setServerPort] = useState("1883");

	return (
		<div
			className="h-full w-full"
			onMouseUpCapture={(event) => {
				console.log("Mouse up event:", event.button);
				if (event.button === 3) {
					onBackClick();
				}
			}}
		>
			<label className="w-full flex pt-5 text-gray20">Adding Server:</label>
			<label className="w-full flex pt-5 text-gray20">Name:</label>
			<input
				className={input_classname}
				type="text"
				title="Name for the Server"
				value={serverName}
				onChange={(event) => {
					setServerName(event.currentTarget.value);
				}}
			></input>
			<label className="w-full flex pt-5 text-gray20">Address:</label>
			<input
				className={input_classname}
				type="text"
				title="Address of the Server"
				value={serverAddress}
				onChange={(event) => {
					setServerAddress(event.currentTarget.value);
				}}
			></input>
			<label className="w-full flex pt-5 text-gray20">Port:</label>
			<input
				className={input_classname + " placeholder:text-black"}
				type="number"
				min="1"
				max="65535"
				placeholder="1883"
				title="Port of the Server (1-65535)"
				value={serverPort === "1883" ? "" : serverPort}
				onChange={(event) => {
					setServerPort(event.currentTarget.value);
				}}
			></input>
			<button
				className={settingsButtonClassname + " mt-5"}
				onClick={() =>
					onAddClick(serverName, serverAddress, parseInt(serverPort))
				}
			>
				{" "}
				Add Server
			</button>
			<button
				className={settingsButtonClassname + " mt-5"}
				onClick={() => onBackClick()}
			>
				Back
			</button>
		</div>
	);
}
