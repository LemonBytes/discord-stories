/* eslint-disable react/no-unused-prop-types */
/* eslint-disable @typescript-eslint/no-unused-vars */
import {
	AbsoluteFill,
	Sequence,
	staticFile,
	Audio,
	Video,
	random,
} from 'remotion';
import {ChatMessageBanner} from './components/ChatMessageBanner';
import React from 'react';
import {TitleBanner} from './components/TitleBanner';
import {Server} from './components/Channel/Server';
import {Gif} from '@remotion/gif';

export type StoryFragment = {
	voiceName: string;
	userName: string;
	text: string;
	audioDurationInFrames: number;
	speakerOrder: number;
	speakerType: 'main' | 'sub_text' | 'sub_voice' | 'title';
	gender: string;
	videoOutput: string;
};

export type Story = {
	storyType: string;
	genre: string;
	fragments: StoryFragment[];
};

export const calculatingStartingFrame = (
	story: Story,
	index: number,
): number => {
	let startingFrame = 0;
	if (index === 0) {
		return startingFrame;
	}

	for (let i = 1; i <= index; i++) {
		startingFrame += story.fragments[i - 1].audioDurationInFrames;
	}

	return startingFrame;
};

function getRandomNumber(min: number, max: number, seed: string): number {
	return Math.floor(random(seed + random(seed)) * (max - min + 1)) + min;
}

// TO DO
// create general file path
// dynamic background video
export const SKIP_FRAMES_FOR_NOTIFICATION = 20;
export const MyComposition: React.FC<Story> = (story) => {
	return (
		<>
			<>
				{story.storyType === 'narrator' && (
					<Audio
						volume={1}
						src={staticFile(`/assets/sounds/discord-join.mp3`)}
					/>
				)}
				<Audio
					loop
					volume={0.2}
					src={staticFile(
						`/assets/background_sounds/background_sound_${getRandomNumber(1, 4, story.fragments[2].userName)}.mp3`,
					)}
				/>
				<Video
					muted
					loop
					startFrom={SKIP_FRAMES_FOR_NOTIFICATION}
					src={staticFile(
						`/assets/background_videos/background_video_${getRandomNumber(1, 12, story.fragments[0].voiceName)}.mp4`,
					)}
					style={{height: 1920, width: 1080}}
				/>
				{story.storyType === 'chat' && (
					<AbsoluteFill
						style={{
							display: 'flex',
							justifyItems: 'center',
						}}
					>
						<Server {...story} />
						<Gif
							style={{
								width: '70px',
								height: '70px',
								position: 'absolute',
								right: '65px',
								top: '59%',
							}}
							src="https://media.tenor.com/dJsk1OmZolMAAAAi/super-bad-mario-kick.gif"
						/>
						<Gif
							style={{
								width: '70px',
								height: '70px',
								position: 'absolute',
								right: '65px',
								top: '69%',
							}}
							src="https://media.tenor.com/aE2yaV9JjUgAAAAi/super-bad-mario-dancing.gif"
						/>
					</AbsoluteFill>
				)}
				{story.storyType === 'narrator' && (
					<>
						{story.fragments.map((speaker: StoryFragment, index: number) => {
							return (
								<>
									<Sequence
										key={speaker.speakerOrder}
										durationInFrames={
											speaker.audioDurationInFrames +
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
													speaker.speakerOrder +
													'.mp3',
											)}
										/>
									</Sequence>
									<AbsoluteFill>
										{speaker.speakerType === 'title' && (
											<TitleBanner
												delay={
													calculatingStartingFrame(story, index) +
													SKIP_FRAMES_FOR_NOTIFICATION
												}
												gender={speaker.gender}
												userName={speaker.userName}
												text={speaker.text}
												duruation={speaker.audioDurationInFrames}
											/>
										)}
									</AbsoluteFill>
									<AbsoluteFill>
										{speaker.speakerType === 'sub_text' && (
											<ChatMessageBanner
												delay={
													calculatingStartingFrame(story, index) +
													SKIP_FRAMES_FOR_NOTIFICATION
												}
												userName={speaker.userName}
												text={speaker.text}
												duruation={speaker.audioDurationInFrames}
											/>
										)}
									</AbsoluteFill>
								</>
							);
						})}
					</>
				)}
			</>
		</>
	);
};
