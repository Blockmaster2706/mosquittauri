'use client'

import { useState } from "react";
import { DocumentIcon, MQTTIcon } from "./icons";

export default function PaneSwitcher() {
    const [isShowingLogs, setShowingLogs] = useState(false)

    const button_classname="mt-auto bottom-5 cursor-pointer bg-neutral-800 w-12 h-12 border-neutral-800 border-none rounded-full text-[var(--accent)] flex justify-center items-center"

    return (
        <div className="h-full flex flex-col">
            {
                isShowingLogs ?
                <button className={button_classname} onClick={() => setShowingLogs(!isShowingLogs)} title="Switch to Logs View"><MQTTIcon className="size-7"/></button>
                : <button className={button_classname} onClick={() => setShowingLogs(!isShowingLogs)} title="Switch to MQTT View"><DocumentIcon className="size-7"/></button>
            }
        </div>
    )
}