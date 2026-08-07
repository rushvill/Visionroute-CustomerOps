import { env as privateEnv } from '$env/dynamic/private';
import { env as publicEnv } from '$env/dynamic/public';

export type MeUser = {
  id: string;
  username: string;
  email: string;
  fullName: string;
  role: string;
  accountId: string | null;
};

export type AuditEvent = {
  id: string;
  actorUserId: string | null;
  entityType: string;
  entityId: string;
  action: string;
  summary: string;
  createdAt: string;
};

export type SignupRequest = {
  id: string;
  status: string;
  fullName: string;
  companyName: string;
  email: string;
  phone: string | null;
  requestedUsername: string | null;
  estimatedDevices: number | null;
  message: string | null;
  preferredContact: string | null;
  reviewedAt: string | null;
  rejectionReason: string | null;
  convertedAccountId: string | null;
  createdAt: string;
};

export type Account = {
  id: string;
  accountCode: string;
  companyName: string;
  displayName: string | null;
  status: string;
  industry: string | null;
  billingEmail: string | null;
  operationsEmail: string | null;
  phone: string | null;
  city: string | null;
  province: string | null;
  country: string;
  source: string | null;
  approvedAt: string | null;
  createdAt: string;
};

export type Device = {
  id: string;
  accountId: string;
  name: string;
  plateNumber: string | null;
  imei: string | null;
  provider: string;
  status: string;
  installDate: string | null;
  createdAt: string;
};

export type SimAdmin = {
  id: string;
  carrier: string;
  iccid: string | null;
  msisdn: string | null;
  simLabel: string | null;
  status: string;
  purchaseDate: string | null;
  purchaseCostCents: number | null;
  dataPlanLabel: string | null;
  accountId: string | null;
  deviceId: string | null;
  activatedAt: string | null;
  createdAt: string;
};

export type SimCustomer = {
  id: string;
  carrier: string;
  iccid: string | null;
  msisdn: string | null;
  simLabel: string | null;
  status: string;
  dataPlanLabel: string | null;
  accountId: string | null;
  deviceId: string | null;
  activatedAt: string | null;
  createdAt: string;
};

export type SubscriptionCustomer = {
  id: string;
  accountId: string;
  planId: string;
  planName?: string;
  status: string;
  startsAt: string;
  endsAt: string | null;
  dataCoverageStartsAt: string | null;
  dataCoverageEndsAt: string | null;
  currency: string;
  createdAt: string;
};

export type SubscriptionAdmin = SubscriptionCustomer & {
  amountCents: number | null;
  notes: string | null;
};

export type Invoice = {
  id: string;
  accountId: string;
  number: string;
  description: string;
  amountCents: number;
  currency: string;
  status: string;
  issuedAt: string;
  dueDate: string | null;
  paidAt: string | null;
  notes: string | null;
  createdAt: string;
};

export type Payment = {
  id: string;
  accountId: string;
  invoiceId: string | null;
  amountCents: number;
  currency: string;
  method: string;
  paidAt: string;
  reference: string | null;
  notes: string | null;
  createdAt: string;
};

export type SimDataCost = {
  id: string;
  accountId: string | null;
  simCardId: string | null;
  amountCents: number;
  currency: string;
  carrier: string | null;
  description: string;
  periodStart: string | null;
  periodEnd: string | null;
  paidAt: string;
  notes: string | null;
  createdAt: string;
};

export type BillingSummary = {
  openOwedCents: number;
  paymentsReceivedCents: number;
  simDataCostCents: number;
};

export type Ticket = {
  id: string;
  accountId: string;
  number: string;
  createdByUserId: string;
  assignedToUserId: string | null;
  deviceId: string | null;
  subject: string;
  description: string | null;
  status: string;
  priority: string;
  category: string;
  createdAt: string;
};

export type CreateSignupBody = {
  fullName: string;
  companyName: string;
  email: string;
  phone?: string;
  estimatedDevices?: number;
  message?: string;
  privacyAccepted: boolean;
};

export type CreateSimBody = {
  msisdn?: string;
  iccid?: string;
  simLabel?: string;
};

export type CreateTicketBody = {
  subject: string;
  description?: string;
};

export type ApiResult<T> =
  | { ok: true; data: T }
  | { ok: false; status: number; message: string };

