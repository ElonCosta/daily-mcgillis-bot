import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = ({ url, fetch, params }) => {
	return fetch(`http://localhost:3000/images/${params.name}`);
};
