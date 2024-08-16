/* eslint-disable guard-for-in */
/* eslint-disable @typescript-eslint/no-explicit-any */
import {Story, StoryFragment} from './Composition';
import {AxiosResponse} from 'axios';

export type JsonFragment = {
	voice_name: string;
	user_name: string;
	speaker_text: string;
	audio_duration_in_frames: number;
	hashed_text: string;
	speaker_type: 'main' | 'sub_text' | 'sub_voice' | 'title';
	gender: string;
};

export type JsonStory = {
	story_type: string;
	genre: string;
	fragments: JsonFragment[];
};

export class StoryMapper {
	mapFragment(jsonFragment: JsonFragment): StoryFragment {
		const fragment: StoryFragment = {
			hashedText: jsonFragment.hashed_text,
			speakerType: jsonFragment.speaker_type,
			audioDurationInFrames: jsonFragment.audio_duration_in_frames,
			text: jsonFragment.speaker_text,
			userName: jsonFragment.user_name,
			voiceName: jsonFragment.voice_name,
			gender: jsonFragment.gender,
		};
		return fragment;
	}

	mapStory(jsonStory: JsonStory): Story {
		const story: Story = {
			storyType: jsonStory.story_type,
			genre: jsonStory.genre,
			fragments: jsonStory.fragments.map((fragment: JsonFragment) =>
				this.mapFragment(fragment),
			),
		};
		return story;
	}

	mapStoryResponse(response: AxiosResponse<any, any>): Story {
		return this.mapStory(response.data);
	}
}
