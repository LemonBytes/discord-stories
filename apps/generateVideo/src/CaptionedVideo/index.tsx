import { useEffect, useState} from 'react';
import {
	AbsoluteFill,

	cancelRender,
	Sequence,
	useVideoConfig,
	
} from 'remotion';

import Subtitle from './Subtitle';
import {loadFont} from '../load-font';

const text = `I've always been a lurker. Scrolling through endless threads on Reddit, occasionally dropping a comment on a funny meme, but that's about it. So, when I found myself inexplicably drawn into the rabbit hole of the "Nosleep Writers' Bunker" Discord server, it surprised even me.
Maybe it was the late-night insomnia, the allure of a community dedicated to crafting horror stories, or perhaps the intriguing prompt for the week's writing challenge: "an unsettling message found online." Whatever the reason, there I was, a wide-eyed newbie amidst seasoned horror scribes.`
export type SubtitleProp = {
	offsets: {
		from: number;
		to: number;
	};
	text: string;
};

export const CaptionedVideo = () => {
	const [subtitles, setSubtitles] = useState<SubtitleProp[]>([]);
	const {fps} = useVideoConfig();

	useEffect(() => {
			const  fetchSubtitles =  () => {

				try {
					loadFont();
					console.log("test22s")
					const words = text.split(" ");
					   for(let i = 0; i < words.length; i ++) {
						console.log(words[i])
						   const subtitle = {
							   offsets: {
								   from: i,
								   to: i * 301 + words[i].length 
							   },
							   text: words[i]
						   }
						   subtitles.push(subtitle)
					   }
					   setSubtitles(subtitles);
				
			   } catch (e) {
				console.log("test")
				   cancelRender(e);
			   }
			}
		 fetchSubtitles()
	}, [subtitles]);



	return (
		<AbsoluteFill>
			{subtitles.map((subtitle, index) => {
				const subtitleStartFrame = (subtitle.offsets.from * fps) / 1000;
				const subtitleEndFrame = (subtitle.offsets.to * fps) / 1000;

				return (
					<Sequence
						from={subtitleStartFrame}
						durationInFrames={subtitleEndFrame - subtitleStartFrame}
					>
						<Subtitle key={index} text={subtitle.text} />;
					</Sequence>
				);
			})}
		
		</AbsoluteFill>
	);
};
