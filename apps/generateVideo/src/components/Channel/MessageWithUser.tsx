import {Audio} from 'remotion';
import {Img, Sequence} from 'remotion';
import {random, staticFile} from 'remotion';
import {calculatingStartingFrame, Story} from '../../Composition';
import {translateY} from '@remotion/animation-utils';
import {Delayed} from '../../hooks/Delayed';

interface IChatMessageText {
	text: string;
	userName: string;
	index: number;
	story: Story;
}

const MessageWithUser: React.FC<IChatMessageText> = ({
	text,
	userName,
	index,
	story,
}) => {
	const defaultAvatars: string[] = ['blue', 'red', 'yellow', 'green', 'grey'];
	const randomNumber = Math.floor(random(text) * 5);
	const transform = translateY(1);
	const date = new Date();
	date.setMonth(1);
	date.setHours(2) + 2;

	return (
		<Sequence
			className="flex py-0.5 pb-8 leading-[22px]"
			from={calculatingStartingFrame(story, index) + 20}
			style={{
				left: '5%',
				width: '90%',
				height: 'fit-content',
				position: 'relative',
				transform,
			}}
		>
			<>
				<Audio
					volume={1}
					src={staticFile('/assets/sounds/discord-notification.mp3')}
				/>

				<div className=" relative mt-0.5 mr-4 w-10 min-w-fit h-10 rounded-full">
					<Img
						className="mt-0.5 mr-4 w-20 h-20 rounded-full z-10"
						height={200}
						width={200}
						src={staticFile(
							`/assets/images/discord_default/discord_${defaultAvatars[randomNumber]}.png`,
						)}
					/>
				</div>
				<div>
					<p className="flex items-baseline">
						<span className="mr-2 font-medium text-4xl text-green-400">
							{userName}
						</span>
						<span className="text-lg font-medium text-gray-400">
							{date.toDateString()}
						</span>
					</p>

					<div className="max-h-fit">
						{text.split(' ').map((word: string, index: number) => {
							return (
								<Delayed waitBeforeShow={index * 200 - index}>
									<p className="text-gray-100 text-3xl float-left pr-2">
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
