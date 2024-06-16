import React from 'react';
import {AbsoluteFill, interpolate, useVideoConfig} from 'remotion';
import {TheBoldFont} from '../load-font';
import {fitText} from '@remotion/layout-utils';
import {makeTransform, scale, translateY} from '@remotion/animation-utils';

const fontFamily = TheBoldFont;

export const Word: React.FC<{
	enterProgress: number;
	text: string;
	stroke: boolean;
}> = ({enterProgress, text, stroke}) => {
	const {width} = useVideoConfig();
	const desiredFontSize = 80;

	const fittedText = fitText({
		fontFamily,
		text,
		withinWidth: width * 0.9,
	});

	const fontSize = Math.min(desiredFontSize, fittedText.fontSize);

	return (
		<AbsoluteFill
			style={{
				justifyContent: 'center',
				alignItems: 'center',
				top: '40%',
				bottom: undefined,
				height: 250,
			}}
		>
			<div
				style={{
					fontSize,
					color: 'white',
					WebkitTextStroke: stroke ? '20px black' : undefined,
					transform: makeTransform([
						scale(interpolate(enterProgress, [0, 1], [0.7, 1])),
						translateY(interpolate(enterProgress, [0, 1], [50, 0])),
					]),
					fontFamily,
					textTransform: 'uppercase',
				}}
			>
				{text}
			</div>
		</AbsoluteFill>
	);
};
