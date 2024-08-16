import {useCallback, useEffect, useState} from 'react';
import {
	AbsoluteFill,
	cancelRender,
	continueRender,
	delayRender,
	getStaticFiles,
	OffthreadVideo,
	Sequence,
	staticFile,
	useCurrentFrame,
	useVideoConfig,
	watchStaticFile,
} from 'remotion';

import Subtitle from './Subtitle';
import {getVideoMetadata} from '@remotion/media-utils';
import {loadFont} from '../load-font';
import {NoCaptionFile} from './NoCaptionFile';

export type SubtitleProp = {
	startInSeconds: number;
	text: string;
};

export const calculateCaptionedVideoMetadata = async () => {
	const fps = 30;
	const src = staticFile('/temp_assets/temp/uncaptioned_story.mp4');
	const metadata = await getVideoMetadata(src);

	return {
		fps,
		durationInFrames: Math.floor(metadata.durationInSeconds * fps),
	};
};

const getFileExists = (file: string) => {
	const files = getStaticFiles();
	const fileExists = files.find((f) => {
		return f.src === file;
	});
	return Boolean(fileExists);
};

type CaptionedVideoProps = {
	src: string;
	story: Story;
};

const SKIP_FRAMES_FOR_NOTIFICATION = 20;

export const CaptionedVideo: React.FC<CaptionedVideoProps> = ({src, story}) => {
	const [subtitles, setSubtitles] = useState<SubtitleProp[]>([]);
	const [showSubtitles, setShowSubtitles] = useState(false);

	const [handle] = useState(() => delayRender());
	const frame = useCurrentFrame(); // 25
	const {fps} = useVideoConfig();

	const getIndexOfFragment = (frame: number) => {
		if (
			!(
				frame >= 0 &&
				frame <=
					story.story[0].audioDurationInFrames + SKIP_FRAMES_FOR_NOTIFICATION
			)
		) {
			setShowSubtitles(true);
		}
	};

	const subtitlesFile = src
		.replace(/.mp4$/, '.json')
		.replace(/.mkv$/, '.json')
		.replace(/.mov$/, '.json')
		.replace(/.webm$/, '.json');

	const fetchSubtitles = useCallback(async () => {
		try {
			await loadFont();
			const res = await fetch(subtitlesFile);
			const data = await res.json();
			setSubtitles(data.transcription);
			continueRender(handle);
		} catch (e) {
			cancelRender(e);
		}
	}, [handle, subtitlesFile]);

	useEffect(() => {
		fetchSubtitles();

		const c = watchStaticFile(subtitlesFile, () => {
			fetchSubtitles();
		});

		return () => {
			c.cancel();
		};
	}, [fetchSubtitles, src, subtitlesFile]);

	return (
		<>
			{showSubtitles ? (
				<AbsoluteFill style={{backgroundColor: 'transparent'}}>
					<AbsoluteFill>
						<OffthreadVideo
							style={{
								objectFit: 'cover',
							}}
							src={src}
						/>
					</AbsoluteFill>
					{subtitles.map((subtitle, index) => {
						if (!showSubtitles) {
							getIndexOfFragment(frame);
						}

						const nextSubtitle = subtitles[index + 1] ?? null;
						const subtitleStartFrame = subtitle.startInSeconds * fps;
						const subtitleEndFrame = Math.min(
							nextSubtitle ? nextSubtitle.startInSeconds * fps : Infinity,
							subtitleStartFrame + fps,
						);
						const durationInFrames = subtitleEndFrame - subtitleStartFrame;
						if (durationInFrames <= 0) {
							return null;
						}

						return (
							<>
								<Sequence
									key={index}
									from={subtitleStartFrame}
									durationInFrames={durationInFrames}
								>
									<Subtitle key={index} text={subtitle.text} />
								</Sequence>
							</>
						);
					})}
					{getFileExists(subtitlesFile) ? null : <NoCaptionFile />}
				</AbsoluteFill>
			) : (
				<AbsoluteFill>
					<OffthreadVideo
						style={{
							objectFit: 'cover',
						}}
						src={src}
					/>
				</AbsoluteFill>
			)}
		</>
	);
};
