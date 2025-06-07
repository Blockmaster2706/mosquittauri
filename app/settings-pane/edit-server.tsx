import { useEffect, useState } from "react";
import { settingsButtonClassname } from "./settings-pane";
import { Server } from "../types/server";

interface EditServerProps {
	input_classname: string;
	onBackClick: () => void;
	onAddClick: (server: Server) => void;
	server: Server;
}

export default function EditServer({
	input_classname,
	onBackClick,
	onAddClick,
	server,
}: EditServerProps) {
	const [serverName, setServerName] = useState(server.name);
	const [serverAddress, setServerAddress] = useState(server.url);
	const [serverPort, setServerPort] = useState(server.port.toString());
	const [serverClientId, setServerClientId] = useState(server.clientId);

	useEffect(() => {
		setServerName(server.name);
		setServerAddress(server.url);
		setServerPort(server.port.toString());
		setServerClientId(server.clientId);
	}, [server]);

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
			<label className="w-full flex pt-5 text-gray20">Editing Server:</label>
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
					onAddClick({
						name: serverName,
						url: serverAddress,
						port: parseInt(serverPort),
						clientId: serverClientId,
						id: server.id,
					})
				}
			>
				{" "}
				Save Server
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
