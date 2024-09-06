/* eslint-disable react/button-has-type */
import {random} from 'remotion';
import * as Icons from './Icons';

interface IChannelTopbar {
	randomSeed: string;
}

export const ChannelTopbar: React.FC<IChannelTopbar> = ({randomSeed}) => {
	const channelList = [
		'gawk-gawk-gawk-3000',
		'yes-yes-yes-yes',
		'Big-Ahhh',
		'Can-I-get-a-hoya-bich',
		'Protectors-of-the-Schlong',
		'Ohio-Rizz',
		'Why-are-you-gay',
		'LGBTQHDT+-4k',
		'Karens-Only',
		'Gingers-have-no-soul',
		'All-About-The-Gawk',
		'Backdoor-Only',
		'Adriana-Chechik',
		'Womp-Womp',
		'To-Joke-to-Woke',
		'Homos-Only',
		'Made-In-Ohio',
		'Trans-Rangers',
		'Boko-no-Schlong',
		'Attack-on-Gawk-Gawk-Gawk',
		'Only-Woke-Gawk-Gawk-Gawk',
		'Send-Location',
		'Your-Wife-Is-In-Me-DM',
		'Fight-me',
		'9+10=21',
		'Daddy-Chill',
		'Huak-Deez-Nuts',
		'Suck-it-Garry',
		'The-Gawk-Gawk-Show',
		'Witty-Whispers',
		'Noodle-Friends',
		'Rizz-Room',
		'Suck-it',
		'Moist',
		'Your_Mom_Looks_Like_Your_Dad',
		'Cock',
		'Cockpit',
		'Shut-up-marc',
		'Keep-Quite-Angela',
	];

	const randomNumber = Math.floor(random(randomSeed) * channelList.length - 1);
	const channelName = channelList[randomNumber];

	return (
		<div className="flex items-center px-2 h-20 shadow-2xl border-b-2  border-[#2F3136]">
			<div className="flex items-center">
				<Icons.Hashtag className="mx-2  font-semibold text-gray-100 font w-8 h-8" />
				<span className="mr-2 font-bold text-white whitespace-nowrap text-4xl mb-2">
					{channelName}
				</span>
			</div>

			<Icons.AddPerson className="ml-auto w-10 h-10 text-gray-100  opacity-0 group-hover:opacity-100" />
			<Icons.HashtagWithSpeechBubble className="mx-2 w-10 h-10 text-gray-100" />
			<Icons.People className="mx-2 w-10 h-10 text-gray-100" />
		</div>
	);
};
