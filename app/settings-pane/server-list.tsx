import { useEffect } from "react";
import { Server } from "../types/server";
import { settingsButtonClassname } from "./settings-pane";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import commands from "../types/commands";

interface ServerListProps {
	serverList: Server[];
	handleClick: (id: number) => void;
	setServerList: (value: Server[]) => void;
	setServerToEdit: (value: Server) => void;
	setEditMode: () => void;
}

export default function ServerList({
	serverList,
	handleClick,
	setServerList,
	setServerToEdit,
	setEditMode,
}: ServerListProps) {
	useEffect(() => {
		const unlisten = listen("server-update", (event) => {
			const updatedServerList = event.payload;
			console.log("Received server update event:", event);
			const newServerList = updatedServerList as { list: Server[] };
			console.log("New server list:", newServerList);

			console.log("Received server update:", updatedServerList);
			setServerList(newServerList.list);
		});

		invoke(commands.get_servers);

		return () => {
			unlisten.then((f) => f());
		};
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	return (
		<div className="h-full w-full">
			<div className="w-full h-full max-h-[90%] mt-5">
				<div className="w-full mb-2">
					<button
						className={settingsButtonClassname}
						onClick={() => handleClick(1)}
					>
						Add Server
					</button>
				</div>
				<ol className="w-full h-full max-h-[55vh] break-words overflow-y-auto scrollbar-theme">
					{serverList.map((server) => {
						return (
							<li
								key={server.id}
								className="w-[calc(100%-10px)] mt-2 bg-gray60 border-gray100 border-1 grid grid-cols-10"
							>
								<button
									className="ml-1 col-span-8 text-left"
									title={server.url + ":" + server.port}
									onClick={() =>
										invoke(commands.select_server, { id: server.id })
									}
								>
									{server.name}
								</button>
								<button
									className="col-span-1 col-start-10"
									title="edit"
									onClick={() => {
										setServerToEdit(server);
										setEditMode();
									}}
								>
									X
								</button>
							</li>
						);
					})}
				</ol>
			</div>
		</div>
	);
}
