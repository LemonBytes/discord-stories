import React from 'react';
import VoiceAdapter from '../adapters/VoiceAdapters';

const text = `I inherited my grandpa's old pocket watch after he passed. It's a beautiful thing, silver with intricate engravings, but it hasn't worked in years. The second hand just twitches erratically.

The other day, I was feeling nostalgic and decided to fiddle with it. I wound it up for the first time, not really expecting anything. To my surprise, it started ticking. A slow, steady tick-tock that filled the silence of my apartment.

I went about my day, the ticking a constant presence in the background. But then, something strange happened. I was making myself a cup of coffee when I swear I saw the second hand jump forward a full minute. I blinked, thinking I must have imagined it, but then it happened again. And again.

The watch wasn't just keeping time, it was skipping it. Minutes at first, then seconds, then whole hours. The room seemed to warp around me, a sense of disorientation settling in. I felt like I was on a train speeding through time, the world blurring past the window.

Panicked, I tried to stop the watch, but it wouldn't budge. The ticking quickened, becoming a frantic, maddening beat. Then, just as suddenly as it started, it stopped.

Silence. My heart pounded in my chest, and I felt strangely lightheaded. I looked at the clock on the wall. It was two hours later. Two hours had vanished in the blink of an eye.

I haven't touched the watch since. I'm scared of it, but also strangely fascinated. Did I experience a glitch in the matrix? Did I travel through time? Or am I just going crazy?

Edit: I forgot to mention, the watch now shows a different date. A date 20 years in the future.`;

export const UseGenerateVoice = () => {
	const [data, setData] = React.useState<unknown>(null);

	React.useEffect(() => {
		const fetchData = async () => {
			const voiceAdapter = new VoiceAdapter();
			const generatedText = voiceAdapter.generateVoice(text);
			setData(generatedText);
		};

		fetchData();
	}, []);

	return [data];
};
