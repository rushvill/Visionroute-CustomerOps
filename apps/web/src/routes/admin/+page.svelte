<script lang="ts">
  import type { ActionData } from './$types';
  import Button from '$lib/components/ui/Button.svelte';
  import Field from '$lib/components/ui/Field.svelte';
  import StatusPill from '$lib/components/ui/StatusPill.svelte';
  import Alert from '$lib/components/ui/Alert.svelte';
  import PageHeader from '$lib/components/ui/PageHeader.svelte';
  import { fmtDate, fmtMoney, humanStatus, maskId, statusTone } from '$lib/format';

  let { data, form }: { data: import('./$types').PageData; form: ActionData } = $props();

  const openTickets = $derived(
    data.tickets.filter((t) => ['open', 'in_progress', 'waiting_customer'].includes(t.status)).length
  );
  const inventorySims = $derived(data.sims.filter((s) => s.status === 'inventory').length);

  function accountLabel(accountId: string | null | undefined): string {
    if (!accountId) return '—';
    const account = data.accounts.find((a) => a.id === accountId);
    return account ? account.companyName : maskId(accountId, 4);
  }
</script>

<main id="main" class="admin-main">
  <section id="overview" class="panel">
    <PageHeader
      eyebrow="Customer Operations"
      title="Needs attention"
      description="Review new signups, call prospects, then approve and create customer access when ready."
    />
    <div class="attention">
      <a href="#signups"><strong>{data.newSignups.length}</strong><span>Signup requests</span></a>
      <a href="#billing"><strong>{data.openInvoices.length}</strong><span>Open invoices</span></a>
      <a href="#coverage"><strong>{data.coverage.length}</strong><span>Coverage nearing expiry</span></a>
      <a href="#tickets"><strong>{openTickets}</strong><span>Open tickets</span></a>
      <a href="#sims"><strong>{inventorySims}</strong><span>SIMs in inventory</span></a>
      <a href="#privacy"><strong>{data.openPrivacy.length}</strong><span>Privacy requests</span></a>
    </div>
    <div class="money-strip">
      <div>
        <span>Customers still owe</span>
        <strong>{fmtMoney(data.billingSummary.openOwedCents)}</strong>
      </div>
      <div>
        <span>Payments received</span>
        <strong>{fmtMoney(data.billingSummary.paymentsReceivedCents)}</strong>
      </div>
      <div>
        <span>SIM data costs paid</span>
        <strong>{fmtMoney(data.billingSummary.simDataCostCents)}</strong>
      </div>
    </div>
  </section>

  <section id="signups" class="panel">
    <h2>Signup inbox</h2>
    <p class="inbox-hint">Call the prospect first. When ready, approve to create their customer login.</p>
    {#if form?.approveSuccess}
      <Alert tone="success" title="Signup approved" />
    {/if}
    {#if form?.rejectSuccess}
      <Alert tone="success" title="Signup rejected" />
    {/if}
    {#if form?.approveError}
      <Alert tone="danger" title="Approval failed"><p>{form.approveError}</p></Alert>
    {/if}
    {#if form?.rejectError}
      <Alert tone="danger" title="Rejection failed"><p>{form.rejectError}</p></Alert>
    {/if}

    {#if data.newSignups.length === 0}
      <p class="empty">No new signup requests.</p>
    {:else}
      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th>Prospect</th>
              <th>Company</th>
              <th>Contact</th>
              <th>Devices</th>
              <th>Submitted</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {#each data.newSignups as signup}
              <tr>
                <td data-label="Prospect"><strong>{signup.fullName}</strong></td>
                <td data-label="Company">{signup.companyName}</td>
                <td data-label="Contact">
                  {signup.email}
                  {#if signup.phone}<br /><span class="muted">{signup.phone}</span>{/if}
                </td>
                <td data-label="Devices">{signup.estimatedDevices ?? '—'}</td>
                <td data-label="Submitted">{fmtDate(signup.createdAt)}</td>
                <td data-label="Action">
                  <form method="POST" action="?/approveSignup" class="row-form">
                    <input type="hidden" name="id" value={signup.id} />
                    <input name="username" placeholder="Username" required />
                    <input name="password" type="password" placeholder="Temp password" required minlength="10" />
                    <Button variant="primary">Approve</Button>
                  </form>
                  <form method="POST" action="?/rejectSignup" class="row-form reject">
                    <input type="hidden" name="id" value={signup.id} />
                    <input name="reason" placeholder="Reject reason" required />
                    <Button variant="danger">Reject</Button>
                  </form>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </section>

  <section id="billing" class="panel">
    <h2>Customer bills &amp; payments</h2>
    <p class="inbox-hint">Track what customers owe and what they have paid VisionRoute.</p>
    {#if form?.invoiceSuccess}
      <Alert tone="success" title="Invoice created" />
    {/if}
    {#if form?.paymentSuccess}
      <Alert tone="success" title="Payment recorded" />
    {/if}
    {#if form?.invoiceError}
      <Alert tone="danger" title="Could not create invoice"><p>{form.invoiceError}</p></Alert>
    {/if}
    {#if form?.paymentError}
      <Alert tone="danger" title="Could not record payment"><p>{form.paymentError}</p></Alert>
    {/if}

    <div class="billing-forms">
      <form method="POST" action="?/createInvoice" class="inline-panel">
        <h3>Create invoice</h3>
        <div class="inline-fields stacked">
          <label>
            <span>Account</span>
            <select name="accountId" required>
              <option value="">Select account</option>
              {#each data.accounts as account}
                <option value={account.id} selected={form?.accountId === account.id}
                  >{account.companyName}</option
                >
              {/each}
            </select>
          </label>
          <Field label="Description" name="description" required value={form?.description ?? ''} />
          <Field
            label="Amount (PHP)"
            name="amountPesos"
            required
            value={form?.amountPesos ?? ''}
            hint="Example: 4500"
          />
          <Field label="Due date" name="dueDate" type="date" value={form?.dueDate ?? ''} />
          <Button variant="primary">Create invoice</Button>
        </div>
      </form>

      <form method="POST" action="?/createPayment" class="inline-panel">
        <h3>Record payment</h3>
        <div class="inline-fields stacked">
          <label>
            <span>Account</span>
            <select name="accountId" required>
              <option value="">Select account</option>
              {#each data.accounts as account}
                <option value={account.id}>{account.companyName}</option>
              {/each}
            </select>
          </label>
          <label>
            <span>Invoice (optional)</span>
            <select name="invoiceId">
              <option value="">No linked invoice</option>
              {#each data.openInvoices as invoice}
                <option value={invoice.id}
                  >{invoice.number} · {accountLabel(invoice.accountId)} · {fmtMoney(
                    invoice.amountCents
                  )}</option
                >
              {/each}
            </select>
          </label>
          <Field label="Amount (PHP)" name="amountPesos" required value="" />
          <label>
            <span>Method</span>
            <select name="method">
              <option value="cash">Cash</option>
              <option value="gcash">GCash</option>
              <option value="maya">Maya</option>
              <option value="bank_transfer">Bank transfer</option>
              <option value="other">Other</option>
            </select>
          </label>
          <Field label="Reference" name="reference" value="" hint="Receipt or transfer ref" />
          <Button variant="primary">Record payment</Button>
        </div>
      </form>
    </div>

    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Invoice</th>
            <th>Account</th>
            <th>Amount</th>
            <th>Status</th>
            <th>Due</th>
          </tr>
        </thead>
        <tbody>
          {#each data.invoices as invoice}
            <tr>
              <td data-label="Invoice">
                <strong>{invoice.number}</strong>
                <div class="muted">{invoice.description}</div>
              </td>
              <td data-label="Account">{accountLabel(invoice.accountId)}</td>
              <td data-label="Amount">{fmtMoney(invoice.amountCents, invoice.currency)}</td>
              <td data-label="Status"
                ><StatusPill tone={statusTone(invoice.status)} label={humanStatus(invoice.status)} /></td
              >
              <td data-label="Due">{fmtDate(invoice.dueDate)}</td>
            </tr>
          {:else}
            <tr><td colspan="5">No invoices yet.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>

    <h3 class="subhead">Recent payments</h3>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>When</th>
            <th>Account</th>
            <th>Amount</th>
            <th>Method</th>
            <th>Reference</th>
          </tr>
        </thead>
        <tbody>
          {#each data.payments as payment}
            <tr>
              <td data-label="When">{fmtDate(payment.paidAt)}</td>
              <td data-label="Account">{accountLabel(payment.accountId)}</td>
              <td data-label="Amount">{fmtMoney(payment.amountCents, payment.currency)}</td>
              <td data-label="Method">{humanStatus(payment.method)}</td>
              <td data-label="Reference">{payment.reference ?? '—'}</td>
            </tr>
          {:else}
            <tr><td colspan="5">No payments recorded yet.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="subscriptions" class="panel">
    <h2>Subscriptions</h2>
    <p class="inbox-hint">Monitor subscription start dates, amounts, and coverage windows.</p>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Account</th>
            <th>Status</th>
            <th>Starts</th>
            <th>Coverage ends</th>
            <th>Amount</th>
          </tr>
        </thead>
        <tbody>
          {#each data.subscriptions as sub}
            <tr>
              <td data-label="Account">{accountLabel(sub.accountId)}</td>
              <td data-label="Status"
                ><StatusPill tone={statusTone(sub.status)} label={humanStatus(sub.status)} /></td
              >
              <td data-label="Starts">{fmtDate(sub.startsAt)}</td>
              <td data-label="Coverage ends">{fmtDate(sub.dataCoverageEndsAt)}</td>
              <td data-label="Amount">{fmtMoney(sub.amountCents, sub.currency)}</td>
            </tr>
          {:else}
            <tr><td colspan="5">No subscriptions yet.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="accounts" class="panel">
    <h2>Accounts</h2>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Account</th>
            <th>Code</th>
            <th>Status</th>
            <th>Created</th>
          </tr>
        </thead>
        <tbody>
          {#each data.accounts as account}
            <tr>
              <td data-label="Account"><strong>{account.companyName}</strong></td>
              <td data-label="Code">{account.accountCode}</td>
              <td data-label="Status"
                ><StatusPill tone={statusTone(account.status)} label={humanStatus(account.status)} /></td
              >
              <td data-label="Created">{fmtDate(account.createdAt)}</td>
            </tr>
          {:else}
            <tr><td colspan="4">No accounts yet.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="sims" class="panel">
    <h2>SIM inventory</h2>
    {#if form?.simError}
      <Alert tone="danger" title="SIM action failed"><p>{form.simError}</p></Alert>
    {/if}
    {#if form?.simCreated}
      <Alert tone="success" title="SIM saved" />
    {/if}
    {#if form?.assignSuccess}
      <Alert tone="success" title="SIM assigned" />
    {/if}

    <form method="POST" action="?/createSim" class="inline-panel">
      <h3>Add SIM</h3>
      <div class="inline-fields">
        <Field label="MSISDN" name="msisdn" value={form?.msisdn ?? ''} />
        <Field label="ICCID" name="iccid" value={form?.iccid ?? ''} />
        <Button variant="primary">Add SIM</Button>
      </div>
    </form>

    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>SIM</th>
            <th>Status</th>
            <th>Account</th>
            <th>Assign</th>
          </tr>
        </thead>
        <tbody>
          {#each data.sims as sim}
            <tr>
              <td data-label="SIM">
                <strong>{maskId(sim.msisdn ?? sim.iccid, 4)}</strong>
                {#if sim.carrier}<span class="muted"> · {sim.carrier}</span>{/if}
              </td>
              <td data-label="Status"
                ><StatusPill tone={statusTone(sim.status)} label={humanStatus(sim.status)} /></td
              >
              <td data-label="Account">{sim.accountId ? maskId(sim.accountId, 4) : '—'}</td>
              <td data-label="Assign">
                {#if sim.status === 'inventory'}
                  <form method="POST" action="?/assignSim" class="row-form">
                    <input type="hidden" name="simId" value={sim.id} />
                    <input name="accountId" placeholder="Account UUID" required />
                    <Button variant="secondary">Assign</Button>
                  </form>
                {:else}
                  —
                {/if}
              </td>
            </tr>
          {:else}
            <tr><td colspan="4">No SIMs in inventory.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="sim-costs" class="panel">
    <h2>SIM data costs</h2>
    <p class="inbox-hint">What VisionRoute paid for SIM data loads (your cost, not customer invoices).</p>
    {#if form?.simCostSuccess}
      <Alert tone="success" title="SIM data cost recorded" />
    {/if}
    {#if form?.simCostError}
      <Alert tone="danger" title="Could not record cost"><p>{form.simCostError}</p></Alert>
    {/if}

    <form method="POST" action="?/createSimDataCost" class="inline-panel">
      <h3>Record SIM data cost</h3>
      <div class="inline-fields stacked">
        <label>
          <span>Account (optional)</span>
          <select name="accountId">
            <option value="">Unassigned / inventory</option>
            {#each data.accounts as account}
              <option value={account.id}>{account.companyName}</option>
            {/each}
          </select>
        </label>
        <label>
          <span>SIM (optional)</span>
          <select name="simCardId">
            <option value="">Not linked</option>
            {#each data.sims as sim}
              <option value={sim.id}
                >{sim.simLabel ?? maskId(sim.msisdn ?? sim.iccid, 4)} · {humanStatus(sim.status)}</option
              >
            {/each}
          </select>
        </label>
        <Field label="Description" name="description" required value={form?.description ?? ''} />
        <Field label="Amount paid (PHP)" name="amountPesos" required value={form?.amountPesos ?? ''} />
        <label>
          <span>Carrier</span>
          <select name="carrier">
            <option value="">—</option>
            <option value="smart">Smart</option>
            <option value="globe">Globe</option>
            <option value="tnt">TNT</option>
            <option value="other">Other</option>
          </select>
        </label>
        <Button variant="primary">Record cost</Button>
      </div>
    </form>

    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Paid</th>
            <th>Description</th>
            <th>Account</th>
            <th>Amount</th>
            <th>Carrier</th>
          </tr>
        </thead>
        <tbody>
          {#each data.simDataCosts as cost}
            <tr>
              <td data-label="Paid">{fmtDate(cost.paidAt)}</td>
              <td data-label="Description"><strong>{cost.description}</strong></td>
              <td data-label="Account">{accountLabel(cost.accountId)}</td>
              <td data-label="Amount">{fmtMoney(cost.amountCents, cost.currency)}</td>
              <td data-label="Carrier">{cost.carrier ? humanStatus(cost.carrier) : '—'}</td>
            </tr>
          {:else}
            <tr><td colspan="5">No SIM data costs recorded yet.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="coverage" class="panel">
    <h2>Coverage expiring</h2>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Account</th>
            <th>Plan</th>
            <th>Ends</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {#each data.coverage as row}
            <tr>
              <td data-label="Account">{maskId(row.accountId, 4)}</td>
              <td data-label="Plan">{row.planName ?? maskId(row.planId, 4)}</td>
              <td data-label="Ends">{fmtDate(row.dataCoverageEndsAt)}</td>
              <td data-label="Status"
                ><StatusPill tone="warning" label={humanStatus(row.status)} /></td
              >
            </tr>
          {:else}
            <tr><td colspan="4">No subscriptions expiring within the selected window.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="tickets" class="panel">
    <h2>Tickets</h2>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Number</th>
            <th>Subject</th>
            <th>Account</th>
            <th>Status</th>
              <th>Opened</th>
            </tr>
          </thead>
          <tbody>
            {#each data.tickets as ticket}
              <tr>
                <td data-label="Number">{ticket.number}</td>
                <td data-label="Subject"><strong>{ticket.subject}</strong></td>
                <td data-label="Account">{maskId(ticket.accountId, 4)}</td>
                <td data-label="Status"
                  ><StatusPill tone={statusTone(ticket.status)} label={humanStatus(ticket.status)} /></td
                >
                <td data-label="Opened">{fmtDate(ticket.createdAt)}</td>
            </tr>
          {:else}
            <tr><td colspan="5">No tickets.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="users" class="panel">
    <h2>Users</h2>
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Username</th>
            <th>Name</th>
            <th>Role</th>
            <th>Account</th>
          </tr>
        </thead>
        <tbody>
          {#each data.users as user}
            <tr>
              <td data-label="Username"><strong>{user.username}</strong></td>
              <td data-label="Name">{user.fullName}</td>
              <td data-label="Role"><StatusPill tone="neutral" label={user.role} /></td>
              <td data-label="Account">{user.accountId ? maskId(user.accountId, 4) : '—'}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="audit" class="panel">
    <h2>Audit events</h2>
    <div class="table-wrap">
      <table class="audit">
        <thead>
          <tr>
            <th>When</th>
            <th>Action</th>
            <th>Summary</th>
          </tr>
        </thead>
        <tbody>
          {#each data.auditEvents as event}
            <tr>
              <td data-label="When">{new Date(event.createdAt).toLocaleString()}</td>
              <td data-label="Action"><code>{event.action}</code></td>
              <td data-label="Summary">{event.summary}</td>
            </tr>
          {:else}
            <tr><td colspan="3">No audit events yet.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section id="privacy" class="panel">
    <h2>Privacy requests</h2>
    <p class="inbox-hint">
      GDPR / Data Privacy Act style requests (access, correction, deletion). Handle manually, then
      mark completed.
    </p>
    {#if form?.privacySuccess}
      <Alert tone="success" title="Privacy request updated" />
    {/if}
    {#if form?.privacyError}
      <Alert tone="danger" title="Update failed"><p>{form.privacyError}</p></Alert>
    {/if}
    <div class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Requester</th>
            <th>Type</th>
            <th>Status</th>
            <th>Submitted</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          {#each data.privacyRequests as req}
            <tr>
              <td data-label="Requester">
                <strong>{req.requesterName ?? '—'}</strong>
                <div class="muted">{req.requesterEmail}</div>
                {#if req.details}<div class="muted">{req.details}</div>{/if}
              </td>
              <td data-label="Type">{humanStatus(req.requestType)}</td>
              <td data-label="Status"
                ><StatusPill tone={statusTone(req.status)} label={humanStatus(req.status)} /></td
              >
              <td data-label="Submitted">{fmtDate(req.createdAt)}</td>
              <td data-label="Action">
                {#if ['received', 'in_progress'].includes(req.status)}
                  <form method="POST" action="?/updatePrivacyRequest" class="row-form">
                    <input type="hidden" name="id" value={req.id} />
                    <select name="status">
                      <option value="in_progress">In progress</option>
                      <option value="completed">Completed</option>
                      <option value="rejected">Rejected</option>
                    </select>
                    <input name="resolutionNotes" placeholder="Notes (optional)" />
                    <Button variant="secondary">Update</Button>
                  </form>
                {:else}
                  —
                {/if}
              </td>
            </tr>
          {:else}
            <tr><td colspan="5">No privacy requests yet.</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>
</main>

<style>
  .admin-main {
    padding: var(--space-8) clamp(1rem, 3vw, 2rem) var(--space-16);
  }
  .panel {
    margin-bottom: var(--space-12);
    scroll-margin-top: var(--space-6);
  }
  .panel h2 {
    font-size: var(--text-section);
    margin-bottom: var(--space-5);
  }
  .inbox-hint {
    margin: calc(var(--space-5) * -0.6) 0 var(--space-5);
    color: var(--color-text-secondary);
    font-size: var(--text-compact);
  }
  .attention {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(9.5rem, 1fr));
    gap: var(--space-4);
  }
  .attention a {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-5);
    text-decoration: none;
    color: inherit;
  }
  .attention strong {
    display: block;
    font-family: var(--font-display);
    font-size: 1.75rem;
    color: var(--color-brand-strong);
  }
  .attention span {
    color: var(--color-text-secondary);
    font-size: var(--text-compact);
  }
  .money-strip {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
    margin-top: var(--space-5);
  }
  .money-strip div {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-4) var(--space-5);
  }
  .money-strip span {
    display: block;
    color: var(--color-text-muted);
    font-size: var(--text-meta);
    margin-bottom: var(--space-1);
  }
  .money-strip strong {
    font-family: var(--font-display);
    font-size: 1.15rem;
    color: var(--color-brand-strong);
  }
  .billing-forms {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-5);
    margin-bottom: var(--space-5);
  }
  .inline-fields.stacked {
    grid-template-columns: 1fr;
  }
  .inline-fields label,
  .inline-fields.stacked label {
    display: grid;
    gap: var(--space-2);
    font-size: var(--text-compact);
  }
  .inline-fields label span {
    font-weight: 500;
  }
  .inline-fields select {
    min-height: 2.75rem;
    padding: 0.55rem 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-control);
    font: inherit;
    background: var(--color-surface);
  }
  .subhead {
    font-size: var(--text-panel);
    margin: var(--space-8) 0 var(--space-4);
  }
  .empty {
    color: var(--color-text-muted);
  }
  .table-wrap {
    overflow-x: auto;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--text-compact);
  }
  th {
    text-align: left;
    background: var(--color-canvas-subtle);
    padding: 0.85rem 1rem;
    font-weight: 600;
    border-bottom: 1px solid var(--color-border);
  }
  td {
    padding: 0.9rem 1rem;
    border-bottom: 1px solid var(--color-border);
    vertical-align: top;
  }
  tr:last-child td {
    border-bottom: 0;
  }
  tr:hover td {
    background: var(--color-surface-raised);
  }
  .muted {
    color: var(--color-text-muted);
  }
  .row-form {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 0.4rem;
  }
  .row-form input {
    min-height: 2.4rem;
    padding: 0.4rem 0.55rem;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-control);
    font: inherit;
  }
  .inline-panel {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-panel);
    padding: var(--space-5);
    margin-bottom: var(--space-5);
  }
  .inline-panel h3 {
    font-size: var(--text-panel);
    margin-bottom: var(--space-4);
  }
  .inline-fields {
    display: grid;
    grid-template-columns: 1fr 1fr auto;
    gap: var(--space-3);
    align-items: end;
  }
  code {
    font-size: 0.8rem;
  }

  @media (max-width: 900px) {
    .admin-main {
      padding-top: var(--space-6);
    }
    .attention,
    .money-strip,
    .billing-forms,
    .inline-fields {
      grid-template-columns: 1fr 1fr;
    }
    .panel {
      scroll-margin-top: 4.5rem;
    }
  }
  @media (max-width: 700px) {
    .attention,
    .money-strip,
    .billing-forms,
    .inline-fields {
      grid-template-columns: 1fr;
    }
    thead {
      display: none;
    }
    tr {
      display: block;
      padding: var(--space-4);
      border-bottom: 1px solid var(--color-border);
    }
    td {
      display: grid;
      grid-template-columns: minmax(5.5rem, 30%) 1fr;
      gap: var(--space-2);
      border: 0;
      padding: 0.45rem 0;
    }
    td::before {
      content: attr(data-label);
      color: var(--color-text-muted);
      font-weight: 500;
    }
    td[data-label='Action'],
    td[data-label='Assign'] {
      grid-template-columns: 1fr;
    }
    td[data-label='Action']::before,
    td[data-label='Assign']::before {
      margin-bottom: var(--space-2);
    }
    .row-form {
      flex-direction: column;
    }
    .row-form input,
    .row-form :global(.btn) {
      width: 100%;
      min-height: 2.75rem;
    }
    .inline-fields :global(.btn) {
      width: 100%;
    }
  }
</style>
