import {
	Sequence,
	interpolate,
	spring,
	staticFile,
	useCurrentFrame,
	useVideoConfig,
	Audio,
} from 'remotion';
import {ChatMessageText} from './ChatMessageText';

export interface IBanner {
	delay: number;
	text: string;
	userName: string;
	duruation: number;
}

export const ChatMessageBanner: React.FC<IBanner> = ({
	delay,
	text,
	userName,
	duruation,
}) => {
	const {fps} = useVideoConfig();
	const frame = useCurrentFrame();

	const slide = spring({
		fps,
		frame: frame - delay,
		config: {
			damping: 200,
		},
	});
	const yPosition = interpolate(slide, [0, 1], [-0.5, 0.15]);

	return (
		<Sequence
			style={{
				top: yPosition * 100 + '%',
				left: '5%',
				width: '90%',
				height: 'fit-content',
				position: 'absolute',
			}}
			from={delay - 15}
			durationInFrames={duruation}
			className="text-6xl text-white text-left bg-zinc-700  border-1 border-zinc-700 w-[80%] rounded-t-[40px] h-fit rounded-b-[40px] pb-10 flex-col bg-opacity-80"
		>
			<ChatMessageText text={text} userName={userName} />
			<Audio
				volume={1}
				src={staticFile('/assets/sounds/discord-notification.mp3')}
			/>
		</Sequence>
	);
};
