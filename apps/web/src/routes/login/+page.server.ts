import type { Actions, PageServerLoad } from './$types';
import { fail, redirect } from '@sveltejs/kit';
import { apiBaseUrl, forwardSetCookies, frontendOrigin } from '$lib/server/api';

type LoginUser = {
  role: string;
  accountId: string | null;
};

export const load: PageServerLoad = async ({ locals }) => {
  if (locals.user) {
    if (locals.user.role === 'admin') throw redirect(303, '/admin');
    if (locals.user.accountId) throw redirect(303, '/portal');
    throw redirect(303, '/');
  }
  return {};
};

export const actions: Actions = {
  default: async ({ request, cookies }) => {
    const form = await request.formData();
    const usernameOrEmail = String(form.get('usernameOrEmail') ?? '').trim();
    const password = String(form.get('password') ?? '');

    if (!usernameOrEmail || !password) {
      return fail(400, { error: 'Username/email and password are required.', usernameOrEmail });
    }

    let response: Response;
    try {
      response = await fetch(`${apiBaseUrl()}/api/v1/auth/login`, {
        method: 'POST',
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json',
          Origin: frontendOrigin()
        },
        body: JSON.stringify({ usernameOrEmail, password })
      });
    } catch {
      return fail(503, {
        error: 'Service is temporarily unavailable. Please try again shortly.',
        usernameOrEmail
      });
    }

    if (!response.ok) {
      let message = 'Invalid credentials.';
      try {
        const body = (await response.json()) as { error?: { message?: string } };
        if (body.error?.message) message = body.error.message;
      } catch {
        /* keep default */
      }
      return fail(response.status, { error: message, usernameOrEmail });
    }

    const user = (await response.json()) as LoginUser;
    forwardSetCookies(response, cookies);

    if (user.role === 'admin') throw redirect(303, '/admin');
    if (user.accountId) throw redirect(303, '/portal');
    throw redirect(303, '/');
  }
};