export function apiBaseUrl(): string {
  return (
    privateEnv.BACKEND_API_BASE_URL ||
    publicEnv.PUBLIC_API_BASE_URL ||
    'http://127.0.0.1:8080'
  ).replace(/\/$/, '');
}

export function frontendOrigin(): string {
  return privateEnv.FRONTEND_ORIGIN || 'http://127.0.0.1:5173';
}

/** Forward Set-Cookie headers from the API onto the SvelteKit response cookies. */
export function forwardSetCookies(
  response: Response,
  cookies: import('@sveltejs/kit').Cookies
): void {
  const raw =
    typeof response.headers.getSetCookie === 'function' ? response.headers.getSetCookie() : [];

  const fallback = response.headers.get('set-cookie');
  const lines = raw.length > 0 ? raw : fallback ? [fallback] : [];

  for (const line of lines) {
    const [pair] = line.split(';');
    const eq = pair.indexOf('=');
    if (eq <= 0) continue;
    const name = pair.slice(0, eq).trim();
    const value = pair.slice(eq + 1).trim();
    const httpOnly = /httponly/i.test(line);
    const secure = /secure/i.test(line);
    const pathMatch = line.match(/path=([^;]+)/i);
    const path = pathMatch?.[1]?.trim() || '/';
    const maxAgeMatch = line.match(/max-age=(\d+)/i);
    const maxAge = maxAgeMatch ? Number(maxAgeMatch[1]) : undefined;

    if (!value || maxAge === 0) {
      cookies.delete(name, { path });
      continue;
    }

    cookies.set(name, value, {
      path,
      httpOnly,
      secure,
      sameSite: 'lax',
      maxAge
    });
  }
}

function readHeaders(cookieHeader: string | null, mutating = false): HeadersInit {
  return {
    Accept: 'application/json',
    ...(cookieHeader ? { Cookie: cookieHeader } : {}),
    ...(mutating ? { Origin: frontendOrigin() } : {})
  };
}

const API_TIMEOUT_MS = 3_000;

function apiSignal(): AbortSignal {
  // Prefer AbortSignal.timeout when available; fall back for older runtimes.
  if (typeof AbortSignal !== 'undefined' && typeof AbortSignal.timeout === 'function') {
    return AbortSignal.timeout(API_TIMEOUT_MS);
  }
  const controller = new AbortController();
  setTimeout(() => controller.abort(), API_TIMEOUT_MS);
  return controller.signal;
}

async function apiErrorMessage(response: Response): Promise<string> {
  try {
    const body = (await response.json()) as { error?: { message?: string } };
    if (body.error?.message) return body.error.message;
  } catch {
    /* keep default */
  }
  return `Request failed (${response.status})`;
}

async function apiGet<T>(path: string, cookieHeader: string | null): Promise<T | null> {
  try {
    const response = await fetch(`${apiBaseUrl()}${path}`, {
      headers: readHeaders(cookieHeader),
      signal: apiSignal()
    });
    if (!response.ok) return null;
    return (await response.json()) as T;
  } catch {
    return null;
  }
}

async function apiPost<T>(
  path: string,
  cookieHeader: string | null,
  body?: unknown
): Promise<ApiResult<T>> {
  try {
    const response = await fetch(`${apiBaseUrl()}${path}`, {
      method: 'POST',
      headers: {
        ...readHeaders(cookieHeader, true),
        ...(body !== undefined ? { 'Content-Type': 'application/json' } : {})
      },
      ...(body !== undefined ? { body: JSON.stringify(body) } : {}),
      signal: apiSignal()
    });
    if (!response.ok) {
      return { ok: false, status: response.status, message: await apiErrorMessage(response) };
    }
    const data = body !== undefined ? ((await response.json()) as T) : (undefined as T);
    return { ok: true, data };
  } catch {
    return { ok: false, status: 503, message: 'API is not reachable.' };
  }
}

export async function fetchMe(cookieHeader: string | null): Promise<MeUser | null> {
  // Anonymous visits should not wait on a cold/missing API.
  if (!cookieHeader?.includes('vr_ops_session=')) return null;
  return apiGet<MeUser>('/api/v1/me', cookieHeader);
}

export async function fetchAdminUsers(cookieHeader: string | null): Promise<MeUser[] | null> {
  return apiGet<MeUser[]>('/api/v1/admin/users', cookieHeader);
}

