export interface message {
    timestamp: string;
    message: string;
    topic: string;
}

export default function MessageView() {
    let messageArray = [
        {timestamp: "8am", message: "Dies ist ein Test", topic: "test"},
        {timestamp: "9:30am", message: "Tauri ist Cool", topic: "test"},
        {timestamp: "10:21am", message: "Mein Mitazubi steht richtig hart auf Matjes oder wie man das schreibt", topic: "Fischbrötchen"},
        {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
        {timestamp: "11:11am", message: "Dies ist ein *weiterer* Test", topic: "test"},
        {timestamp: "69am", message: "Lol funny number", topic: "hehe"},
        {timestamp: "420am", message: "Wann gratis Brokkoli für alle, Herr Habeck?", topic: "Brokkoli"},
        {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
        {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
        {timestamp: "11am", message: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.", topic: "LoremIpsum"},
    ]
    
    return (
        <div className="w-full h-full mt-5">
            <ul>
                {messageArray.map((message, index) => {
                    return (
                        <li key={index} className="w-full bg-neutral-800 rounded-2xl pl-2 mt-3">
                            {`[${message.topic}] at ${message.timestamp}: ${message.message}`}
                        </li>
                    )
                })}
            </ul>
        </div>
    )
}