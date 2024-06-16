import {useState} from 'react';
import {useEffect} from 'react';
import {Composition, staticFile} from 'remotion';
import {
	CaptionedVideo,
	calculateCaptionedVideoMetadata,
	captionedVideoSchema,
} from './CaptionedVideo';
import './style.css';
import {MyComposition} from './Composition';
import axios from 'axios';
import {SpeakerMapper} from './SpeakerMapper';

// Each <Composition> is an entry in the sidebar!
export interface ISpeaker {
	hashedText: string;
	speakerType: 'main' | 'sub_text' | 'sub_voice' | 'title';
	audioDurationInFrames: number;
	text: string;
	userName: string;
}

const calculateVideoFrames = (speakers: ISpeaker[]) => {
	let maxDuration = 0;
	for (const speaker of speakers) {
		maxDuration += speaker.audioDurationInFrames;
	}

	return Math.floor(maxDuration);
};

export const RemotionRoot: React.FC = () => {
	const [story, setStoryData] = useState<ISpeaker[]>();

	useEffect(() => {
		const fetchStory = async () => {
			const response = await axios.get(staticFile('/temp_assets/story.json'));
			const speakerMapper = new SpeakerMapper();
			const parsedStories: ISpeaker[] = speakerMapper.mapSpeakerBulk(response);
			setStoryData(parsedStories);
		};
		fetchStory();
	}, []);

	if (!story) {
		return null;
	}

	return (
		<>
			<Composition
				id="CaptionedVideo"
				component={CaptionedVideo}
				calculateMetadata={calculateCaptionedVideoMetadata}
				schema={captionedVideoSchema}
				width={1080}
				height={1920}
				defaultProps={{
					src: staticFile('/temp_assets/temp/story.mp4'),
				}}
			/>

			<Composition
				id="Story"
				component={MyComposition}
				durationInFrames={calculateVideoFrames(story)}
				defaultProps={{story}}
				fps={30}
				width={1080}
				height={1920}
			/>
		</>
	);
};
