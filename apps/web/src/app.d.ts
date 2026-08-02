import type { MeUser } from '$lib/server/api';

declare global {
	namespace App {
		interface Locals {
			user: MeUser | null;
		}
	}
}

export {};
