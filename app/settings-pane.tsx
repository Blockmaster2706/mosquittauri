export default function SettingsPage() {
    const input_classname = "w-[calc(100%-10px)] bg-transparent text-base text-[--white] border-b-[2px] border-white/50 outline-none transition-opacity duration-300 placeholder:text-white/50 focus:opacity-100 focus:border-[var(--accent)]";
    
    return (
        <div className="absolute left-0 top-0 bg-neutral-800 h-full w-[15%] pt-2 pl-2">
            <label className="w-full accent-text flex justify-center content-center align-middle">Mosquittauri</label>

            <label className="w-full flex pt-5">Server Address:</label>
            <input className={input_classname} type="text" placeholder="localhost" title="Server Address"></input>
            
            <label className="w-full flex pt-5">Topic:</label>
            <input className={input_classname} type="text" placeholder="" title="Topic"></input>

            <div className="absolute bottom-5 w-full">

                <label className="w-full flex pt-5">Status: <label className="text-green-500 pl-2 pb-5">Connected</label></label>
                <button className="w-[calc(100%-20px)] h-10 bg-[var(--accent)] text-black border-2 hover:bg-neutral-800 hover:border-[var(--accent)] hover:text-[var(--accent)] duration-100">Connect</button>
            </div>
        </div>
    )
}