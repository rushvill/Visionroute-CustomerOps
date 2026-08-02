import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { apiBaseUrl, forwardSetCookies, frontendOrigin } from '$lib/server/api';

export const load: PageServerLoad = async ({ locals }) => {
  return { user: locals.user };
};

export const actions: Actions = {
  logout: async ({ request, cookies }) => {
    const cookieHeader = request.headers.get('cookie') ?? '';
    try {
      const response = await fetch(`${apiBaseUrl()}/api/v1/auth/logout`, {
        method: 'POST',
        headers: {
          Accept: 'application/json',
          Origin: frontendOrigin(),
          Cookie: cookieHeader
        }
      });
      forwardSetCookies(response, cookies);
    } catch {
      /* still clear local cookie */
    }
    cookies.delete('vr_ops_session', { path: '/' });
    throw redirect(303, '/login');
  }
};
