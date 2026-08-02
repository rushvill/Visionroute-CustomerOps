import type { Actions } from './$types';
import { fail } from '@sveltejs/kit';
import { createPrivacyRequest } from '$lib/server/api';

export const actions: Actions = {
  default: async ({ request }) => {
    const form = await request.formData();
    const requesterName = String(form.get('requesterName') ?? '').trim();
    const requesterEmail = String(form.get('requesterEmail') ?? '').trim();
    const requestType = String(form.get('requestType') ?? 'other').trim();
    const details = String(form.get('details') ?? '').trim();

    if (!requesterEmail) {
      return fail(400, {
        error: 'Email is required.',
        requesterName,
        requesterEmail,
        details
      });
    }

    const result = await createPrivacyRequest({
      requesterEmail,
      requestType,
      ...(requesterName ? { requesterName } : {}),
      ...(details ? { details } : {})
    });

    if (!result.ok) {
      return fail(result.status, {
        error: result.message,
        requesterName,
        requesterEmail,
        details
      });
    }

    return { success: true };
  }
};
