import type { Actions, PageServerLoad } from './$types';
import { fail } from '@sveltejs/kit';
import { createSignupRequest } from '$lib/server/api';

export const load: PageServerLoad = async () => ({});

export const actions: Actions = {
  default: async ({ request }) => {
    const form = await request.formData();
    const fullName = String(form.get('fullName') ?? '').trim();
    const companyName = String(form.get('companyName') ?? '').trim();
    const email = String(form.get('email') ?? '').trim();
    const phone = String(form.get('phone') ?? '').trim();
    const estimatedDevicesRaw = String(form.get('estimatedDevices') ?? '').trim();
    const message = String(form.get('message') ?? '').trim();

    if (!fullName || !companyName || !email) {
      return fail(400, {
        error: 'Full name, company name, and email are required.',
        fullName,
        companyName,
        email,
        phone,
        estimatedDevices: estimatedDevicesRaw,
        message
      });
    }

    const privacyAccepted = form.get('privacyAccepted') === 'on' || form.get('privacyAccepted') === 'true';
    if (!privacyAccepted) {
      return fail(400, {
        error: 'Please accept the privacy notice to continue.',
        fullName,
        companyName,
        email,
        phone,
        estimatedDevices: estimatedDevicesRaw,
        message
      });
    }

    const estimatedDevices = estimatedDevicesRaw ? Number(estimatedDevicesRaw) : undefined;
    if (estimatedDevicesRaw && (!Number.isFinite(estimatedDevices) || estimatedDevices! < 0)) {
      return fail(400, {
        error: 'Estimated devices must be a non-negative number.',
        fullName,
        companyName,
        email,
        phone,
        estimatedDevices: estimatedDevicesRaw,
        message
      });
    }

    const result = await createSignupRequest({
      fullName,
      companyName,
      email,
      privacyAccepted: true,
      ...(phone ? { phone } : {}),
      ...(estimatedDevices !== undefined ? { estimatedDevices } : {}),
      ...(message ? { message } : {})
    });

    if (!result.ok) {
      return fail(result.status, {
        error: result.message,
        fullName,
        companyName,
        email,
        phone,
        estimatedDevices: estimatedDevicesRaw,
        message
      });
    }

    return { success: true };
  }
};
