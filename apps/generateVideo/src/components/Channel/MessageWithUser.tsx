import {Audio} from 'remotion';
import {Img, Sequence} from 'remotion';
import {random, staticFile} from 'remotion';
import {calculatingStartingFrame, Story} from '../../Composition';
import {Delayed} from '../../hooks/Delayed';

interface IChatMessageText {
	text: string;
	userName: string;
	index: number;
	story: Story;
	date: string;
	gender: string;
	videoOutput?: {name: string; file_name: string};
	durationInFrames: number;
}

const MessageWithUser: React.FC<IChatMessageText> = ({
	text,
	userName,
	index,
	story,
	date,
	gender,
}) => {
	const colorList = [
		'#f43f5e',
		'#e11d48',
		'#be123c',
		'#9f1239',
		'#881337',
		'#ec4899',
		'#db2777',
		'#be185d',
		'#9d174d',
		'#831843',
		'#e879f9',
		'#d946ef',
		'#c026d3',
		'#a21caf',
		'#86198f',
		'#701a75',
		'#818cf8',
		'#6366f1',
		'#3b82f6',
		'#2563eb',
		'#1d4ed8',
		'#1e40af',
		'#4ade80',
		'#22c55e',
		'#16a34a',
		'#84cc16',
		'#65a30d',
		'#4d7c0f',
		'#f87171',
		'#ef4444',
		'#dc2626',
		'#b91c1c',
		'#991b1b',
		'#7f1d1d',
	];

	const randomColorNumber = Math.floor(
		random(userName + random(userName) + 5) * colorList.length,
	);

	function getRandomNumber(min: number, max: number, seed: string): number {
		return Math.floor(random(seed + random(seed)) * (max - min + 1)) + min;
	}

	return (
		<Sequence
			className="flex pt-8 pb-14 px-8 leading-[25px] min-h-40 bg-[#2F3136]"
			from={calculatingStartingFrame(story, index) + 20}
			style={{
				top: '40%',
				left: '5%',
				width: '90%',
				height: 'fit-content',
				position: 'relative',
				borderRadius: '5px',
				margin: '30px 0px 20px 0px',
			}}
		>
			<>
				<Audio
					volume={1}
					src={staticFile('/assets/sounds/discord-notification.mp3')}
				/>

				<div className="relative mt-0.5 mr-4 mb-10  w-12 min-w-fit h-12 rounded-full ">
					<Img
						style={{borderColor: colorList[randomColorNumber]}}
						className=" w-32 h-32 rounded-full z-10 border-4"
						height={250}
						width={250}
						src={staticFile(
							`/assets/images/${gender}/${gender}_${getRandomNumber(1, 14, userName + random(userName) * 2)}.webp`,
						)}
					/>
				</div>
				<div>
					<p className="flex items-center space-x-2">
						<span
							className="mr-2 font-extrabold text-4xl"
							style={{
								color: colorList[randomColorNumber],
								WebkitTextStroke: '1.5px black',
							}}
						>
							{userName}
						</span>
						<span className="text-base pt-2 font-medium text-gray-50">
							{date}
						</span>
					</p>

					<div className="max-h-fit">
						{text.split(' ').map((word: string, index: number) => {
							return (
								<Delayed waitBeforeShow={index * 160 - index}>
									<p
										style={{
											color: 'whitesmoke',
										}}
										className="text-[2rem] leading-10 font-normal float-left pr-2 text-gray-50"
									>
										{word}
									</p>
								</Delayed>
							);
						})}
						{/* <Sequence
							from={calculatingStartingFrame(story, index) + 15}
							style={{position: 'relative'}}
						>
							<Delayed waitBeforeShow={0}>
								<Video
									startFrom={calculatingStartingFrame(story, index)}
									src={staticFile(`/assets/videos/poop_video.mp4`)}
									style={{
										left: '18%',
										width: '75%',
										top: '100%',
										margin: '25px 0px 20px 0px',
									}}
								/>
							</Delayed>
						</Sequence> */}
					</div>
				</div>
			</>
		</Sequence>
	);
};

export default MessageWithUser;
