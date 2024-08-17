/* eslint-disable react/button-has-type */
import * as Icons from './Icons';

export const ChannelTopbar = () => {
	const channelName: string = 'gawk gawk gawk >';
	return (
		<div className="flex items-center px-2 h-20 shadow-xl">
			<div className="flex items-center">
				<Icons.Hashtag className="mx-2  font-semibold text-gray-200 font w-8 h-8" />
				<span className="mr-2 font-bold text-white whitespace-nowrap text-4xl mb-2">
					{channelName}
				</span>
			</div>

			<Icons.AddPerson className="ml-auto w-10 h-10 text-gray-400 hover:text-gray-100 opacity-0 group-hover:opacity-100" />
			<Icons.HashtagWithSpeechBubble className="mx-2 w-10 h-10 text-gray-400" />
			<Icons.People className="mx-2 w-10 h-10 text-gray-400" />
		</div>
	);
};
