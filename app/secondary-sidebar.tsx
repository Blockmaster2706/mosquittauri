'use client'

import { Dispatch, SetStateAction } from "react";
import { DocumentIcon, MQTTIcon, SendIcon } from "./icons";
import { topic } from "./types";
import { handleSubmit } from "./publish-bar";

interface PaneSwitcherProps {
    isShowingLogs: boolean,
    setShowingLogs: Dispatch<SetStateAction<boolean>>,
    inputValue: string,
    topic: topic | null
    setInputValue: Dispatch<SetStateAction<string>>,
    sendButtonEnabled: boolean
}

export default function SecondarySidebar({isShowingLogs, setShowingLogs, inputValue, topic, setInputValue, sendButtonEnabled}: PaneSwitcherProps) {

    const button_classname="mt-auto bottom-5 cursor-pointer bg-neutral-800 w-12 h-12 border-neutral-800 border-none rounded-full text-[var(--accent)] flex justify-center items-center "

    const disabled_condition = !(sendButtonEnabled && !(topic === null))

    return (
        <div className="h-full flex flex-col">
            {
                isShowingLogs ?
                <button className={button_classname} onClick={() => setShowingLogs(!isShowingLogs)} title="Switch to MQTT View"><DocumentIcon className="size-7"/></button>
                : <button className={button_classname} onClick={() => setShowingLogs(!isShowingLogs)} title="Switch to Logs View"><MQTTIcon className="size-7"/></button>
            }
            <button disabled={disabled_condition} className={button_classname + (disabled_condition ? "text-neutral-500" : "")} onClick={() => handleSubmit(inputValue, topic, setInputValue)} title={disabled_condition ? "Please select a Topic" : "Publish Message"}><SendIcon className="size-7"/></button>
        </div>
    )
}