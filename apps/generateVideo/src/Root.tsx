import {useState} from 'react';
import {useEffect} from 'react';
import {
	cancelRender,
	Composition,
	continueRender,
	delayRender,
	staticFile,
} from 'remotion';
import {
	CaptionedVideo,
	calculateCaptionedVideoMetadata,
} from './CaptionedVideo';
import './style.css';
import {MyComposition, Story, StoryFragment} from './Composition';
import axios from 'axios';
import {StoryMapper} from './StoryMapper';

// Each <Composition> is an entry in the sidebar!

const calculateVideoFrames = (fragments: StoryFragment[] | null) => {
	let maxDuration = 0;
	if (fragments) {
		for (const speaker of fragments) {
			maxDuration += speaker.audioDurationInFrames;
		}
	}
	return Math.floor(maxDuration);
};

export const RemotionRoot: React.FC = () => {
	const [story, setStoryData] = useState<Story | undefined>(undefined);
	const [handle] = useState(() => delayRender());

	useEffect(() => {
		const fetchStory = async () => {
			try {
				const response = await axios.get(
					staticFile('/temp_assets/story_fragments.json'),
				);
				const speakerMapper = new StoryMapper();
				const parsedStory = speakerMapper.mapStoryResponse(response);
				setStoryData(parsedStory);
				continueRender(handle);
			} catch (error) {
				cancelRender(error);
			}
		};

		fetchStory();
	}, [handle]);

	if (!story) {
		return null;
	}
	console.log(story);
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
				durationInFrames={calculateVideoFrames(story.fragments)}
				defaultProps={story}
				fps={30}
				width={1080}
				height={1920}
			/>
		</>
	);
};
