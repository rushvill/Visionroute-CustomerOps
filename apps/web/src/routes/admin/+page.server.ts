import type { Actions, PageServerLoad } from './$types';
import { error, fail } from '@sveltejs/kit';
import {
  approveSignup,
  assignSim,
  createAdminInvoice,
  createAdminPayment,
  createAdminSim,
  createAdminSimDataCost,
  fetchAdminAccounts,
  fetchAdminBillingSummary,
  fetchAdminCoverage,
  fetchAdminInvoices,
  fetchAdminPayments,
  fetchAdminPrivacyRequests,
  fetchAdminSignups,
  fetchAdminSimDataCosts,
  fetchAdminSims,
  fetchAdminSubscriptions,
  fetchAdminTickets,
  fetchAdminUsers,
  fetchAuditEvents,
  patchAdminPrivacyRequest,
  rejectSignup
} from '$lib/server/api';
import { pesosToCents } from '$lib/format';

export const load: PageServerLoad = async ({ request }) => {
  const cookieHeader = request.headers.get('cookie');

  const [
    users,
    auditEvents,
    signups,
    accounts,
    sims,
    coverage,
    tickets,
    billingSummary,
    invoices,
    payments,
    simDataCosts,
    subscriptions,
    privacyRequests
  ] = await Promise.all([
    fetchAdminUsers(cookieHeader),
    fetchAuditEvents(cookieHeader),
    fetchAdminSignups(cookieHeader),
    fetchAdminAccounts(cookieHeader),
    fetchAdminSims(cookieHeader),
    fetchAdminCoverage(cookieHeader, 90),
    fetchAdminTickets(cookieHeader),
    fetchAdminBillingSummary(cookieHeader),
    fetchAdminInvoices(cookieHeader),
    fetchAdminPayments(cookieHeader),
    fetchAdminSimDataCosts(cookieHeader),
    fetchAdminSubscriptions(cookieHeader),
    fetchAdminPrivacyRequests(cookieHeader)
  ]);

  if (!users || !auditEvents) {
    throw error(403, 'Admin access required');
  }

  const newSignups = (signups ?? []).filter((s) => s.status === 'new');
  const openInvoices = (invoices ?? []).filter((i) =>
    ['sent', 'partial', 'overdue'].includes(i.status)
  );
  const openPrivacy = (privacyRequests ?? []).filter((r) =>
    ['received', 'in_progress'].includes(r.status)
  );

  return {
    users,
    auditEvents,
    newSignups,
    accounts: accounts ?? [],
    sims: sims ?? [],
    coverage: coverage ?? [],
    tickets: tickets ?? [],
    billingSummary: billingSummary ?? {
      openOwedCents: 0,
      paymentsReceivedCents: 0,
      simDataCostCents: 0
    },
    invoices: invoices ?? [],
    openInvoices,
    payments: payments ?? [],
    simDataCosts: simDataCosts ?? [],
    subscriptions: subscriptions ?? [],
    privacyRequests: privacyRequests ?? [],
    openPrivacy
  };
};

