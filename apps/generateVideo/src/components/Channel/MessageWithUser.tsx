import {Audio} from 'remotion';
import {Img, Sequence} from 'remotion';
import {random, staticFile} from 'remotion';
import {calculatingStartingFrame, Story} from '../../Composition';
import {Delayed} from '../../hooks/Delayed';
import useRandomUserPicture from '../../hooks/RandomPicture';

interface IChatMessageText {
	text: string;
	userName: string;
	index: number;
	story: Story;
	date: string;
	gender: string;
}

const MessageWithUser: React.FC<IChatMessageText> = ({
	text,
	userName,
	index,
	story,
	date,
	gender,
}) => {
	const {picture} = useRandomUserPicture(userName, gender);

	const defaultAvatars: string[] = ['blue', 'red', 'yellow', 'green', 'grey'];
	const randomNumber = Math.floor(random(userName) * 5);

	function getRandomRgb(seed: string) {
		const num = Math.round(0xffffff * random(seed));
		const r = num >> 16;
		const b = num & 255;
		return 'rgb(' + r + ', ' + 77 + ', ' + b + 30 + ')';
	}
	return (
		<Sequence
			className="flex py-0.5 pb-8 leading-[22px]"
			from={calculatingStartingFrame(story, index) + 20}
			style={{
				left: '5%',
				width: '90%',
				height: 'fit-content',
				position: 'relative',
				borderRadius: '20px',
				margin: '20px 0px 20px 0px',
			}}
		>
			<>
				<Audio
					volume={1}
					src={staticFile('/assets/sounds/discord-notification.mp3')}
				/>

				<div className=" relative mt-0.5 mr-4 w-10 min-w-fit h-10 rounded-full">
					<Img
						className="mt-0.5 mr-4 w-28 h-28 rounded-full z-10"
						height={300}
						width={300}
						src={
							picture ??
							staticFile(
								`/assets/images/discord_default/discord_${defaultAvatars[randomNumber]}.png`,
							)
						}
					/>
				</div>
				<div>
					<p className="flex items-center space-x-2">
						<span
							className="mr-2 font-bold text-5xl"
							style={{
								color: getRandomRgb(userName),
								WebkitTextStroke: '1px black',
							}}
						>
							{userName}
						</span>
						<span className="text-base pt-2 font-medium text-gray-300">
							{date}
						</span>
					</p>

					<div className="max-h-fit ">
						{text.split(' ').map((word: string, index: number) => {
							return (
								<Delayed waitBeforeShow={index * 200 - index}>
									<p
										style={{
											color: 'whitesmoke',
											WebkitTextStroke: '0.1px black',
										}}
										className="text-4xl font-semibold float-left pr-2 text-gray-50"
									>
										{word}
									</p>
								</Delayed>
							);
						})}
					</div>
				</div>
			</>
		</Sequence>
	);
};

export default MessageWithUser;
