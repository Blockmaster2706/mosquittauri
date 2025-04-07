'use client'

import { Dispatch, SetStateAction, useState } from "react";
import { DocumentIcon, MQTTIcon } from "./icons";

interface PaneSwitcherProps {
    isShowingLogs: boolean,
    setShowingLogs: Dispatch<SetStateAction<boolean>>
}

export default function PaneSwitcher({isShowingLogs, setShowingLogs}: PaneSwitcherProps) {

    const button_classname="mt-auto bottom-5 cursor-pointer bg-neutral-800 w-12 h-12 border-neutral-800 border-none rounded-full text-[var(--accent)] flex justify-center items-center"

    return (
        <div className="h-full flex flex-col">
            {
                isShowingLogs ?
                <button className={button_classname} onClick={() => setShowingLogs(!isShowingLogs)} title="Switch to MQTT View"><DocumentIcon className="size-7"/></button>
                : <button className={button_classname} onClick={() => setShowingLogs(!isShowingLogs)} title="Switch to Logs View"><MQTTIcon className="size-7"/></button>
            }
        </div>
    )
}