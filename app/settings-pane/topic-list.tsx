import { topic } from "../types";

interface TopicListProps {
	topicList: topic[];
	handleClick: (id: number) => void;
}

export default function TopicList({ topicList, handleClick }: TopicListProps) {
	return (
		<div>
			<div className="w-full h-full max-h-[48%] mt-5">
				<ol className="w-full h-full overflow-y-scroll scrollbar-theme break-words">
					{topicList.map((topic) => {
						return (
							<li key={topic.id} className="w-full">
								<button
									onClick={() => handleClick(topic.id)}
									className={
										"w-full border-1 border-gray100 " +
										(topic.selected
											? "bg-accent hover:bg-accentHover text-gray100"
											: "bg-gray60 hover:bg-gray80")
									}
								>
									{topic?.name}
								</button>
							</li>
						);
					})}
				</ol>
			</div>
		</div>
	);
}
