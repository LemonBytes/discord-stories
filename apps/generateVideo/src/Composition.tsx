/* eslint-disable react/no-unused-prop-types */
/* eslint-disable @typescript-eslint/no-unused-vars */
import {AbsoluteFill, Sequence, staticFile, Audio, Video} from 'remotion';
import {ChatMessageBanner} from './components/ChatMessageBanner';
import React from 'react';
import {TitleBanner} from './components/TitleBanner';

export type StoryFragment = {
	voiceName: string;
	userName: string;
	text: string;
	audioDurationInFrames: number;
	hashedText: string;
	speakerType: 'main' | 'sub_text' | 'sub_voice' | 'title';
	gender: string;
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

// TO DO
// create general file path
// dynamic background video

export const MyComposition: React.FC<Story> = (story) => {
	const SKIP_FRAMES_FOR_NOTIFICATION = 20;
	console.log(story.storyType);
	return (
		<>
			{story.storyType === 'comments' && (
				<>
					<Audio
						volume={1}
						src={staticFile(`/assets/sounds/discord-notification.mp3`)}
					/>
					<Audio
						loop
						volume={0.15}
						src={staticFile(
							`/assets/background_sounds/horrific_background_sound.mp3`,
						)}
					/>
					<Video
						muted
						loop
						startFrom={SKIP_FRAMES_FOR_NOTIFICATION}
						src={staticFile(
							'/assets/background_videos/crazy_background_video.mp4',
						)}
						style={{height: 1920, width: 1080}}
					/>

					{story.fragments.map((speaker: StoryFragment, index: number) => {
						return (
							<>
								<Sequence
									key={speaker.hashedText}
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
										volume={0.55}
										src={staticFile(
											'/temp_assets/story_audio/' + speaker.hashedText + '.mp3',
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
	);
};
