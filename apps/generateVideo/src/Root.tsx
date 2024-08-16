import {useState} from 'react';
import {useEffect} from 'react';
import {Composition, staticFile} from 'remotion';
import {
	CaptionedVideo,
	calculateCaptionedVideoMetadata,
} from './CaptionedVideo';
import './style.css';
import {ISpeaker, MyComposition, Story} from './Composition';
import axios from 'axios';
import {SpeakerMapper} from './SpeakerMapper';

// Each <Composition> is an entry in the sidebar!

const calculateVideoFrames = (speakers: ISpeaker[] | null) => {
	let maxDuration = 0;
	if (speakers) {
		for (const speaker of speakers) {
			maxDuration += speaker.audioDurationInFrames;
		}
	}
	return Math.floor(maxDuration);
};

export const RemotionRoot: React.FC = () => {
	const [story, setStoryData] = useState<Story>();

	useEffect(() => {
		const fetchStory = async () => {
			const response = await axios.get(
				staticFile('/temp_assets/story_fragments.json'),
			);
			const speakerMapper = new SpeakerMapper();
			const parsedStories = speakerMapper.mapSpeakerBulk(response);
			setStoryData({story: parsedStories});
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
				width={1080}
				height={1920}
				defaultProps={{
					src: staticFile('/temp_assets/temp/uncaptioned_story.mp4'),
					story,
				}}
			/>

			<Composition
				id="Story"
				component={MyComposition}
				durationInFrames={calculateVideoFrames(story.story)}
				defaultProps={{...story}}
				fps={30}
				width={1080}
				height={1920}
			/>
		</>
	);
};
