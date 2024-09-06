/* eslint-disable react/jsx-curly-brace-presence */
import {Audio, random, Sequence} from 'remotion';
import {staticFile} from 'remotion';
import {AbsoluteFill} from 'remotion';
import {
	calculatingStartingFrame,
	SKIP_FRAMES_FOR_NOTIFICATION,
	Story,
	StoryFragment,
} from '../../Composition';
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

export const generateRandomDateForMessage = () => {
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
				width: '75%',
				margin: '12.5%',
				marginRight: '20%',
				maxHeight: '60%',
				marginTop: '18%',
				borderRadius: '30px',
			}}
		>
			<div className="flex flex-col flex-auto shrink min-w-0 bg-opacity-0  overflow-hidden">
				{/* <ChannelTopbar randomSeed={story.fragments[0].userName} /> */}

				<div className="flex-1 flex flex-col overflow-y-hidden justify-end">
					<div className="pb-96">
						{story.fragments?.map((fragment: StoryFragment, index: number) => {
							const messageDate = generateRandomDateForMessage(); // Generate a random date

							return (
								<div key={fragment.speakerOrder}>
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
													fragment.speakerOrder +
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
										date={messageDate}
										durationInFrames={fragment.audioDurationInFrames}
										// Pass the generated date to MessageWithUser
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