export async function fetchAuditEvents(
  cookieHeader: string | null
): Promise<AuditEvent[] | null> {
  return apiGet<AuditEvent[]>('/api/v1/admin/audit-events?limit=30', cookieHeader);
}

export async function createSignupRequest(body: CreateSignupBody): Promise<ApiResult<SignupRequest>> {
  return apiPost<SignupRequest>('/api/v1/signup-requests', null, body);
}

export async function fetchAdminSignups(
  cookieHeader: string | null
): Promise<SignupRequest[] | null> {
  return apiGet<SignupRequest[]>('/api/v1/admin/signup-requests', cookieHeader);
}

export async function approveSignup(
  cookieHeader: string | null,
  id: string,
  body: { username: string; password: string; planCode?: string }
): Promise<ApiResult<{ signup: SignupRequest; accountId: string; userId: string }>> {
  return apiPost(`/api/v1/admin/signup-requests/${id}/approve`, cookieHeader, body);
}

export async function rejectSignup(
  cookieHeader: string | null,
  id: string,
  reason: string
): Promise<ApiResult<SignupRequest>> {
  return apiPost(`/api/v1/admin/signup-requests/${id}/reject`, cookieHeader, { reason });
}

export async function fetchAdminAccounts(cookieHeader: string | null): Promise<Account[] | null> {
  return apiGet<Account[]>('/api/v1/admin/accounts', cookieHeader);
}

export async function fetchAdminSims(cookieHeader: string | null): Promise<SimAdmin[] | null> {
  return apiGet<SimAdmin[]>('/api/v1/admin/sims', cookieHeader);
}

export async function fetchAdminCoverage(
  cookieHeader: string | null,
  withinDays = 90
): Promise<SubscriptionAdmin[] | null> {
  return apiGet<SubscriptionAdmin[]>(
    `/api/v1/admin/coverage-expiring?withinDays=${withinDays}`,
    cookieHeader
  );
}

export async function fetchAdminTickets(cookieHeader: string | null): Promise<Ticket[] | null> {
  return apiGet<Ticket[]>('/api/v1/admin/tickets', cookieHeader);
}

export async function createAdminSim(
  cookieHeader: string | null,
  body: CreateSimBody
): Promise<ApiResult<SimAdmin>> {
  return apiPost<SimAdmin>('/api/v1/admin/sims', cookieHeader, body);
}

export async function assignSim(
  cookieHeader: string | null,
  id: string,
  body: { accountId: string; deviceId?: string }
): Promise<ApiResult<SimAdmin>> {
  return apiPost<SimAdmin>(`/api/v1/admin/sims/${id}/assign`, cookieHeader, body);
}

export async function fetchMeAccount(cookieHeader: string | null): Promise<Account | null> {
  return apiGet<Account>('/api/v1/me/account', cookieHeader);
}

export async function fetchMeDevices(cookieHeader: string | null): Promise<Device[] | null> {
  return apiGet<Device[]>('/api/v1/me/devices', cookieHeader);
}

export async function fetchMeSims(cookieHeader: string | null): Promise<SimCustomer[] | null> {
  return apiGet<SimCustomer[]>('/api/v1/me/sims', cookieHeader);
}

export async function fetchMeSubscription(
  cookieHeader: string | null
): Promise<SubscriptionCustomer | null | undefined> {
  try {
    const response = await fetch(`${apiBaseUrl()}/api/v1/me/subscription`, {
      headers: readHeaders(cookieHeader),
      signal: apiSignal()
    });
    if (!response.ok) return null;
    return (await response.json()) as SubscriptionCustomer | null;
  } catch {
    return null;
  }
}

export async function fetchMeTickets(cookieHeader: string | null): Promise<Ticket[] | null> {
  return apiGet<Ticket[]>('/api/v1/me/tickets', cookieHeader);
}

export async function createMeTicket(
  cookieHeader: string | null,
  body: CreateTicketBody
): Promise<ApiResult<Ticket>> {
  return apiPost<Ticket>('/api/v1/me/tickets', cookieHeader, body);
}

export async function fetchAdminBillingSummary(
  cookieHeader: string | null
): Promise<BillingSummary | null> {
  return apiGet<BillingSummary>('/api/v1/admin/billing/summary', cookieHeader);
}

