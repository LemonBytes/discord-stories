import {Audio, Sequence} from 'remotion';
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

export const Server: React.FC<Story> = (story) => {
	const date = new Date('2024-08-17');

	const options: Intl.DateTimeFormatOptions = {
		weekday: 'long',
		day: '2-digit',
		month: 'long',
		year: 'numeric',
	};
	const formattedDate = date.toLocaleDateString('en-GB', options);

	return (
		<AbsoluteFill
			style={{position: 'relative', width: '80%', margin: '10%', padding: '2%'}}
		>
			<div className="flex flex-col flex-auto shrink min-w-0 bg-[#2F3136] max-h-[175vh] bg-opacity-90 overflow-hidden">
				<ChannelTopbar />

				{/* Container for the messages */}
				<div className="flex-1 flex flex-col  overflow-y-hidden justify-end">
					<div className=" w-full h-20 flex flex-row items-center justify-between my-10">
						<div className="border-1 border border-gray-300  w-[30%]" />
						<p className="text-gray-300 text-2xl  mx-15 text-center">
							{formattedDate}
						</p>
						<div className="border-1 border w-[30%] border-gray-300 " />
					</div>

					{story.fragments?.map((fragment: StoryFragment, index: number) => {
						return (
							<div key={fragment.hashedText}>
								<Sequence
									key={fragment.hashedText}
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
										volume={0.7}
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
								/>
							</div>
						);
					})}
				</div>
			</div>
		</AbsoluteFill>
	);
};
