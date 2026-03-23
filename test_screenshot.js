const { chromium } = require('playwright');
(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto('http://127.0.0.1:9531', { waitUntil: 'networkidle' });
  await page.screenshot({ path: 'screenshot.png' });
  await browser.close();
  console.log('Screenshot saved');
})();