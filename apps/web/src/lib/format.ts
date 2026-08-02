/** Human-readable status labels for Route Command UI */

const STATUS_LABELS: Record<string, string> = {
  active: 'Active',
  pending: 'Pending',
  pending_install: 'Pending install',
  inactive: 'Inactive',
  retired: 'Retired',
  inventory: 'Inventory',
  assigned: 'Assigned',
  suspended: 'Suspended',
  exhausted: 'Exhausted',
  open: 'Open',
  in_progress: 'In progress',
  waiting_customer: 'Waiting for customer',
  resolved: 'Resolved',
  closed: 'Closed',
  new: 'New',
  reviewing: 'Reviewing',
  approved: 'Approved',
  rejected: 'Rejected',
  trial: 'Trial',
  past_due: 'Past due',
  paused: 'Paused',
  cancelled: 'Cancelled',
  expired: 'Expired',
  churned: 'Churned',
  draft: 'Draft',
  sent: 'Sent',
  partial: 'Partial',
  paid: 'Paid',
  overdue: 'Overdue',
  cash: 'Cash',
  bank_transfer: 'Bank transfer',
  gcash: 'GCash',
  maya: 'Maya',
  other: 'Other',
  access: 'Access',
  rectification: 'Correction',
  erasure: 'Deletion',
  restriction: 'Restriction',
  portability: 'Portability',
  objection: 'Objection',
  received: 'Received',
  completed: 'Completed'
};

export function humanStatus(value: string | null | undefined): string {
  if (!value) return '—';
  return STATUS_LABELS[value] ?? value.replaceAll('_', ' ');
}

export function statusTone(
  value: string | null | undefined
): 'neutral' | 'success' | 'warning' | 'danger' | 'info' {
  switch (value) {
    case 'active':
    case 'approved':
    case 'resolved':
    case 'assigned':
    case 'paid':
      return 'success';
    case 'pending':
    case 'pending_install':
    case 'new':
    case 'reviewing':
    case 'waiting_customer':
    case 'in_progress':
    case 'inventory':
    case 'sent':
    case 'partial':
    case 'draft':
    case 'received':
    case 'in_progress':
      return 'warning';
    case 'rejected':
    case 'expired':
    case 'exhausted':
    case 'suspended':
    case 'past_due':
    case 'churned':
    case 'overdue':
    case 'cancelled':
      return 'danger';
    case 'open':
    case 'completed':
      return 'info';
    default:
      return 'neutral';
  }
}

export function fmtDate(value: string | null | undefined): string {
  if (!value) return '—';
  return new Date(value).toLocaleDateString(undefined, {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  });
}

export function maskId(value: string | null | undefined, keep = 4): string {
  if (!value) return '—';
  if (value.length <= keep * 2) return value;
  return `${value.slice(0, keep)}••••${value.slice(-keep)}`;
}

/** Format integer cents as PHP currency for small-business billing UI. */
export function fmtMoney(cents: number | null | undefined, currency = 'PHP'): string {
  if (cents == null || Number.isNaN(cents)) return '—';
  const amount = cents / 100;
  try {
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency,
      maximumFractionDigits: 2
    }).format(amount);
  } catch {
    return `${currency} ${amount.toFixed(2)}`;
  }
}

/** Parse a peso amount string (e.g. "4500" or "4500.50") into cents. */
export function pesosToCents(raw: string): number | null {
  const normalized = raw.trim().replace(/,/g, '');
  if (!normalized) return null;
  const value = Number(normalized);
  if (!Number.isFinite(value) || value < 0) return null;
  return Math.round(value * 100);
}
