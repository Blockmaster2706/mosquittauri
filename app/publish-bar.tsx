import { emit } from "@tauri-apps/api/event";
import { useState } from "react";
import { message } from "./message-view";

interface PublishBarProps {
    topic: string,
    enabled: boolean
}

export default function PublishBar({topic, enabled}: PublishBarProps) {
    const [inputValue, setInputValue] = useState('');

    const handleKeyDown = (event: { key: string }) => {
        if (event.key === "Enter") {
            handleSubmit(inputValue)
        }
    };

    const handleSubmit = (value: string) => {
        // Process the input value
        const now = new Date();
        emit<message>("newMessage", {timestamp: now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }), message: value, topic: topic})
        setInputValue(''); // Clear the input after submitting
    };

    const handleChange = (event: { target: { value: any; }; }) => {
        setInputValue(event.target.value); // Update state on input change
    };

    return (
    <div className="flex flex-col">
        <input
        title={enabled ? "" : "Not connected to Broker"}
        disabled={!enabled}
        onKeyDown={handleKeyDown}
        value={inputValue} // Bind the value to the state
        onChange={handleChange} // Update state on input change
        placeholder="Type here to Publish"
        className={"w-full mt-auto bg-transparent text-base text-[--white] border-b-[2px] border-white/50 outline-none transition-opacity duration-300 placeholder:text-white/50 focus:opacity-100 focus:border-[var(--accent)] " + (enabled ? "" : "cursor-not-allowed")}
        ></input>
    </div>
    );
}
