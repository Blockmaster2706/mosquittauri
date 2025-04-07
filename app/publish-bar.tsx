export default function PublishBar() {
    return (
        <div className="flex flex-col">
            <input placeholder="Type here to Publish" className="w-full mt-auto bg-transparent text-base text-[--white] border-b-[2px] border-white/50 outline-none transition-opacity duration-300 placeholder:text-white/50 focus:opacity-100 focus:border-[var(--accent)]"></input>
        </div>
    )
}