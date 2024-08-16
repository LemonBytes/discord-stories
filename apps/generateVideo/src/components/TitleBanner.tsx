import React from 'react';
import {IBanner} from './ChatMessageBanner';
import {
	Img,
	Sequence,
	interpolate,
	random,
	staticFile,
	useCurrentFrame,
} from 'remotion';

export const TitleBanner: React.FC<IBanner> = ({
	delay,
	text,
	userName,
	duruation,
}) => {
	const frame = useCurrentFrame();

	const defaultAvatars: string[] = ['blue', 'red', 'yellow', 'green', 'grey'];

	const randomNumber = Math.floor(random(text) * 5);

	const opacity = interpolate(
		frame,
		[0, 30, duruation - 20, duruation],

		[0, 1, 1, 0],
	);

	return (
		<Sequence
			from={delay}
			durationInFrames={duruation}
			className="text-white text-left bg-zinc-700 border-zinc-700 flex justify-around pb-10 w-[95%] "
			style={{
				opacity,
				top: '35%',
				height: 'fit-content',
				position: 'absolute',
				width: '80%',
				justifySelf: 'center',
				overflowWrap: 'break-word',
				borderRadius: '10px',
			}}
		>
			<section className="px-3 py-5">
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
					className="rounded-[50%]"
				/>
			</section>
			<section className="flex-col text-[30px]  grow ">
				<div className="flex items-center w-full gap-x-5">
					<p className="text-[50px] text-red-400  px-1  font-sans font-semibold text-center pt-1 max-w-[70%] break-words ">
						{userName}
					</p>
					<p className="text-center font-semibold text-[1.5rem] text-gray-300 pt-5 font-sans max-w-[40%] ">
						Today at 7:05 PM
					</p>
				</div>
				<p className=" h-auto text-[2.6rem] text-gray-200 font-semibold min-w-[95%] max-w-[90%] pl-3">
					{text}
				</p>
			</section>
		</Sequence>
	);
};
