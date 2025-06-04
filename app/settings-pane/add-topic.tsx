interface AddTopicProps {
	input_classname: string;
	topic: string;
	setTopic: (value: string) => void;
	handleKeyDown: (event: { key: string }) => void;
}

export default function AddTopic({
	input_classname,
	topic,
	setTopic,
	handleKeyDown,
}: AddTopicProps) {
	return (
		<div>
			<label className="w-full flex pt-5 text-gray20">Add new Topic:</label>
			<input
				className={input_classname}
				onKeyDown={handleKeyDown}
				type="text"
				placeholder=""
				title="Topic"
				value={topic}
				onChange={(event) => {
					setTopic(event.currentTarget.value);
				}}
			></input>
		</div>
	);
}
