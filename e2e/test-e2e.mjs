import { chromium } from 'playwright';

const BASE = 'http://localhost';

async function run() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  // 1. Visit homepage
  console.log('=== 1. Homepage ===');
  await page.goto(BASE);
  await page.waitForLoadState('networkidle');
  const title = await page.title();
  console.log(`Title: ${title}`);
  const bodyText = await page.textContent('body');
  console.log(`Body preview: ${bodyText.slice(0, 200)}`);

  // 2. Check navbar
  console.log('\n=== 2. Navbar ===');
  const navLinks = await page.$$eval('nav a', els => els.map(e => ({ text: e.textContent, href: e.getAttribute('href') })));
  console.log('Nav links:', JSON.stringify(navLinks));

  // 3. Register a user
  console.log('\n=== 3. Register ===');
  await page.goto(`${BASE}/register`);
  await page.waitForLoadState('networkidle');
  await page.fill('input[type="email"], input[name="email"]', 'playwright@test.com');
  await page.fill('input[type="text"], input[name="username"]', 'playwright');
  await page.fill('input[type="password"], input[name="password"]', 'pass123');
  await page.click('button[type="submit"]');
  await page.waitForTimeout(2000);
  const afterRegister = page.url();
  console.log(`After register URL: ${afterRegister}`);
  const navAfterRegister = await page.$$eval('nav a', els => els.map(e => e.textContent));
  console.log(`Nav after register: ${navAfterRegister}`);

  // 4. Check if Users link appears
  console.log('\n=== 4. Users link ===');
  const usersLink = await page.$('a[href="/users"]');
  console.log(`Users link exists: ${!!usersLink}`);

  // 5. Go to Users page
  if (usersLink) {
    await usersLink.click();
    await page.waitForTimeout(2000);
    console.log(`Users page URL: ${page.url()}`);
    const usersText = await page.textContent('body');
    console.log(`Users page: ${usersText.slice(0, 300)}`);
  }

  // 6. Check follow flow
  console.log('\n=== 5. Follow flow ===');
  // Find first user link
  const userLinks = await page.$$('a[href^="/users/"]');
  console.log(`User links found: ${userLinks.length}`);
  if (userLinks.length > 0) {
    await userLinks[0].click();
    await page.waitForTimeout(2000);
    console.log(`Profile URL: ${page.url()}`);
    const followBtn = await page.$('button');
    if (followBtn) {
      const btnText = await followBtn.textContent();
      console.log(`Button text: ${btnText}`);
    }
  }

  // 7. Check notifications page
  console.log('\n=== 6. Notifications ===');
  await page.goto(`${BASE}/notifications`);
  await page.waitForTimeout(2000);
  const notifText = await page.textContent('body');
  console.log(`Notifications page: ${notifText.slice(0, 300)}`);

  // 8. Check feed
  console.log('\n=== 7. Feed ===');
  await page.goto(`${BASE}/feed`);
  await page.waitForTimeout(2000);
  const feedText = await page.textContent('body');
  console.log(`Feed page: ${feedText.slice(0, 300)}`);

  // Take screenshot
  await page.goto(`${BASE}`);
  await page.waitForTimeout(2000);
  await page.screenshot({ path: '/tmp/lievre-home.png', fullPage: true });
  console.log('\nScreenshot saved to /tmp/lievre-home.png');

  await browser.close();
}

run().catch(e => { console.error(e); process.exit(1); });
