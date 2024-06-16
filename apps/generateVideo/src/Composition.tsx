/* eslint-disable @typescript-eslint/no-unused-vars */
import {AbsoluteFill, Sequence, staticFile, Audio, Video} from 'remotion';
import {ChatMessageBanner} from './components/ChatMessageBanner';
import React from 'react';
import {TitleBanner} from './components/TitleBanner';

export type ISpeaker = {
	hashedText: string;
	speakerType: 'main' | 'sub_text' | 'sub_voice' | 'title';
	audioDurationInFrames: number;
	text: string;
	userName: string;
};

type Story = {
	story: ISpeaker[];
};

const calculatingStartingFrame = (
	speakers: ISpeaker[],
	index: number,
): number => {
	let startingFrame = 0;
	if (index === 0) {
		return startingFrame;
	}

	for (let i = 1; i <= index; i++) {
		startingFrame += speakers[i - 1].audioDurationInFrames;
	}

	return startingFrame;
};

export const MyComposition: React.FC<Story> = ({story}) => {
	return (
		<>
			<Video
				muted
				src={staticFile('/assets/background_videos/scary_background.mp4')}
				style={{height: 1920, width: 1080}}
			/>

			{story.map((speaker: ISpeaker, index: number) => {
				return (
					<>
						<Sequence
							key={speaker.hashedText}
							durationInFrames={speaker.audioDurationInFrames}
							from={calculatingStartingFrame(story, index)}
						>
							<Audio
								src={staticFile(
									'/temp_assets/story_audio/' + speaker.hashedText + '.mp3',
								)}
							/>
						</Sequence>
						<AbsoluteFill>
							{speaker.speakerType === 'title' && (
								<TitleBanner
									delay={calculatingStartingFrame(story, index) - 5}
									userName={speaker.userName}
									text={speaker.text}
									duruation={speaker.audioDurationInFrames}
								/>
							)}
						</AbsoluteFill>
						<AbsoluteFill>
							{speaker.speakerType === 'sub_text' && (
								<ChatMessageBanner
									delay={calculatingStartingFrame(story, index) - 5}
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
	);
};
