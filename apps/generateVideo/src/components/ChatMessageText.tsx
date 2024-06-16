import React from 'react';
import {Img, staticFile} from 'remotion';

interface IChatMessageText {
	text: string;
	userName: string;
}

export const ChatMessageText: React.FC<IChatMessageText> = ({
	text,
	userName,
}) => {
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
			<p className="p-6 pr-7 font-light">{text}</p>
		</div>
	);
};
