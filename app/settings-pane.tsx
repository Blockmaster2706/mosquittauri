'use client'

import { Dispatch, SetStateAction } from "react";

interface SettingsPageProps {
    topic: string,
    setTopic: Dispatch<SetStateAction<string>>,
    connected: boolean,
    setConnected: Dispatch<SetStateAction<boolean>>
}

export default function SettingsPage({topic, setTopic, connected, setConnected}: SettingsPageProps) {
    const input_classname = "w-[calc(100%-10px)] bg-transparent text-base text-[--white] border-b-[2px] border-white/50 outline-none transition-opacity duration-300 placeholder:text-white/50 focus:opacity-100 focus:border-[var(--accent)]";

    const connected_label = connected ? <label className="text-green-500 pl-2 pb-5">Connected</label> : <label className="text-red-500 pl-2 pb-5">Disconnected</label>

    return (
        <div className="w-full bg-neutral-800 h-full pt-2 pl-2 flex flex-col">
            <label className="w-full accent-text flex justify-center content-center align-middle">Mosquittauri</label>

            <label className="w-full flex pt-5">Server Address:</label>
            <input className={input_classname} type="text" placeholder="localhost" title="Server Address"></input>
            
            <label className="w-full flex pt-5">Topic:</label>
            <input className={input_classname} type="text" placeholder="" title="Topic" value={topic} onChange={(event) => {setTopic(event.currentTarget.value)}}></input>

            <div className="mt-auto mb-5 w-full">

                <label className="w-full flex pt-5">Status: {connected_label}</label>
                <button onClick={() => {setConnected(!connected)}} className="w-[calc(100%-20px)] cursor-pointer h-10 bg-[var(--accent)] text-black border-2 hover:bg-neutral-800 border-[var(--accent)] hover:border-[var(--accent)] hover:text-[var(--accent)] duration-100">Connect</button>
            </div>
        </div>
    )
}