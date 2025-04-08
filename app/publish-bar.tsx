'use client'

import { emit } from "@tauri-apps/api/event";
import { useState } from "react";
import { message } from "./message-view";
import { topic } from "./types";
import PaginatedDropdown from "./paginated-dropdown";

interface PublishBarProps {
    topicList: topic[],
    enabled: boolean
}

export default function PublishBar({topicList, enabled}: PublishBarProps) {
    const [inputValue, setInputValue] = useState('');
    const [topic, setTopic] = useState<topic | null>(null)

    const handleKeyDown = (event: { key: string }) => {
        if (event.key === "Enter") {
            handleSubmit(inputValue)
        }
    };

    const handleSubmit = (value: string) => {
        // Process the input value
        const now = new Date();
        emit<message>("newMessage", {timestamp: now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }), message: value, topic: topic?.name ?? ""})
        setInputValue(''); // Clear the input after submitting
    };

    const handleChange = (event: { target: { value: string; }; }) => {
        setInputValue(event.target.value); // Update state on input change
    };

    return (
    <div className="flex">
        <input
        title={enabled ? "" : "Not connected to Broker"}
        disabled={!(enabled && topic)}
        onKeyDown={handleKeyDown}
        value={inputValue} // Bind the value to the state
        onChange={handleChange} // Update state on input change
        placeholder={enabled ? topic === null ? "Please select a Topic first" : "Type here to Publish" : "Connect to Broker to Publish"}
        className={"w-full mt-auto bg-transparent text-base text-[--white] border-b-[2px] border-neutral-500 outline-none transition-opacity duration-300 placeholder:text-neutral-500 focus:opacity-100 focus:border-[var(--accent)] " + ((enabled && topic) ? "" : "cursor-not-allowed")}
        ></input>
        <PaginatedDropdown options={topicList} onChange={(topic) => {setTopic(topic)}}></PaginatedDropdown>
    </div>
    );
}
