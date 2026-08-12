import { chromium } from 'playwright';

const BASE = 'http://localhost';

async function register(page, email, username, password) {
  await page.goto(`${BASE}/register`);
  await page.waitForLoadState('networkidle');
  await page.fill('input[placeholder="Email"]', email);
  await page.fill('input[placeholder="Username"]', username);
  await page.fill('input[placeholder="Password"]', password);
  await page.click('button[type="submit"]');
  await page.waitForURL('**/', { timeout: 10000 });
}

async function createActivity(page, { title, type = 'ride', visibility = 'public', distance, duration }) {
  await page.goto(`${BASE}/activities/new`);
  await page.waitForLoadState('networkidle');
  await page.waitForSelector('select[name="activity_type"]');

  await page.selectOption('select[name="activity_type"]', type);
  await page.fill('input[name="title"]', title);
  await page.locator('input[name="started_at"]').fill('2026-08-12T09:00');
  if (distance) await page.fill('input[name="distance_meters"]', String(distance));
  if (duration) await page.fill('input[name="duration_seconds"]', String(duration));
  await page.selectOption('select[name="visibility"]', visibility);

  const formValues = await page.evaluate(() => {
    const form = document.querySelector('form');
    if (!form) return 'NO FORM FOUND';
    const data = new FormData(form);
    return Object.fromEntries(data.entries());
  });
  console.log(`  Form data: ${JSON.stringify(formValues)}`);

  // Submit and wait for redirect to activity detail (NOT /activities/new)
  await Promise.all([
    page.waitForURL(url => {
      const u = url.toString();
      return u.includes('/activities/') && !u.includes('/activities/new');
    }, { timeout: 15000 }),
    page.click('button[type="submit"]'),
  ]);

  const errorEl = await page.$('[style*="color: red"], [style*="color:red"]');
  if (errorEl) {
    const errorText = await errorEl.textContent();
    console.log(`  ❌ Error: ${errorText}`);
  }
}