export const actions: Actions = {
  approveSignup: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const id = String(form.get('id') ?? '');
    const username = String(form.get('username') ?? '').trim();
    const password = String(form.get('password') ?? '');

    if (!id || !username || !password) {
      return fail(400, { approveError: 'Signup id, username, and password are required.' });
    }

    const result = await approveSignup(cookieHeader, id, { username, password });
    if (!result.ok) {
      return fail(result.status, { approveError: result.message });
    }
    return { approveSuccess: true };
  },

  rejectSignup: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const id = String(form.get('id') ?? '');
    const reason = String(form.get('reason') ?? '').trim();

    if (!id || !reason) {
      return fail(400, { rejectError: 'Signup id and reason are required.' });
    }

    const result = await rejectSignup(cookieHeader, id, reason);
    if (!result.ok) {
      return fail(result.status, { rejectError: result.message });
    }
    return { rejectSuccess: true };
  },

  createSim: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const msisdn = String(form.get('msisdn') ?? '').trim();
    const iccid = String(form.get('iccid') ?? '').trim();

    if (!msisdn && !iccid) {
      return fail(400, { simError: 'MSISDN or ICCID is required.', msisdn, iccid });
    }

    const result = await createAdminSim(cookieHeader, {
      ...(msisdn ? { msisdn } : {}),
      ...(iccid ? { iccid } : {})
    });

    if (!result.ok) {
      return fail(result.status, { simError: result.message, msisdn, iccid });
    }
    return { simCreated: true };
  },

  assignSim: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const simId = String(form.get('simId') ?? '').trim();
    const accountId = String(form.get('accountId') ?? '').trim();

    if (!simId || !accountId) {
      return fail(400, { assignError: 'SIM id and account id are required.', simId, accountId });
    }

    const result = await assignSim(cookieHeader, simId, { accountId });
    if (!result.ok) {
      return fail(result.status, { assignError: result.message, simId, accountId });
    }
    return { assignSuccess: true };
  },

  createInvoice: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const accountId = String(form.get('accountId') ?? '').trim();
    const description = String(form.get('description') ?? '').trim();
    const amountPesos = String(form.get('amountPesos') ?? '').trim();
    const dueDate = String(form.get('dueDate') ?? '').trim();
    const amountCents = pesosToCents(amountPesos);

    if (!accountId || !description || amountCents == null) {
      return fail(400, {
        invoiceError: 'Account, description, and amount are required.',
        accountId,
        description,
        amountPesos,
        dueDate
      });
    }

    const result = await createAdminInvoice(cookieHeader, {
      accountId,
      description,
      amountCents,
      ...(dueDate ? { dueDate } : {})
    });
    if (!result.ok) {
      return fail(result.status, {
        invoiceError: result.message,
        accountId,
        description,
        amountPesos,
        dueDate
      });
    }
    return { invoiceSuccess: true };
  },

  createPayment: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const accountId = String(form.get('accountId') ?? '').trim();
    const invoiceId = String(form.get('invoiceId') ?? '').trim();
    const amountPesos = String(form.get('amountPesos') ?? '').trim();
    const method = String(form.get('method') ?? 'cash').trim();
    const reference = String(form.get('reference') ?? '').trim();
    const amountCents = pesosToCents(amountPesos);

    if (!accountId || amountCents == null || amountCents <= 0) {
      return fail(400, {
        paymentError: 'Account and a positive amount are required.',
        accountId,
        invoiceId,
        amountPesos,
        method,
        reference
      });
    }

    const result = await createAdminPayment(cookieHeader, {
      accountId,
      amountCents,
      method,
      ...(invoiceId ? { invoiceId } : {}),
      ...(reference ? { reference } : {})
    });
    if (!result.ok) {
      return fail(result.status, {
        paymentError: result.message,
        accountId,
        invoiceId,
        amountPesos,
        method,
        reference
      });
    }
    return { paymentSuccess: true };
  },

  createSimDataCost: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const accountId = String(form.get('accountId') ?? '').trim();
    const simCardId = String(form.get('simCardId') ?? '').trim();
    const description = String(form.get('description') ?? '').trim();
    const amountPesos = String(form.get('amountPesos') ?? '').trim();
    const carrier = String(form.get('carrier') ?? '').trim();
    const amountCents = pesosToCents(amountPesos);

    if (!description || amountCents == null) {
      return fail(400, {
        simCostError: 'Description and amount are required.',
        accountId,
        simCardId,
        description,
        amountPesos,
        carrier
      });
    }

    const result = await createAdminSimDataCost(cookieHeader, {
      description,
      amountCents,
      ...(accountId ? { accountId } : {}),
      ...(simCardId ? { simCardId } : {}),
      ...(carrier ? { carrier } : {})
    });
    if (!result.ok) {
      return fail(result.status, {
        simCostError: result.message,
        accountId,
        simCardId,
        description,
        amountPesos,
        carrier
      });
    }
    return { simCostSuccess: true };
  },

  updatePrivacyRequest: async ({ request }) => {
    const cookieHeader = request.headers.get('cookie');
    const form = await request.formData();
    const id = String(form.get('id') ?? '').trim();
    const status = String(form.get('status') ?? '').trim();
    const resolutionNotes = String(form.get('resolutionNotes') ?? '').trim();

    if (!id || !status) {
      return fail(400, { privacyError: 'Request id and status are required.' });
    }

    const result = await patchAdminPrivacyRequest(cookieHeader, id, {
      status,
      ...(resolutionNotes ? { resolutionNotes } : {})
    });
    if (!result.ok) {
      return fail(result.status, { privacyError: result.message });
    }
    return { privacySuccess: true };
  }
};
