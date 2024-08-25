import {useState, useEffect} from 'react';
import axios from 'axios';

const useRandomUserPicture = (seed: string, gender: string) => {
	const [picture, setPicture] = useState(null);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<null | unknown>(null);

	useEffect(() => {
		const fetchUser = async () => {
			setLoading(true);
			try {
				const response = await axios.get(
					`https://randomuser.me/api/?gender=${gender}?seed=${seed}`,
				);
				const user = response.data.results[0];
				setPicture(user.picture.medium);
			} catch (err: unknown) {
				setError(err);
			} finally {
				setLoading(false);
			}
		};

		fetchUser();
	}, [seed, gender]);

	return {picture, loading, error};
};

export default useRandomUserPicture;