async function run() {
  const browser = await chromium.launch({ headless: true });

  // ── User A: athlete ──────────────────────────────────────────
  console.log('\n━━━ User A (athlete) ━━━');
  const ctxA = await browser.newContext();
  const pageA = await ctxA.newPage();

  pageA.on('console', msg => {
    if (msg.type() === 'error') console.log(`  [A console.error] ${msg.text()}`);
  });
  pageA.on('pageerror', err => console.log(`  [A pageerror] ${err.message}`));

  await register(pageA, 'athlete@test.com', 'athlete', 'pass123');
  console.log(`Registered → ${pageA.url()}`);

  await createActivity(pageA, { title: 'Morning Ride', distance: 42000, duration: 5400, visibility: 'public' });
  console.log(`After create → ${pageA.url()}`);

  // ── User B: follower ─────────────────────────────────────────
  console.log('\n━━━ User B (follower) ━━━');
  const ctxB = await browser.newContext();
  const pageB = await ctxB.newPage();

  pageB.on('console', msg => {
    if (msg.type() === 'error') console.log(`  [B console.error] ${msg.text()}`);
  });
  pageB.on('pageerror', err => console.log(`  [B pageerror] ${err.message}`));

  await register(pageB, 'follower@test.com', 'follower', 'pass123');
  console.log(`Registered → ${pageB.url()}`);

  // Check nav
  const navLinks = await pageB.$$eval('nav a', els => els.map(e => e.textContent));
  console.log(`Nav: ${navLinks.join(' | ')}`);

  // ── Users page ───────────────────────────────────────────────
  console.log('\n━━━ Users page ━━━');
  await pageB.click('a[href="/users"]');
  await pageB.waitForLoadState('networkidle');
  await pageB.waitForTimeout(1000);

  const userLinks = await pageB.$$eval('a[href^="/users/"]', els => els.map(e => ({
    text: e.textContent,
    href: e.getAttribute('href'),
  })));
  console.log(`User links: ${JSON.stringify(userLinks)}`);

  // ── Profile + Follow ─────────────────────────────────────────
  console.log('\n━━━ Profile + Follow ━━━');
  const athleteLink = await pageB.$('a[href^="/users/"]:has-text("athlete")');
  if (!athleteLink) {
    console.log('No athlete link found. Trying first user link...');
    const firstLink = await pageB.$('a[href^="/users/"]');
    if (firstLink) await firstLink.click();
  } else {
    await athleteLink.click();
  }
  await pageB.waitForURL('**/users/**');
  await pageB.waitForLoadState('networkidle');
  await pageB.waitForTimeout(2000);
  console.log(`Profile URL: ${pageB.url()}`);

  // Check for activities — wait for at least one activity link to load
  await pageB.waitForSelector('a[href^="/activities/"]', { timeout: 5000 }).catch(() => {});
  const profileActs = await pageB.$$eval('a[href^="/activities/"]', els =>
    els.filter(e => e.textContent !== '+ New').map(e => e.textContent)
  ).catch(() => []);
  console.log(`Profile activities: ${profileActs.join(', ') || '(none)'}`);

  // Follow
  const followBtn = await pageB.$('button:has-text("Follow")');
  if (followBtn) {
    const btnText = await followBtn.textContent();
    if (btnText.includes('Unfollow')) {
      console.log('Already following');
    } else {
      await followBtn.click();
      await pageB.waitForTimeout(1000);
      const newBtn = await pageB.$eval('button:has-text("Unfollow")', e => e.textContent).catch(() => null);
      console.log(`Follow result: ${newBtn ? '✅ Now following' : '❌ Follow failed'}`);
    }
  } else {
    const allBtns = await pageB.$$eval('button', els => els.map(e => e.textContent));
    console.log(`No follow button. All buttons: ${allBtns.join(', ')}`);
  }

  // ── Feed ─────────────────────────────────────────────────────
  console.log('\n━━━ Feed ━━━');
  await pageB.goto(`${BASE}/feed`);
  await pageB.waitForLoadState('networkidle');
  await pageB.waitForTimeout(1000);

  const feedItems = await pageB.$$eval('a[href^="/activities/"]', els =>
    els.filter(e => e.textContent !== '+ New').map(e => e.textContent)
  );
  const feedAuthors = await pageB.$$eval('a[href^="/users/"] strong', els => els.map(e => e.textContent));

  console.log(`Feed activities: ${feedItems.join(', ') || '(empty)'}`);
  console.log(`Feed authors: ${feedAuthors.join(', ') || '(none)'}`);
  console.log(feedItems.length > 0 ? '✅ Feed has activities' : '❌ Feed is empty');

  // ── Activity detail ──────────────────────────────────────────
  console.log('\n━━━ Activity detail ━━━');
  const actLink = await pageB.$('a[href^="/activities/"]:not(:has-text("+ New"))');
  if (actLink) {
    await actLink.click();
    await pageB.waitForURL('**/activities/**');
    await pageB.waitForLoadState('networkidle');
    await pageB.waitForTimeout(2000);

    const hasMap = await pageB.$('.leaflet-container');
    console.log(`Map: ${hasMap ? '✅' : '❌ (no GPS route for manual activities)'}`);

    // Like
    const likeBtn = await pageB.$('button:has-text("Like"), button:has-text("♥")');
    if (likeBtn) {
      await likeBtn.click();
      await pageB.waitForTimeout(1000);
      console.log('✅ Liked');
    } else {
      console.log('❌ No like button');
    }

    // Comment
    const commentInput = await pageB.$('input[placeholder*="comment" i], textarea[placeholder*="comment" i]');
    if (commentInput) {
      await commentInput.fill('Nice ride!');
      await pageB.keyboard.press('Enter');
      await pageB.waitForTimeout(1000);
      const hasComment = await pageB.textContent('body').then(t => t.includes('Nice ride'));
      console.log(`Comment: ${hasComment ? '✅' : '❌'}`);
    } else {
      console.log('❌ No comment input');
    }
  }

  // ── Notifications (athlete) ──────────────────────────────────
  console.log('\n━━━ Notifications (athlete) ━━━');
  await pageA.goto(`${BASE}/notifications`);
  await pageA.waitForLoadState('networkidle');
  await pageA.waitForTimeout(1000);

  const bodyText = await pageA.textContent('body');
  console.log(`Has follow notif: ${bodyText.includes('follow') ? '✅' : '❌'}`);
  console.log(`Has like notif: ${bodyText.includes('like') ? '✅' : '❌'}`);
  console.log(`Has comment notif: ${bodyText.includes('Nice ride') ? '✅' : '❌'}`);

  // ── Screenshots ──────────────────────────────────────────────
  await pageB.goto(`${BASE}/feed`);
  await pageB.waitForLoadState('networkidle');
  await pageB.waitForTimeout(500);
  await pageB.screenshot({ path: '/tmp/lievre-feed.png', fullPage: true });

  await pageA.goto(`${BASE}/notifications`);
  await pageA.waitForLoadState('networkidle');
  await pageA.waitForTimeout(500);
  await pageA.screenshot({ path: '/tmp/lievre-notifications.png', fullPage: true });

  console.log('\n📸 /tmp/lievre-feed.png, /tmp/lievre-notifications.png');
  await browser.close();
  console.log('✅ Done');
}

run().catch(e => { console.error('❌', e.message); process.exit(1); });
