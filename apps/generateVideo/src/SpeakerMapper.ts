/* eslint-disable guard-for-in */
/* eslint-disable @typescript-eslint/no-explicit-any */
import {ISpeaker} from './Composition';
import {AxiosResponse} from 'axios';

export class SpeakerMapper {
	mapSpeaker(jsonSpeaker: string): ISpeaker {
		const parsedSpeaker = JSON.parse(jsonSpeaker);

		const speaker: ISpeaker = {
			hashedText: parsedSpeaker.hashed_text,
			speakerType: parsedSpeaker.speaker_type,
			audioDurationInFrames: parsedSpeaker.audio_duration_in_frames,
			text: parsedSpeaker.speaker_text,
			userName: parsedSpeaker.user_name,
		};
		return speaker;
	}

	mapSpeakerBulk(response: AxiosResponse<any, any>): ISpeaker[] {
		const parsedSpeakers: ISpeaker[] = [];
		const speakers = response.data.story;

		for (const speaker of speakers) {
			parsedSpeakers.push(this.mapSpeaker(JSON.stringify(speaker)));
		}
		return parsedSpeakers;
	}
}
