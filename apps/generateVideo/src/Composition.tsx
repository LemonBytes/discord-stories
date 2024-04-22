import {AbsoluteFill, Sequence, staticFile, Audio, Video} from 'remotion';

import {z} from 'zod';
import {zColor} from '@remotion/zod-types';
import {ChatMessage} from './components/ChatMessage';
import {CaptionedVideo} from './CaptionedVideo';

export const myCompSchema = z.object({
	titleText: z.string(),
	titleColor: zColor(),
	logoColor: zColor(),
});

export const MyComposition: React.FC<z.infer<typeof myCompSchema>> = () => {
	return (
		<>
			<AbsoluteFill>
				<Video
					muted
					src={staticFile('/background_videos/scary_background.mp4')}
					style={{height: 1920, width: 1080}}
				/>
				<ChatMessage delay={30} />
				<CaptionedVideo />
			</AbsoluteFill>
			<Sequence>
				<Audio src={staticFile('/video_1/audio_script.mp3')} />
			</Sequence>
		</>
	);
};
