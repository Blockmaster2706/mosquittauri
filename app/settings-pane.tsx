'use client'

import { Dispatch, MouseEvent, SetStateAction, useEffect, useState } from "react";
import { topic } from "./page";

interface SettingsPageProps {
    topicList: topic[],
    setTopicList: Dispatch<SetStateAction<topic[]>>,
    address: string,
    setAddress: Dispatch<SetStateAction<string>>,
    connected: boolean,
    setConnected: Dispatch<SetStateAction<boolean>>
}

export default function SettingsPage({topicList, setTopicList, connected, setConnected, address, setAddress}: SettingsPageProps) {
    const [topic, setTopic] = useState("")

    const input_classname = "w-[calc(100%-10px)] bg-transparent text-base text-[--white] border-b-[2px] border-white/50 outline-none transition-opacity duration-300 placeholder:text-white/50 focus:opacity-100 focus:border-[var(--accent)]";

    const connected_label = connected ? <label className="text-green-500 pl-2 pb-5">Connected</label> : <label className="text-red-500 pl-2 pb-5">Disconnected</label>

    const handleKeyDown = (event: { key: string }) => {
        if (event.key === "Enter") {
            handleSubmit(topic)
        }
    };

    const handleSubmit = (value: string) => {
        // Process the input value
        if (value === "") return
        setTopicList([...topicList, {id: topicList.length, name: value, selected: false}])
        setTopic(''); // Clear the input after submitting
    };

    const handleClick = (id: number) => {
        const topicListCopy = [...topicList];
        const index = topicList.findIndex((value) => value.id === id)
        console.log(`${id} is currently ${topicListCopy[index].selected}`)
        topicListCopy[index].selected = !topicListCopy[index].selected
        setTopicList(topicListCopy)
    }

    useEffect(() => {

    },)

    return (
        <div className="w-full bg-neutral-800 h-full pt-2 pl-2 flex flex-col">
            <label className="w-full accent-text flex justify-center content-center align-middle">Mosquittauri</label>

            <label className="w-full flex pt-5">Server Address:</label>
            <input className={input_classname} type="text" title="Server Address" value={address} onChange={(event) => {setAddress(event.currentTarget.value)}}></input>
            
            <label className="w-full flex pt-5">Add new Topic:</label>
            <input className={input_classname} onKeyDown={handleKeyDown} type="text" placeholder="" title="Topic" value={topic} onChange={(event) => {setTopic(event.currentTarget.value)}}></input>

            <div className="w-full h-full max-h-[48%] mt-5">
                <ol className= "w-full h-full overflow-y-scroll scrollbar-theme break-words">
                    {topicList.map((topic) => {
                        return (
                            <li key={topic.id} className="w-full">
                                <button onClick={() => handleClick(topic.id)} className={"w-full border-1 border-black " + (topic.selected ? "bg-[var(--accent)] hover:bg-[var(--accentHover)] text-black" : "bg-neutral-600 hover:bg-neutral-800")}>{topic?.name}</button>
                            </li>
                        )
                    })}
                </ol>
            </div>

            <div className="mt-auto mb-5 w-full">

                <label className="w-full flex pt-5">Status: {connected_label}</label>
                <button title={address === "" ? "Please input a Server Address" : ""} disabled={address === ""} onClick={() => {setConnected(!connected)}} className="w-[calc(100%-20px)] cursor-pointer disabled:bg-neutral-600 disabled:text-neutral-300 disabled:border-black disabled:cursor-not-allowed disabled:bg- h-10 bg-[var(--accent)] text-black border-2 enabled:hover:bg-neutral-800 border-[var(--accent)] enabled:hover:border-[var(--accent)] enabled:hover:text-[var(--accent)] duration-100">Connect</button>
            </div>
        </div>
    )
}