export async function fetchAdminInvoices(cookieHeader: string | null): Promise<Invoice[] | null> {
  return apiGet<Invoice[]>('/api/v1/admin/invoices', cookieHeader);
}

export async function fetchAdminPayments(cookieHeader: string | null): Promise<Payment[] | null> {
  return apiGet<Payment[]>('/api/v1/admin/payments', cookieHeader);
}

export async function fetchAdminSimDataCosts(
  cookieHeader: string | null
): Promise<SimDataCost[] | null> {
  return apiGet<SimDataCost[]>('/api/v1/admin/sim-data-costs', cookieHeader);
}

export async function fetchAdminSubscriptions(
  cookieHeader: string | null
): Promise<SubscriptionAdmin[] | null> {
  return apiGet<SubscriptionAdmin[]>('/api/v1/admin/subscriptions', cookieHeader);
}

export async function createAdminInvoice(
  cookieHeader: string | null,
  body: {
    accountId: string;
    description: string;
    amountCents: number;
    dueDate?: string;
    notes?: string;
  }
): Promise<ApiResult<Invoice>> {
  return apiPost<Invoice>('/api/v1/admin/invoices', cookieHeader, body);
}

export async function createAdminPayment(
  cookieHeader: string | null,
  body: {
    accountId: string;
    invoiceId?: string;
    amountCents: number;
    method?: string;
    reference?: string;
    notes?: string;
  }
): Promise<ApiResult<Payment>> {
  return apiPost<Payment>('/api/v1/admin/payments', cookieHeader, body);
}

export async function createAdminSimDataCost(
  cookieHeader: string | null,
  body: {
    accountId?: string;
    simCardId?: string;
    amountCents: number;
    carrier?: string;
    description: string;
    periodStart?: string;
    periodEnd?: string;
    paidAt?: string;
    notes?: string;
  }
): Promise<ApiResult<SimDataCost>> {
  return apiPost<SimDataCost>('/api/v1/admin/sim-data-costs', cookieHeader, body);
}

export async function fetchMeInvoices(cookieHeader: string | null): Promise<Invoice[] | null> {
  return apiGet<Invoice[]>('/api/v1/me/invoices', cookieHeader);
}

export async function fetchMePayments(cookieHeader: string | null): Promise<Payment[] | null> {
  return apiGet<Payment[]>('/api/v1/me/payments', cookieHeader);
}

export type PrivacyRequest = {
  id: string;
  accountId: string | null;
  requesterName: string | null;
  requesterEmail: string;
  requestType: string;
  details: string | null;
  status: string;
  handledAt: string | null;
  resolutionNotes: string | null;
  createdAt: string;
};

export async function createPrivacyRequest(body: {
  requesterName?: string;
  requesterEmail: string;
  requestType: string;
  details?: string;
}): Promise<ApiResult<PrivacyRequest>> {
  return apiPost<PrivacyRequest>('/api/v1/privacy-requests', null, body);
}

export async function createMePrivacyRequest(
  cookieHeader: string | null,
  body: {
    requesterName?: string;
    requesterEmail: string;
    requestType: string;
    details?: string;
  }
): Promise<ApiResult<PrivacyRequest>> {
  return apiPost<PrivacyRequest>('/api/v1/me/privacy-requests', cookieHeader, body);
}

export async function fetchAdminPrivacyRequests(
  cookieHeader: string | null
): Promise<PrivacyRequest[] | null> {
  return apiGet<PrivacyRequest[]>('/api/v1/admin/privacy-requests', cookieHeader);
}

export async function patchAdminPrivacyRequest(
  cookieHeader: string | null,
  id: string,
  body: { status: string; resolutionNotes?: string }
): Promise<ApiResult<PrivacyRequest>> {
  try {
    const response = await fetch(`${apiBaseUrl()}/api/v1/admin/privacy-requests/${id}`, {
      method: 'PATCH',
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        Origin: frontendOrigin(),
        ...(cookieHeader ? { Cookie: cookieHeader } : {})
      },
      body: JSON.stringify(body),
      signal: apiSignal()
    });
    if (!response.ok) {
      return { ok: false, status: response.status, message: await apiErrorMessage(response) };
    }
    return { ok: true, data: (await response.json()) as PrivacyRequest };
  } catch {
    return { ok: false, status: 503, message: 'API is not reachable.' };
  }
}
