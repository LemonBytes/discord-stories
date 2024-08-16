import React from 'react';
import {Img, staticFile} from 'remotion';
import {Delayed} from '../hooks/Delayed';

interface IChatMessageText {
	text: string;
	userName: string;
}

export const ChatMessageText: React.FC<IChatMessageText> = ({
	text,
	userName,
}) => {
	console.log(text.split('').length);
	return (
		<div>
			<div className="flex justify-between p-5 mt-3">
				<div className="flex items-center space-x-5">
					<Img
						src={staticFile('/assets/images/discord_icon.png')}
						width="100"
						height="100"
					/>
					<div className="font-thin  text-zinc-400">DISCORD</div>
				</div>
				<div className="px-5 text-zinc-400 font-thin">now</div>
			</div>
			<p className="px-6 py-2">{userName}</p>
			<div className="p-6 pr-7">
				{text.split(' ').map((word: string, index: number) => {
					return (
						<Delayed waitBeforeShow={index * 200 - index}>
							<p
								key={index + word}
								className="p-1 font-light text-5xl transition-all float-left"
							>
								{word}
							</p>
						</Delayed>
					);
				})}
			</div>
		</div>
	);
};
