import type { Handle } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import { dev } from '$app/environment';
import { fetchMe } from '$lib/server/api';

function applySecurityHeaders(response: Response, isDev: boolean): void {
  response.headers.set('X-Content-Type-Options', 'nosniff');
  response.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');
  response.headers.set('X-Frame-Options', 'DENY');
  response.headers.set(
    'Permissions-Policy',
    'camera=(), microphone=(), geolocation=()'
  );
  response.headers.set('Cross-Origin-Opener-Policy', 'same-origin');
  response.headers.set('X-Permitted-Cross-Domain-Policies', 'none');

  // SvelteKit needs an inline bootstrap script to hydrate. Prefer nonces later;
  // without 'unsafe-inline' in production, all client interactivity (Show password, etc.) is dead.
  const csp = [
    "default-src 'self'",
    "base-uri 'none'",
    "object-src 'none'",
    "frame-ancestors 'none'",
    "form-action 'self'",
    "img-src 'self' data:",
    "font-src 'self' https://fonts.gstatic.com",
    "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com",
    isDev
      ? "script-src 'self' 'unsafe-inline' 'unsafe-eval'"
      : "script-src 'self' 'unsafe-inline'",
    "connect-src 'self' https://*.onrender.com http://127.0.0.1:8080 http://localhost:8080"
  ].join('; ');

  response.headers.set(
    isDev ? 'Content-Security-Policy-Report-Only' : 'Content-Security-Policy',
    csp
  );

  if (!isDev) {
    response.headers.set(
      'Strict-Transport-Security',
      'max-age=31536000; includeSubDomains'
    );
  }
}

export const handle: Handle = async ({ event, resolve }) => {
  const cookieHeader = event.request.headers.get('cookie');
  event.locals.user = await fetchMe(cookieHeader);

  const path = event.url.pathname;
  if (path.startsWith('/admin')) {
    if (!event.locals.user) {
      throw redirect(303, '/login');
    }
    if (event.locals.user.role !== 'admin') {
      throw redirect(303, '/');
    }
  }

  if (path.startsWith('/portal')) {
    if (!event.locals.user) {
      throw redirect(303, '/login');
    }
  }

  const response = await resolve(event);
  applySecurityHeaders(response, dev);
  return response;
};
