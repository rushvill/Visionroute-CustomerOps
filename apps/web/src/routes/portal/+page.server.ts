import type { Actions, PageServerLoad } from './$types';
import { fail } from '@sveltejs/kit';
import {
  createMePrivacyRequest,
  fetchMeAccount,
  fetchMeDevices,
  fetchMeInvoices,
  fetchMePayments,
  fetchMeSubscription
} from '$lib/server/api';

export const load: PageServerLoad = async ({ locals, request }) => {
  const cookieHeader = request.headers.get('cookie');
  const user = locals.user!;

  if (!user.accountId && user.role !== 'admin') {
    return {
      user,
      noAccount: true as const,
      account: null,
      devices: [],
      subscription: null,
      invoices: [],
      payments: []
    };
  }

  const [account, devices, subscription, invoices, payments] = await Promise.all([
    fetchMeAccount(cookieHeader),
    fetchMeDevices(cookieHeader),
    fetchMeSubscription(cookieHeader),
    fetchMeInvoices(cookieHeader),
    fetchMePayments(cookieHeader)
  ]);

  return {
    user,
    noAccount: false as const,
    account,
    devices: devices ?? [],
    subscription: subscription ?? null,
    invoices: invoices ?? [],
    payments: payments ?? []
  };
};

export const actions: Actions = {
  privacyRequest: async ({ request, locals }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const requestType = String(form.get('requestType') ?? 'other').trim();
    const details = String(form.get('details') ?? '').trim();
    const requesterEmail =
      String(form.get('requesterEmail') ?? '').trim() || locals.user?.email || '';
    const requesterName =
      String(form.get('requesterName') ?? '').trim() || locals.user?.fullName || '';

    if (!requesterEmail) {
      return fail(400, { privacyError: 'Email is required.', details });
    }

    const result = await createMePrivacyRequest(cookieHeader, {
      requesterEmail,
      requestType,
      ...(requesterName ? { requesterName } : {}),
      ...(details ? { details } : {})
    });

    if (!result.ok) {
      return fail(result.status, { privacyError: result.message, details });
    }

    return { privacySuccess: true };
  }
};
