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
			className="text-white text-left bg-zinc-700 border-zinc-700 flex justify-around pb-10"
			style={{
				opacity,
				top: '35%',
				height: 'fit-content',
				position: 'absolute',
				width: '98%',
				justifySelf: 'center',
				overflowWrap: 'break-word',
			}}
		>
			<section className="px-3 py-5">
				<Img
					src={staticFile(
						`/assets/images/discord_default/discord_${defaultAvatars[randomNumber]}.png`,
					)}
					style={{
						width: '170px',
						height: 'auto',
						maxWidth: '170px',
						position: 'relative',
					}}
					className="rounded-[50%]"
				/>
			</section>
			<section className="flex-col text-[30px]  grow ">
				<div className="flex items-center justify-between w-full gap-x-3">
					<p className="text-[4rem] text-red-400  px-1  font-sans font-semibold text-center pt-1 max-w-[60%] break-words ">
						{userName}
					</p>
					<p className="text-center font-semibold text-[2rem] text-gray-300 pt-5 font-sans max-w-[40%] pr-5">
						05/08/2024 3:41AM
					</p>
				</div>
				<p className=" h-auto text-[2.5rem] text-gray-200 font-semibold min-w-[90%] max-w-[90%] pl-3">
					{text}
				</p>
			</section>
		</Sequence>
	);
};
