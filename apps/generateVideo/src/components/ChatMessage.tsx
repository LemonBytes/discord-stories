import {
	AbsoluteFill,
	Sequence,
	interpolate,
	spring,
	staticFile,
	useCurrentFrame,
	useVideoConfig,
	Audio,
} from 'remotion';

interface IChatMessage {
	delay: number;
}

export const ChatMessage: React.FC<IChatMessage> = ({delay}) => {
	const {fps} = useVideoConfig();
	const frame = useCurrentFrame();

	const slide = spring({
		fps,
		frame: frame - delay,
		config: {
			damping: 50,
		},
	});
	const yPosition = interpolate(slide, [0, 1], [-0.5, 0.03]);

	return (
		<AbsoluteFill
			style={{
				left: 20,
				top: yPosition * 100 + '%',
				position: 'absolute',
			}}
		>
			<div className=" text-5xl text-white text-left bg-zinc-700 border-1 font-semibold border-zinc-700 w-[95%] h-auto p-4 rounded-xl">
				Welcome! Glad to have you. Unsure what unsettling message awaits, but
				prepare for chills.
			</div>

			<Sequence from={delay - 8}>
				<Audio src={staticFile('/sounds/discord-notification.mp3')} />
			</Sequence>
		</AbsoluteFill>
	);
};
