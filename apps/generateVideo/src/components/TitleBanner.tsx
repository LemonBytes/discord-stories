import React from 'react';
import {IBanner} from './ChatMessageBanner';
import {
	Img,
	Sequence,
	interpolate,
	random,
	spring,
	staticFile,
	useCurrentFrame,
	useVideoConfig,
} from 'remotion';

export const TitleBanner: React.FC<IBanner> = ({
	delay,
	text,
	userName,
	duruation,
}) => {
	const {fps} = useVideoConfig();
	const frame = useCurrentFrame();
	const slide = spring({
		fps,
		frame,
		config: {
			damping: 500,
		},
	});
	const opacity = interpolate(slide, [100, 200], [1, 0]);
	const defaultAvatars: string[] = ['blue', 'red', 'yellow', 'green', 'grey'];

	const randomNumber = Math.floor(random(text) * 5);

	return (
		<Sequence
			from={delay}
			durationInFrames={duruation}
			className="text-white text-left bg-zinc-700 border-zinc-700 w-screen flex justify-between pb-10"
			style={{
				opacity,
				top: '35%',
				height: 'fit-content',
				position: 'absolute',
			}}
		>
			<section className="px-5 py-5">
				<Img
					src={staticFile(
						`/assets/images/discord_default/discord_${defaultAvatars[randomNumber]}.png`,
					)}
					style={{
						width: '150px',
						height: 'auto',
						maxWidth: '150px',
						position: 'relative',
					}}
					className="rounded-[50%] "
				/>
			</section>
			<section className="flex-col text-6xl ">
				<div className="flex items-center ">
					<p className="text-[70px] text-cyan-500 px-1  font-sans font-bold text-center pt-1 capitalize">
						{userName}
					</p>
					<p className="text-center font-semibold text-[40px] text-gray-400 pt-5 font-sans px-3">
						05/08/2024 3:41AM
					</p>
				</div>
				<p className="p-1 w-[90%] ">{text}</p>
			</section>
		</Sequence>
	);
};
