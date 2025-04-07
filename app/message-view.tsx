export type message = {
    timestamp: string;
    message: string;
    topic: string;
}

export interface MessageViewProps {
    messageArray: message[]
}

export default function MessageView({messageArray}: MessageViewProps) {
    return (
        <div className="w-full h-full mt-5">
            <ul>
                {messageArray.map((message, index) => {
                    return (
                        <li key={index} className="w-full bg-neutral-800 rounded-2xl pl-2 mt-3 break-words">
                            {`[${message.topic}] at ${message.timestamp}: ${message.message}`}
                        </li>
                    )
                })}
            </ul>
            <div className="w-full h-30"></div>
        </div>
    )
}