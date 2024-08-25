import {Audio, random, Sequence} from 'remotion';
import {staticFile} from 'remotion';
import {AbsoluteFill} from 'remotion';
import {
	calculatingStartingFrame,
	SKIP_FRAMES_FOR_NOTIFICATION,
	Story,
	StoryFragment,
} from '../../Composition';
import {ChannelTopbar} from './ChannelHeader';
import MessageWithUser from './MessageWithUser';

const getRandomDate = (startDate: Date, endDate: Date): Date => {
	const start = startDate.getTime();
	const end = endDate.getTime();
	const randomTimestamp = start + random(startDate.toString()) * (end - start);
	return new Date(randomTimestamp);
};

const formatDate = (date: Date): string => {
	const options: Intl.DateTimeFormatOptions = {
		weekday: 'long',
		day: '2-digit',
		month: 'long',
		year: 'numeric',
	};
	return date.toLocaleDateString('en-GB', options);
};

const generateRandomDateForMessage = () => {
	const now = new Date();
	const startDate = new Date(
		now.getFullYear(),
		now.getMonth() - 1,
		now.getDate(),
	); // Start from a month ago
	const endDate = now; // End at the current date
	const randomDate = getRandomDate(startDate, endDate);
	return formatDate(randomDate);
};

export const Server: React.FC<Story> = (story) => {
	return (
		<AbsoluteFill
			style={{
				position: 'relative',
				width: '80%',
				margin: '10%',
				padding: '2%',
				maxHeight: '70%',
				marginTop: '20%',
				borderRadius: '30px',
			}}
		>
			<div className="flex flex-col flex-auto shrink min-w-0 bg-[#2F3136] bg-opacity-30  overflow-hidden rounded-lg border-[1.5px] border-green-100">
				<ChannelTopbar randomSeed={story.fragments[4].userName} />

				<div className="flex-1 flex flex-col  overflow-y-hidden justify-end">
					<div className=" w-full h-20 flex flex-row items-center justify-between my-10">
						<div className="border-1 border border-gray-300  w-[30%]" />
						<p className="text-gray-300 text-2xl  mx-15 text-center">
							{generateRandomDateForMessage()}
						</p>
						<div className="border-1 border w-[30%] border-gray-300 " />
					</div>
					<div className="pb-36">
						{story.fragments?.map((fragment: StoryFragment, index: number) => {
							const messageDate = generateRandomDateForMessage(); // Generate a random date

							return (
								<div key={fragment.hashedText}>
									<Sequence
										durationInFrames={
											fragment.audioDurationInFrames +
											SKIP_FRAMES_FOR_NOTIFICATION * 2
										}
										from={
											calculatingStartingFrame(story, index) +
											SKIP_FRAMES_FOR_NOTIFICATION
										}
									>
										<Audio
											volume={1}
											src={staticFile(
												'/temp_assets/story_audio/' +
													fragment.hashedText +
													'.mp3',
											)}
										/>
									</Sequence>

									<MessageWithUser
										text={fragment.text}
										userName={fragment.userName}
										index={index}
										story={story}
										gender={fragment.gender}
										date={messageDate} // Pass the generated date to MessageWithUser
									/>
								</div>
							);
						})}
					</div>
				</div>
			</div>
		</AbsoluteFill>
	);
};
