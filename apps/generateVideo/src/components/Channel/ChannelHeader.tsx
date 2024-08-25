/* eslint-disable react/button-has-type */
import {random} from 'remotion';
import * as Icons from './Icons';

interface IChannelTopbar {
	randomSeed: string;
}

export const ChannelTopbar: React.FC<IChannelTopbar> = ({randomSeed}) => {
	const channelList = [
		'𝚃𝚑𝚎 𝙷𝚊𝚔𝚎𝚛𝚜 𝚘𝚏 𝙼𝚞𝚕𝚝𝚒𝚙𝚕𝚊𝚢',
		'𝓒𝓱𝓪𝓽𝓽𝓲𝓮𝓼 𝓕𝓸𝓻 𝓕𝓻𝓲𝓮𝓷𝓭𝓼',
		'𝐆𝐑𝐎𝐔𝐏 𝐆𝐋𝐀𝐃𝐒',
		'ƮĦĔ ŔΔŊĠΣŘŞ',
		'𝐓𝐡𝐞 𝐆𝐫𝐞𝐚𝐭 𝐁𝐚𝐝𝐝',
		'𝐊𝐞𝐞𝐩 𝐓𝐡𝐞 𝐂𝐡𝐚𝐭𝐭',
		'𝕊𝕦𝕡𝕡𝕠𝕣𝕥 𝕊𝕙𝕦𝕣𝕝𝕤',
		'ʂυρєrѕтαя Ɠαмєяѕ',
		'𝓗𝓸𝓹𝓹𝓲𝓮 𝓗𝓪𝔃𝓮𝓻𝓼',
		'𝖆𝖚𝖙𝖙𝖎𝖈𝖍 𝖉𝖆𝖛𝖊𝖓𝖉𝖊𝖗𝖘',
		'𝕃𝕠𝕧𝕖𝕝𝕪 𝕃𝕚𝕥𝕥𝕝𝕖𝕤',
		'ᴛʜᴇ ᴍᴀɢɪᴄᴀʟ ᴍᴏᴠᴇʀs',
		'𝐓𝐇𝐄 𝐃𝐑𝐈𝐕𝐄𝐑𝐒',
		'gawk-gawk-gawk 3000',
		'yes yes yes yes',
		'Big Ahhh',
		'Can I get a hoya bich',
		'Protectors of the Schlong',
		'Ohio Rizz',
		'Why are you gay',
		'LGBTQHDT+ 4k',
		'To Joke to Woke',
		'Boko no Schlong',
		'Attack on Gawk Gawk Gawk',
		'Only Woke Gawk Gawk Gawk',
		'Fight me',
		'9 + 10 = 21',
		'Daddy Chill',
		'Huak Deez Nuts',
		'Suck it Garry',
		'Shut up marc',
		'Keep Quite Angela',
	];

	const randomNumber = Math.floor(random(randomSeed) * channelList.length - 1);
	const channelName = channelList[randomNumber];

	return (
		<div className="flex items-center px-2 h-20 shadow-2xl border-1 border-green-50">
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
